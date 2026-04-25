//! # 执行端口 (Execution Port) - v2 完整实现
//!
//! 定义执行指令的抽象入口。
//!
//! ## 架构位置
//! Domain Layer > Port
//!
//! ## 版本说明
//! v2 支持多种订单类型：市价单、限价单、止损单、止盈单。
//!
//! ## 职责
//! - 接收执行指令
//! - 作为"执行入口"的抽象通道
//!
//! ## 规则
//! - 只定义 trait，不包含实现
//! - 不允许引入任何 infrastructure 依赖
//! - 不允许包含业务判断逻辑

use std::sync::Arc;
use async_trait::async_trait;
use rust_decimal::Decimal;

/// 订单类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderType {
    /// 市价单 - 立即以市场最优价格成交
    Market,
    /// 限价单 - 指定价格，只有达到该价格才成交
    Limit,
    /// 止损限价单 - 当价格达到止损价时，以限价单方式下单
    StopLossLimit,
    /// 止盈限价单 - 当价格达到止盈价时，以限价单方式下单
    TakeProfitLimit,
    /// 止损市价单 - 当价格达到止损价时，以市价单方式下单
    StopLossMarket,
    /// 止盈市价单 - 当价格达到止盈价时，以市价单方式下单
    TakeProfitMarket,
}

impl OrderType {
    /// 转换为币安 API 订单类型字符串
    pub fn to_binance_str(&self) -> &'static str {
        match self {
            OrderType::Market => "MARKET",
            OrderType::Limit => "LIMIT",
            OrderType::StopLossLimit => "STOP_LOSS_LIMIT",
            OrderType::TakeProfitLimit => "TAKE_PROFIT_LIMIT",
            OrderType::StopLossMarket => "STOP_LOSS",
            OrderType::TakeProfitMarket => "TAKE_PROFIT",
        }
    }
}

/// 订单方向
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrderSide {
    /// 买入
    Buy,
    /// 卖出
    Sell,
}

impl OrderSide {
    /// 从字符串解析
    pub fn from_str(s: &str) -> Option<Self> {
        match s.trim().to_lowercase().as_str() {
            "buy" => Some(OrderSide::Buy),
            "sell" => Some(OrderSide::Sell),
            _ => None,
        }
    }

    /// 转换为币安 API 字符串
    pub fn to_binance_str(&self) -> &'static str {
        match self {
            OrderSide::Buy => "BUY",
            OrderSide::Sell => "SELL",
        }
    }
}

/// 时间有效性类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeInForce {
    /// Good Till Cancel - 订单一直有效直到成交或取消
    GTC,
    /// Immediate Or Cancel - 立即成交，未成交部分自动取消
    IOC,
    /// Fill Or Kill - 全部成交或全部取消
    FOK,
}

impl TimeInForce {
    /// 转换为币安 API 字符串
    pub fn to_binance_str(&self) -> &'static str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
        }
    }
}

/// 执行指令 - v2 完整结构
///
/// # 设计说明
/// 支持多种订单类型和参数配置。
#[derive(Debug, Clone)]
pub struct ExecutionCommand {
    /// 交易对符号（如 BTCUSDT）
    pub symbol: String,
    /// 订单方向
    pub side: OrderSide,
    /// 订单类型
    pub order_type: OrderType,
    /// 数量
    pub quantity: Decimal,
    /// 限价单价格（仅限价单需要）
    pub price: Option<Decimal>,
    /// 止损/止盈触发价格（仅止损/止盈单需要）
    pub stop_price: Option<Decimal>,
    /// 时间有效性（仅限价单需要，默认 GTC）
    pub time_in_force: Option<TimeInForce>,
    /// 客户端订单 ID（可选，用于幂等）
    pub client_order_id: Option<String>,
}

impl ExecutionCommand {
    /// 创建市价单
    pub fn market(symbol: String, side: OrderSide, quantity: Decimal) -> Self {
        Self {
            symbol,
            side,
            order_type: OrderType::Market,
            quantity,
            price: None,
            stop_price: None,
            time_in_force: None,
            client_order_id: None,
        }
    }

    /// 创建限价单
    pub fn limit(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Decimal,
    ) -> Self {
        Self {
            symbol,
            side,
            order_type: OrderType::Limit,
            quantity,
            price: Some(price),
            stop_price: None,
            time_in_force: Some(TimeInForce::GTC),
            client_order_id: None,
        }
    }

    /// 创建止损限价单
    pub fn stop_loss_limit(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Decimal,
        stop_price: Decimal,
    ) -> Self {
        Self {
            symbol,
            side,
            order_type: OrderType::StopLossLimit,
            quantity,
            price: Some(price),
            stop_price: Some(stop_price),
            time_in_force: Some(TimeInForce::GTC),
            client_order_id: None,
        }
    }

    /// 创建止盈限价单
    pub fn take_profit_limit(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        price: Decimal,
        stop_price: Decimal,
    ) -> Self {
        Self {
            symbol,
            side,
            order_type: OrderType::TakeProfitLimit,
            quantity,
            price: Some(price),
            stop_price: Some(stop_price),
            time_in_force: Some(TimeInForce::GTC),
            client_order_id: None,
        }
    }

    /// 创建止损市价单
    pub fn stop_loss_market(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        stop_price: Decimal,
    ) -> Self {
        Self {
            symbol,
            side,
            order_type: OrderType::StopLossMarket,
            quantity,
            price: None,
            stop_price: Some(stop_price),
            time_in_force: None,
            client_order_id: None,
        }
    }

    /// 创建止盈市价单
    pub fn take_profit_market(
        symbol: String,
        side: OrderSide,
        quantity: Decimal,
        stop_price: Decimal,
    ) -> Self {
        Self {
            symbol,
            side,
            order_type: OrderType::TakeProfitMarket,
            quantity,
            price: None,
            stop_price: Some(stop_price),
            time_in_force: None,
            client_order_id: None,
        }
    }

    /// 设置客户端订单 ID
    pub fn with_client_order_id(mut self, client_order_id: String) -> Self {
        self.client_order_id = Some(client_order_id);
        self
    }

    /// 设置时间有效性
    pub fn with_time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = Some(time_in_force);
        self
    }
}

/// 执行结果
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// 订单 ID（交易所返回）
    pub order_id: String,
    /// 客户端订单 ID
    pub client_order_id: Option<String>,
    /// 交易对符号
    pub symbol: String,
    /// 订单状态
    pub status: String,
    /// 已成交数量
    pub executed_qty: Decimal,
    /// 平均成交价格
    pub avg_price: Option<Decimal>,
}

/// 执行端口 - v2 完整实现
///
/// 定义执行指令的抽象接口。
/// Infrastructure 层的具体执行实现必须实现此 trait。
///
/// # 设计说明
/// v2 支持多种订单类型，返回详细的执行结果。
#[async_trait]
pub trait ExecutionPort: Send + Sync {
    /// 执行指令
    ///
    /// # 参数
    /// - `command`: 执行指令（只读引用）
    ///
    /// # 返回
    /// - `Ok(ExecutionResult)`: 执行成功，返回订单信息
    /// - `Err`: 执行失败
    async fn execute(&self, command: &ExecutionCommand) -> anyhow::Result<ExecutionResult>;
}

// Arc<T> 自动实现 ExecutionPort
#[async_trait]
impl<T: ?Sized + ExecutionPort> ExecutionPort for Arc<T> {
    async fn execute(&self, command: &ExecutionCommand) -> anyhow::Result<ExecutionResult> {
        (**self).execute(command).await
    }
}
