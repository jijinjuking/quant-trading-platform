//! # API Gateway 服务库
//!
//! 本模块是 API Gateway 服务的库入口，对外暴露所有公共模块。
//!
//! ## 服务职责
//! - **请求路由**: 将外部请求转发到对应的后端服务
//! - **认证鉴权**: JWT Token 验证和权限检查
//! - **限流保护**: 基于用户/IP 的请求频率限制
//! - **负载均衡**: 多实例服务的请求分发
//!
//! ## 模块结构
//! - `state`: 应用状态管理
//! - `interface`: 接口层（HTTP/gRPC）
//! - `application`: 应用层（用例编排）
//! - `domain`: 领域层（核心业务逻辑）
//! - `infrastructure`: 基础设施层（外部依赖实现）
//!
//! ## 架构遵循
//! 本服务遵循 DDD + 六边形架构（端口-适配器模式）

/// 应用状态模块
pub mod state;

/// 接口层 - HTTP/gRPC 入口
pub mod interface;

/// 应用层 - 用例编排
pub mod application;

/// 领域层 - 核心业务逻辑
pub mod domain;

/// 基础设施层 - 外部依赖实现
pub mod infrastructure;
