//! # 应用层 (Application Layer)
//! 
//! 用例编排层，协调领域对象完成业务流程。
//! 
//! ## 子模块
//! - `service`: 应用服务（用例实现）
//! 
//! ## 规则
//! - 只依赖 domain::port 中的 trait
//! - 不直接调用 infrastructure

/// 应用服务模块 - 用例编排
pub mod service;
