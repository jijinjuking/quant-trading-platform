//! # 端口定义模块 (Ports)
//!
//! 本模块定义领域层的端口（抽象接口），遵循六边形架构的端口-适配器模式。
//!
//! ## 所属层
//! Domain Layer > Port
//!
//! ## 设计原则
//! - 端口只定义 trait（接口），不包含实现
//! - 入参和出参只能是领域对象或基础类型
//! - ❌ 禁止出现 HTTP/DB/Redis/Kafka 等外部类型
//!
//! ## 依赖方向
//! ```text
//! infrastructure → domain::port (实现 trait)
//! application → domain::port (依赖 trait)
//! ```

/// 用户仓储端口
pub mod user_repository_port;
