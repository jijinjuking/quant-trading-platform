//! # 数据仓储 (Repositories)
//! 
//! 提供数据持久化的具体实现。
//! 
//! ## 子模块
//! - `order_repository`: 订单仓储（实现 OrderRepositoryPort）
//! 
//! ## Hexagonal 架构角色
//! Repository 是「出站适配器」，负责：
//! - 实现 Domain 层定义的 Repository Port
//! - 处理 Domain ↔ DB 的数据转换

/// 订单仓储 - 实现 OrderRepositoryPort
pub mod order_repository;
