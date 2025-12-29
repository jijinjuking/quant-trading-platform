//! # 订单模型 (Order Model)
//! 
//! 定义交易订单的领域实体和相关枚举类型。
//! 
//! ## 包含类型
//! - `Order`: 订单实体（聚合根）
//! - `OrderSide`: 订单方向（买/卖）
//! - `OrderType`: 订单类型（市价/限价等）
//! - `OrderStatus`: 订单状态

// ============================================================================
// 外部依赖导入
// ============================================================================

use chrono::{DateTime, Utc};  // 时间处理库 - 用于订单创建时间
use rust_decimal::Decimal;     // 高精度十进制 - 用于金额和数量计算
use serde::{Deserialize, Serialize};  // 序列化/反序列化 - 用于 JSON 转换
use uuid::Uuid;                // UUID 生成 - 用于唯一标识符

// ============================================================================
// 订单实体 (Order Entity) - 聚合根
// ============================================================================

/// 订单实体 - 交易引擎的核心领域模型
/// 
/// 这是一个聚合根，代表用户提交的交易订单。
/// 包含订单的所有必要信息，用于交易执行和状态跟踪。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// 订单唯一标识符
    pub id: Uuid,
    
    /// 用户唯一标识符 - 订单所属用户
    pub user_id: Uuid,
    
    /// 交易所名称 - 如 "binance", "okx"
    pub exchange: String,
    
    /// 交易对符号 - 如 "BTC/USDT"
    pub symbol: String,
    
    /// 订单方向 - 买入或卖出
    pub side: OrderSide,
    
    /// 订单类型 - 市价、限价等
    pub order_type: OrderType,
    
    /// 订单数量 - 要交易的数量
    pub quantity: Decimal,
    
    /// 订单价格 - 限价单必填，市价单为 None
    pub price: Option<Decimal>,
    
    /// 订单状态 - 当前订单的生命周期状态
    pub status: OrderStatus,
    
    /// 已成交数量 - 部分成交时记录
    pub filled_quantity: Decimal,
    
    /// 创建时间 - 订单提交时间（UTC）
    pub created_at: DateTime<Utc>,
}

// ============================================================================
// 订单方向枚举 (Order Side)
// ============================================================================

/// 订单方向 - 定义交易的买卖方向
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderSide {
    /// 买入 - 做多
    Buy,
    /// 卖出 - 做空或平仓
    Sell,
}

// ============================================================================
// 订单类型枚举 (Order Type)
// ============================================================================

/// 订单类型 - 定义订单的执行方式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderType {
    /// 市价单 - 以当前市场价格立即成交
    Market,
    /// 限价单 - 以指定价格或更优价格成交
    Limit,
    /// 止损单 - 价格触及止损点时触发
    StopLoss,
    /// 止盈单 - 价格触及止盈点时触发
    TakeProfit,
}

// ============================================================================
// 订单状态枚举 (Order Status)
// ============================================================================

/// 订单状态 - 定义订单的生命周期状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OrderStatus {
    /// 待处理 - 订单已创建，等待提交
    Pending,
    /// 已提交 - 订单已提交到交易所
    Submitted,
    /// 部分成交 - 订单部分数量已成交
    PartialFilled,
    /// 完全成交 - 订单全部数量已成交
    Filled,
    /// 已取消 - 订单被用户取消
    Cancelled,
    /// 已拒绝 - 订单被交易所拒绝
    Rejected,
    /// 失败 - 订单执行失败
    Failed,
}
