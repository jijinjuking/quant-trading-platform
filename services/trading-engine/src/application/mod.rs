//! # 应用层模块
//! 
//! ## 功能层级: 【应用层 Application】
//! ## 职责:
//! - 用例编排（协调多个领域服务）
//! - 事务管理
//! - 只依赖domain::port中的trait
//! 
//! ## 依赖规则:
//! - 可以依赖: domain层（model + port trait）
//! - 不可依赖: infrastructure层的具体实现
//! - 不可依赖: interface层

/// 应用服务模块 - 包含所有用例编排服务
pub mod service;
