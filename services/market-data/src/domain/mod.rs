//! # 领域层 (Domain Layer)
//!
//! market-data 服务的领域层，只包含端口定义。
//!
//! ## 说明
//! market-data 是行情采集器，不定义领域模型。
//! 行情事件类型定义在 shared::event::market_event 中。

pub mod port;
