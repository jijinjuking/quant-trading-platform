//! # 应用服务模块 (Application Services)
//!
//! 本模块包含风险管理服务的所有应用服务（用例）。
//!
//! ## 包含的服务
//! - [`risk_check_service`]: 风险检查服务，负责订单前的风险校验

/// 风险检查服务 - 订单前风险校验用例
pub mod risk_check_service;
