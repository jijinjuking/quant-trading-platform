//! # 合约策略模块 (Futures Strategies)
//!
//! 合约市场专用策略实现。
//! 支持杠杆、双向持仓、资金费率等合约特性。

pub mod grid;
pub mod mean;
pub mod funding_arb;

pub use grid::FuturesGridStrategy;
pub use mean::FuturesMeanReversionStrategy;
pub use funding_arb::FundingArbStrategy;
