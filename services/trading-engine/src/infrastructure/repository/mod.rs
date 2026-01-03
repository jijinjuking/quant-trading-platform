//! # 数据仓储 (Repositories)
//!
//! 提供数据持久化的具体实现。

pub mod postgres_order_repository;

pub use postgres_order_repository::PostgresOrderRepository;
