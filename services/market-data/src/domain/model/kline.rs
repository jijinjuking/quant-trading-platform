//! # K 线模型 (Kline Model)
//! 
//! 定义 K 线（蜡烛图）数据的领域实体。
//! 
//! ## 说明
//! K 线是将一段时间内的行情数据聚合后的表示形式，
//! 包含开盘价、最高价、最低价、收盘价和成交量。

// ============================================================================
// 外部依赖导入
// ============================================================================

use chrono::{DateTime, Utc};  // 时间处理 - K 线时间范围
use rust_decimal::Decimal;     // 高精度十进制 - 价格和数量
use serde::{Deserialize, Serialize};  // 序列化/反序列化

// ============================================================================
// Kline 实体
// ============================================================================

/// K 线实体 - 蜡烛图数据
/// 
/// 聚合一段时间内的行情数据，常用于技术分析。
/// 
/// ## 时间周期
/// 常见周期：1m, 5m, 15m, 1h, 4h, 1d, 1w
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kline {
    /// 交易所名称 - 如 "binance", "okx"
    pub exchange: String,
    
    /// 交易对符号 - 如 "BTC/USDT"
    pub symbol: String,
    
    /// 时间周期 - 如 "1m", "1h", "1d"
    pub interval: String,
    
    /// 开盘时间（UTC）- K 线开始时间
    pub open_time: DateTime<Utc>,
    
    /// 收盘时间（UTC）- K 线结束时间
    pub close_time: DateTime<Utc>,
    
    /// 开盘价 - 周期内第一笔成交价
    pub open: Decimal,
    
    /// 最高价 - 周期内最高成交价
    pub high: Decimal,
    
    /// 最低价 - 周期内最低成交价
    pub low: Decimal,
    
    /// 收盘价 - 周期内最后一笔成交价
    pub close: Decimal,
    
    /// 成交量 - 周期内总成交量
    pub volume: Decimal,
}
