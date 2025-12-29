//! # 仓储实现模块
//!
//! 本模块包含所有仓储的具体实现。
//!
//! ## 所属层
//! Infrastructure Layer > Repository
//!
//! ## 职责
//! - 实现 `domain::port` 中定义的仓储 trait
//! - 处理数据库操作
//! - 实现 DTO 与领域对象的转换

/// 用户仓储实现
pub mod user_repository;
