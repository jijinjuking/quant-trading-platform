//! # 交易所连接器 (Exchange Connectors)
//! 
//! 提供与各交易所 API 交互的具体实现。
//! 
//! ## 子模块
//! - `binance`: 币安交易所连接器
//! 
//! ## 扩展说明
//! 添加新交易所时，创建新模块并实现 ExchangePort trait

/// 币安交易所连接器 - 实现 ExchangePort
pub mod binance;
