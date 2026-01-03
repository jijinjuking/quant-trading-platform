//! # HTTP 请求处理器模块
//!
//! 本模块包含所有 HTTP 请求处理器。
//!
//! ## 职责
//! - 接收和解析 HTTP 请求
//! - 代理转发到后端服务
//! - 构造 HTTP 响应
//!
//! ## 子模块
//! - `health`: 健康检查处理器
//! - `proxy`: 通用代理转发处理器
//! - `strategies`: 策略管理处理器
//! - `orders`: 订单管理处理器
//! - `positions`: 持仓管理处理器
//! - `services`: 服务状态处理器

/// 健康检查处理器
pub mod health;

/// 通用代理转发处理器
pub mod proxy;

/// 策略管理处理器
pub mod strategies;

/// 订单管理处理器
pub mod orders;

/// 持仓管理处理器
pub mod positions;

/// 服务状态处理器
pub mod services;
