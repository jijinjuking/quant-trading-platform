//! # ClickHouse Integration - ClickHouse集成模块
//!
//! ## 模块职责
//! 提供 ClickHouse 时序数据库的集成能力：
//! - 连接管理
//! - 查询执行
//! - 数据转换
//!
//! ## ClickHouse 简介
//! ClickHouse 是一个高性能的列式数据库，适用于：
//! - 时序数据存储（行情、交易记录）
//! - 实时分析查询
//! - 大数据量聚合统计
//!
//! ## 子模块
//! - `client`: ClickHouse 客户端实现

/// ClickHouse 客户端
pub mod client;
