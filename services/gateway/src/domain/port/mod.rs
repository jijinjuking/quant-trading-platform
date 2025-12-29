//! # 端口定义模块
//!
//! 本模块包含 API Gateway 领域层的端口（Port）定义。
//!
//! ## 六边形架构
//! 端口是六边形架构的核心概念，定义了领域层与外部世界的边界。
//!
//! ```text
//!                    ┌─────────────────┐
//!     Adapter ──────►│      Port       │◄────── Adapter
//!   (Infrastructure) │    (trait)      │    (Infrastructure)
//!                    │                 │
//!                    │  Domain Layer   │
//!                    └─────────────────┘
//! ```
//!
//! ## 规则（强制）
//! - 端口只能是 trait
//! - 入参/出参只能是领域对象或基础类型
//! - ❌ 禁止 HTTP/DB/Redis/Kafka 类型
//!
//! ## 子模块
//! - `auth_port`: 认证端口
//! - `cache_port`: 缓存端口

/// 认证端口
pub mod auth_port;

/// 缓存端口
pub mod cache_port;
