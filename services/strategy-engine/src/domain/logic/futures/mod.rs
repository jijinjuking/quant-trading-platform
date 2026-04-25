//! # 合约策略模块 (Futures Strategies)
//!
//! 合约市场专用策略实现。
//! 支持杠杆、双向持仓、资金费率等合约特性。

pub mod grid;
pub mod mean;
pub mod funding_arb;
pub mod macd;
pub mod bollinger;
pub mod rsi;
pub mod trend_following;
pub mod breakout;
pub mod reversal;
pub mod calendar_spread;

pub use grid::FuturesGridStrategy;
pub use mean::FuturesMeanReversionStrategy;
pub use funding_arb::FundingArbStrategy;
pub use macd::FuturesMacdStrategy;
pub use bollinger::FuturesBollingerStrategy;
pub use rsi::FuturesRsiStrategy;
pub use trend_following::TrendFollowingStrategy;
pub use breakout::BreakoutStrategy;
pub use reversal::ReversalStrategy;
pub use calendar_spread::CalendarSpreadStrategy;
