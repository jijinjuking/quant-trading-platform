//! # Domain Layer - 领域层
//!
//! ## 层级职责
//! 领域层是系统的核心，包含：
//! - 领域模型（实体、值对象、聚合）
//! - 领域服务（跨实体的业务规则）
//! - 端口定义（trait，供基础设施层实现）
//!
//! ## 架构约束（最重要）
//! - ✅ 纯业务逻辑，不依赖任何外部框架
//! - ✅ 只使用标准库和共享内核类型
//! - ❌ 禁止依赖 HTTP/DB/Redis/Kafka 等外部类型
//! - ❌ 禁止依赖 interface/application/infrastructure 层
//!
//! ## 子模块
//! - `model`: 领域模型（绩效指标等）
//! - `port`: 端口定义（仓储trait等）
//!
//! ## 依赖方向
//! ```text
//! interface → application → domain ← infrastructure
//!                            ↑
//!                     核心层不依赖任何外层
//! ```

/// 领域模型模块
pub mod model;

/// 端口定义模块（trait）
pub mod port;
