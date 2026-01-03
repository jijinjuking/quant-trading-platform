//! # 审计基础设施适配器
//!
//! 路径: services/trading-engine/src/infrastructure/audit/mod.rs

pub mod noop_audit;

pub use noop_audit::NoopAuditAdapter;
