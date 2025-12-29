//! # 领域层 (Domain Layer)
//! 
//! 行情数据服务的核心领域层。
//! 
//! ## 子模块
//! - `model`: 领域模型（Tick、Kline）
//! - `port`: 端口定义（trait 接口）
//! 
//! ## 依赖规则
//! - Domain 层不依赖任何外部层
//! - 只使用标准库和 shared 模块

/// 领域模型 - Tick、Kline 等数据结构
pub mod model;

/// 端口定义 - 抽象接口（Hexagonal 架构）
pub mod port;
