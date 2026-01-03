//! # 执行端口 (Execution Port) - v1 占位骨架
//!
//! 定义执行指令的抽象入口。
//!
//! ## 架构位置
//! Domain Layer > Port
//!
//! ## 版本说明
//! v1 仅作为结构性通道，不包含任何真实执行逻辑。
//! 验证 Execution Engine 在架构上"站得住"。
//!
//! ## 职责
//! - 接收执行指令
//! - 作为"执行入口"的抽象通道
//!
//! ## 规则
//! - 只定义 trait，不包含实现
//! - 不允许引入任何 infrastructure 依赖
//! - 不允许包含业务判断逻辑
//!
//! ## v1 限制
//! - 不区分市价/限价
//! - 不维护订单状态
//! - 不处理失败
//! - 不重试
//! - 不并发
//! - 不 backpressure

use std::sync::Arc;
use async_trait::async_trait;

/// 执行指令 - v1 最小占位结构
///
/// # 设计说明
/// v1 仅定义最小字段，不提前设计复杂结构。
/// 未来版本可扩展。
#[derive(Debug, Clone)]
pub struct ExecutionCommand {
    /// 交易对符号
    pub symbol: String,
    /// 方向（buy/sell）
    pub side: String,
    /// 数量
    pub quantity: String,
}

/// 执行端口 - v1 占位骨架
///
/// 定义执行指令的抽象接口。
/// Infrastructure 层的具体执行实现必须实现此 trait。
///
/// # 设计说明
/// v1 仅作为结构性通道，不包含任何执行逻辑。
/// 未来可替换为真实执行实现。
#[async_trait]
pub trait ExecutionPort: Send + Sync {
    /// 执行指令（v1 占位）
    ///
    /// # 参数
    /// - `command`: 执行指令（只读引用）
    ///
    /// # 返回
    /// - `Ok(())`: 执行成功（v1 始终成功）
    /// - `Err`: 执行失败
    ///
    /// # TODO
    /// v2 需要定义 ExecutionResult 类型，支持成交/拒绝/部分成交
    async fn execute(&self, command: &ExecutionCommand) -> anyhow::Result<()>;
}

// Arc<T> 自动实现 ExecutionPort
#[async_trait]
impl<T: ?Sized + ExecutionPort> ExecutionPort for Arc<T> {
    async fn execute(&self, command: &ExecutionCommand) -> anyhow::Result<()> {
        (**self).execute(command).await
    }
}
