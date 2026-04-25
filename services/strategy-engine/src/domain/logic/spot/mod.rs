//! # 现货策略模块 (Spot Strategies)
//!
//! 现货市场专用策略实现。

pub mod grid;
pub mod mean;
pub mod macd;
pub mod bollinger;
pub mod rsi;

pub use grid::SpotGridStrategy;
pub use mean::SpotMeanReversionStrategy;
pub use macd::SpotMacdStrategy;
pub use bollinger::SpotBollingerStrategy;
pub use rsi::SpotRsiStrategy;
