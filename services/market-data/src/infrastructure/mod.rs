//! # 基础设施层 (Infrastructure Layer)
//!
//! market-data 服务的基础设施层，提供端口的具体实现。
//!
//! ## 包含模块
//! - `exchange`: 交易所适配器
//! - `messaging`: 消息队列适配器
//! - `storage`: 数据存储适配器

pub mod exchange;
pub mod messaging;
pub mod storage;
