//! # 策略运行时 (Strategy Runtime)
//!
//! 策略实例的执行壳，保证：
//! - 单 Task 隔离
//! - 串行执行（不可并发）
//! - panic 捕获
//! - 显式 shutdown
//!
//! ## 职责边界（冻结）
//! - ❌ 不订阅行情
//! - ❌ 不访问 Trading Engine
//! - ❌ 不管理多个策略
//! - ❌ 不持有全局状态

use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinHandle;
use tracing::{error, info, warn};
use uuid::Uuid;

// 复用 shared 的类型
pub use shared::types::order::{OrderSide, OrderType};

// ============================================================================
// 执行请求/响应（Strategy Engine 内部使用）
// ============================================================================

/// 执行请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionRequest {
    /// 请求 ID
    pub request_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 当前价格
    pub price: Decimal,
    /// 数量
    pub quantity: Decimal,
    /// 时间戳
    pub timestamp: DateTime<Utc>,
    /// 是否买方主动
    pub is_buyer_maker: bool,
}

impl ExecutionRequest {
    /// 创建执行请求
    pub fn new(symbol: impl Into<String>, price: Decimal, quantity: Decimal) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            symbol: symbol.into(),
            price,
            quantity,
            timestamp: Utc::now(),
            is_buyer_maker: false,
        }
    }

    /// 从行情事件创建
    pub fn from_market_data(
        symbol: impl Into<String>,
        price: Decimal,
        quantity: Decimal,
        timestamp_ms: i64,
        is_buyer_maker: bool,
    ) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            symbol: symbol.into(),
            price,
            quantity,
            timestamp: DateTime::from_timestamp_millis(timestamp_ms)
                .unwrap_or_else(Utc::now),
            is_buyer_maker,
        }
    }
}

/// 执行结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// 请求 ID
    pub request_id: Uuid,
    /// 是否产生交易意图
    pub has_intent: bool,
    /// 交易意图
    pub intent: Option<TradeIntent>,
    /// 执行耗时（微秒）
    pub execution_time_us: u64,
    /// 错误信息
    pub error: Option<String>,
}

/// 交易意图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeIntent {
    /// 意图 ID
    pub id: Uuid,
    /// 策略实例 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向（使用 shared 的类型）
    pub side: OrderSide,
    /// 数量
    pub quantity: Decimal,
    /// 价格（限价单）
    pub price: Option<Decimal>,
    /// 订单类型（使用 shared 的类型）
    pub order_type: OrderType,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f64,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

impl TradeIntent {
    /// 创建市价买入意图
    pub fn market_buy(strategy_id: Uuid, symbol: impl Into<String>, quantity: Decimal, confidence: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            strategy_id,
            symbol: symbol.into(),
            side: OrderSide::Buy,
            quantity,
            price: None,
            order_type: OrderType::Market,
            confidence,
            created_at: Utc::now(),
        }
    }

    /// 创建市价卖出意图
    pub fn market_sell(strategy_id: Uuid, symbol: impl Into<String>, quantity: Decimal, confidence: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            strategy_id,
            symbol: symbol.into(),
            side: OrderSide::Sell,
            quantity,
            price: None,
            order_type: OrderType::Market,
            confidence,
            created_at: Utc::now(),
        }
    }

    /// 创建限价买入意图
    pub fn limit_buy(strategy_id: Uuid, symbol: impl Into<String>, quantity: Decimal, price: Decimal, confidence: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            strategy_id,
            symbol: symbol.into(),
            side: OrderSide::Buy,
            quantity,
            price: Some(price),
            order_type: OrderType::Limit,
            confidence,
            created_at: Utc::now(),
        }
    }

    /// 创建限价卖出意图
    pub fn limit_sell(strategy_id: Uuid, symbol: impl Into<String>, quantity: Decimal, price: Decimal, confidence: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            strategy_id,
            symbol: symbol.into(),
            side: OrderSide::Sell,
            quantity,
            price: Some(price),
            order_type: OrderType::Limit,
            confidence,
            created_at: Utc::now(),
        }
    }
}

// ============================================================================
// Runtime Command（内部通信）
// ============================================================================

/// Runtime 指令
pub enum RuntimeCommand {
    /// 执行策略计算
    Execute {
        context: ExecutionRequest,
        reply: oneshot::Sender<Result<ExecutionResult>>,
    },
    /// 关闭
    Shutdown,
}

// ============================================================================
// 策略执行器 Trait
// ============================================================================

/// 策略执行器
///
/// 由具体策略实现，Runtime 持有并调用。
pub trait StrategyExecutor: Send + 'static {
    /// 获取策略实例 ID
    fn instance_id(&self) -> Uuid;

    /// 执行策略计算（纯函数，不做 IO）
    fn execute(&mut self, request: &ExecutionRequest) -> Result<ExecutionResult>;

    /// 重置策略状态
    fn reset(&mut self) -> Result<()>;

    /// 获取状态快照（用于调试）
    fn state_snapshot(&self) -> Result<serde_json::Value>;
}

// ============================================================================
// Runtime Handle（外部持有）
// ============================================================================

/// Runtime 句柄
///
/// 外部持有此句柄，通过它向 Runtime Task 发送指令。
pub struct RuntimeHandle {
    instance_id: Uuid,
    command_tx: mpsc::Sender<RuntimeCommand>,
    task_handle: JoinHandle<RuntimeExitReason>,
}

impl RuntimeHandle {
    /// 获取实例 ID
    pub fn instance_id(&self) -> Uuid {
        self.instance_id
    }

    /// 发送执行指令，等待结果
    pub async fn execute(&self, request: ExecutionRequest) -> Result<ExecutionResult> {
        let (reply_tx, reply_rx) = oneshot::channel();

        self.command_tx
            .send(RuntimeCommand::Execute {
                context: request,
                reply: reply_tx,
            })
            .await
            .map_err(|_| anyhow!("Runtime 已关闭"))?;

        reply_rx.await.map_err(|_| anyhow!("响应通道已关闭"))?
    }

    /// 发送关闭指令
    pub async fn shutdown(&self) -> Result<()> {
        let _ = self.command_tx.send(RuntimeCommand::Shutdown).await;
        Ok(())
    }

    /// 等待 Task 退出
    pub async fn wait(self) -> Result<RuntimeExitReason> {
        self.task_handle
            .await
            .map_err(|e| anyhow!("Task join 失败: {}", e))
    }

    /// 检查 Task 是否已退出
    pub fn is_finished(&self) -> bool {
        self.task_handle.is_finished()
    }

    /// 强制终止
    pub fn abort(&self) {
        self.task_handle.abort();
    }
}

// ============================================================================
// Runtime 退出原因
// ============================================================================

/// Runtime 退出原因
#[derive(Debug, Clone)]
pub enum RuntimeExitReason {
    /// 正常关闭
    Shutdown,
    /// 通道关闭
    ChannelClosed,
    /// panic
    Panic(String),
}

// ============================================================================
// 启动 Runtime
// ============================================================================

/// 启动策略 Runtime
///
/// 创建独立 Task 托管策略实例。
pub fn spawn_runtime<E: StrategyExecutor>(
    executor: E,
    buffer_size: usize,
) -> RuntimeHandle {
    let instance_id = executor.instance_id();
    let (command_tx, command_rx) = mpsc::channel(buffer_size);

    let task_handle = tokio::spawn(runtime_task_loop(instance_id, executor, command_rx));

    info!(instance_id = %instance_id, "策略 Runtime 已启动");

    RuntimeHandle {
        instance_id,
        command_tx,
        task_handle,
    }
}

// ============================================================================
// Runtime Task 主循环
// ============================================================================

async fn runtime_task_loop<E: StrategyExecutor>(
    instance_id: Uuid,
    mut executor: E,
    mut command_rx: mpsc::Receiver<RuntimeCommand>,
) -> RuntimeExitReason {
    info!(instance_id = %instance_id, "Runtime Task 开始运行");

    loop {
        let command = match command_rx.recv().await {
            Some(cmd) => cmd,
            None => {
                warn!(instance_id = %instance_id, "指令通道已关闭");
                return RuntimeExitReason::ChannelClosed;
            }
        };

        match command {
            RuntimeCommand::Execute { context, reply } => {
                let request_id = context.request_id;
                let start = std::time::Instant::now();

                // catch_unwind 捕获 panic
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    executor.execute(&context)
                }));

                match result {
                    Ok(exec_result) => {
                        let output = match exec_result {
                            Ok(mut r) => {
                                r.execution_time_us = start.elapsed().as_micros() as u64;
                                Ok(r)
                            }
                            Err(e) => Err(e),
                        };
                        let _ = reply.send(output);
                    }
                    Err(panic_info) => {
                        let panic_msg = extract_panic_message(&panic_info);
                        error!(
                            instance_id = %instance_id,
                            request_id = %request_id,
                            panic = %panic_msg,
                            "策略执行 panic，Runtime 退出"
                        );
                        let _ = reply.send(Err(anyhow!("策略执行 panic: {}", panic_msg)));
                        return RuntimeExitReason::Panic(panic_msg);
                    }
                }
            }

            RuntimeCommand::Shutdown => {
                info!(instance_id = %instance_id, "收到 Shutdown 指令");
                return RuntimeExitReason::Shutdown;
            }
        }
    }
}

fn extract_panic_message(panic_info: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = panic_info.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = panic_info.downcast_ref::<String>() {
        s.clone()
    } else {
        "Unknown panic".to_string()
    }
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    struct TestExecutor {
        id: Uuid,
        should_panic: bool,
        should_fail: bool,
        call_count: u32,
    }

    impl TestExecutor {
        fn new() -> Self {
            Self {
                id: Uuid::new_v4(),
                should_panic: false,
                should_fail: false,
                call_count: 0,
            }
        }
    }

    impl StrategyExecutor for TestExecutor {
        fn instance_id(&self) -> Uuid {
            self.id
        }

        fn execute(&mut self, request: &ExecutionRequest) -> Result<ExecutionResult> {
            self.call_count += 1;

            if self.should_panic {
                panic!("测试 panic");
            }
            if self.should_fail {
                return Err(anyhow!("测试错误"));
            }

            Ok(ExecutionResult {
                request_id: request.request_id,
                has_intent: false,
                intent: None,
                execution_time_us: 0,
                error: None,
            })
        }

        fn reset(&mut self) -> Result<()> {
            self.call_count = 0;
            Ok(())
        }

        fn state_snapshot(&self) -> Result<serde_json::Value> {
            Ok(serde_json::json!({
                "call_count": self.call_count
            }))
        }
    }

    #[tokio::test]
    async fn test_spawn_and_execute() {
        let executor = TestExecutor::new();
        let handle = spawn_runtime(executor, 16);

        let request = ExecutionRequest::new("BTCUSDT", Decimal::new(50000, 0), Decimal::new(1, 3));
        let result = handle.execute(request).await;
        assert!(result.is_ok());

        handle.shutdown().await.unwrap();
        let reason = handle.wait().await.unwrap();
        assert!(matches!(reason, RuntimeExitReason::Shutdown));
    }

    #[tokio::test]
    async fn test_execute_error() {
        let mut executor = TestExecutor::new();
        executor.should_fail = true;
        let handle = spawn_runtime(executor, 16);

        let request = ExecutionRequest::new("BTCUSDT", Decimal::new(50000, 0), Decimal::new(1, 3));
        let result = handle.execute(request).await;
        assert!(result.is_err());

        assert!(!handle.is_finished());
        handle.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_panic_causes_exit() {
        let mut executor = TestExecutor::new();
        executor.should_panic = true;
        let handle = spawn_runtime(executor, 16);

        let request = ExecutionRequest::new("BTCUSDT", Decimal::new(50000, 0), Decimal::new(1, 3));
        let result = handle.execute(request).await;
        assert!(result.is_err());

        let reason = handle.wait().await.unwrap();
        assert!(matches!(reason, RuntimeExitReason::Panic(_)));
    }
}
