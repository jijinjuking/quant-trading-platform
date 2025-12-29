//! # 交易所数据端口 (Exchange Port)
//! 
//! 定义获取交易所行情数据的抽象接口。
//! 
//! ## Hexagonal 架构说明
//! - 这是一个「出站端口」(Outbound Port)
//! - Domain 层通过此 trait 获取行情数据
//! - Infrastructure 层提供具体实现（如 BinanceWebSocket）

// ============================================================================
// 领域模型导入
// ============================================================================

use crate::domain::model::tick::Tick;    // Tick 模型
use crate::domain::model::kline::Kline;  // Kline 模型

// ============================================================================
// 交易所数据端口 Trait 定义
// ============================================================================

/// 交易所数据端口 - Domain 层定义的抽象接口
/// 
/// 定义了获取行情数据所需的所有操作。
/// 
/// # 实现要求
/// - `Send + Sync`: 支持跨线程安全使用
/// - 所有方法只返回 Domain 对象，不暴露 SDK 类型
/// 
/// # 示例实现
/// ```ignore
/// impl MarketExchangePort for BinanceWebSocket {
///     fn get_latest_tick(&self, symbol: &str) -> Option<Tick> {
///         // WebSocket 数据 → Domain Tick
///     }
/// }
/// ```
pub trait MarketExchangePort: Send + Sync {
    /// 订阅 Tick 数据流
    /// 
    /// # 参数
    /// - `symbol`: 交易对符号（如 "BTCUSDT"）
    /// 
    /// # 返回
    /// - `true`: 订阅成功
    /// - `false`: 订阅失败
    fn subscribe_ticker(&self, symbol: &str) -> bool;
    
    /// 获取最新 Tick 数据
    /// 
    /// # 参数
    /// - `symbol`: 交易对符号
    /// 
    /// # 返回
    /// - `Some(Tick)`: 获取成功
    /// - `None`: 无数据或获取失败
    fn get_latest_tick(&self, symbol: &str) -> Option<Tick>;
    
    /// 获取 K 线数据
    /// 
    /// # 参数
    /// - `symbol`: 交易对符号
    /// - `interval`: 时间周期（如 "1m", "1h"）
    /// - `limit`: 返回数量限制
    /// 
    /// # 返回
    /// - K 线数据列表（可能为空）
    fn get_klines(&self, symbol: &str, interval: &str, limit: u32) -> Vec<Kline>;
}
