//! # 存储模块 (Storage Module)
//!
//! 行情数据持久化实现。

pub mod clickhouse_storage;

pub use clickhouse_storage::ClickHouseStorage;
