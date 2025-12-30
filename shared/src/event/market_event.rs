//! # 行情事件 (Market Event)
//!
//! 定义行情数据的标准化事件结构。

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 行情事件 - 标准化的行情数据
///
/// 由 market-data 服务产生，供其他服务消费。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketEvent {
    /// 事件类型
    pub event_type: MarketEventType,
    /// 交易所来源
    pub exchange: String,
    /// 交易对符号
    pub symbol: String,
    /// 事件时间戳
    pub timestamp: DateTime<Utc>,
    /// 事件数据
    pub data: MarketEventData,
}

/// 行情事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketEventType {
    /// Tick 数据
    Tick,
    /// 深度数据
    Depth,
    /// K线数据
    Kline,
    /// 成交数据
    Trade,
}

/// 行情事件数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarketEventData {
    /// Tick 数据
    Tick(TickData),
    /// 深度数据
    Depth(DepthData),
    /// K线数据
    Kline(KlineData),
    /// 成交数据
    Trade(TradeData),
}

/// Tick 数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TickData {
    /// 最新价格
    pub price: Decimal,
    /// 24小时成交量
    pub volume_24h: Decimal,
    /// 24小时涨跌幅
    pub change_24h: Decimal,
    /// 买一价
    pub bid: Decimal,
    /// 卖一价
    pub ask: Decimal,
}

/// 深度数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepthData {
    /// 买盘 (价格, 数量)
    pub bids: Vec<(Decimal, Decimal)>,
    /// 卖盘 (价格, 数量)
    pub asks: Vec<(Decimal, Decimal)>,
}

/// K线数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KlineData {
    /// 时间周期
    pub interval: String,
    /// 开盘价
    pub open: Decimal,
    /// 最高价
    pub high: Decimal,
    /// 最低价
    pub low: Decimal,
    /// 收盘价
    pub close: Decimal,
    /// 成交量
    pub volume: Decimal,
    /// 开盘时间
    pub open_time: DateTime<Utc>,
    /// 收盘时间
    pub close_time: DateTime<Utc>,
}

/// 成交数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeData {
    /// 成交ID
    pub trade_id: String,
    /// 成交价格
    pub price: Decimal,
    /// 成交数量
    pub quantity: Decimal,
    /// 买方是否为 maker
    pub is_buyer_maker: bool,
}
