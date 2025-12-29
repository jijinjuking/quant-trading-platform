//! # 领域层模块
//!
//! 本模块是 API Gateway 服务的领域层入口。
//!
//! ## 架构位置
//! ```text
//! interface → application → domain ← infrastructure
//!                           ^^^^^^
//!                           当前层（核心）
//! ```
//!
//! ## 职责
//! - 定义领域模型（Entity / Value Object / Aggregate）
//! - 定义端口（Port）接口
//! - 包含核心业务规则
//!
//! ## 规则（强制）
//! - ❌ 不依赖任何外部框架（axum、tokio 等）
//! - ❌ 不依赖基础设施层
//! - ✅ 只依赖标准库和 shared 模块
//! - ✅ 端口只能是 trait
//!
//! ## 子模块
//! - `model`: 领域模型
//! - `port`: 端口定义（trait）

/// 领域模型模块
pub mod model;

/// 端口定义模块
pub mod port;
