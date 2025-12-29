//! # 领域层 (Domain Layer)
//! 
//! 这是 DDD 架构的核心层，包含所有业务逻辑和领域模型。
//! 
//! ## 子模块说明
//! - `model`: 领域模型（实体、值对象、聚合根）
//! - `logic`: 领域逻辑（业务规则、算法）
//! - `port`: 端口定义（trait 接口，Hexagonal 架构核心）
//! 
//! ## 依赖规则
//! - Domain 层不依赖任何外部层
//! - 其他层通过 port 中的 trait 与 Domain 交互
//! - Infrastructure 层实现 port 中定义的 trait

/// 领域模型模块 - 定义核心业务实体
pub mod model;

/// 领域逻辑模块 - 定义业务规则和算法
pub mod logic;

/// 端口模块 - 定义抽象接口（Hexagonal 架构）
pub mod port;
