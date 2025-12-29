//! # Analytics Library - 数据分析服务库
//!
//! ## 服务职责
//! 数据分析服务负责：
//! - 交易绩效分析（收益率、夏普比率、最大回撤等）
//! - 统计报表生成
//! - 时序数据查询（基于ClickHouse）
//!
//! ## 模块结构
//! ```text
//! analytics/
//! ├── state          - 应用状态管理
//! ├── interface      - 接口层（HTTP API）
//! ├── application    - 应用层（用例编排）
//! ├── domain         - 领域层（核心业务模型和端口）
//! └── infrastructure - 基础设施层（ClickHouse适配器）
//! ```
//!
//! ## 架构说明
//! 遵循 DDD + 六边形架构（端口-适配器模式）：
//! - Domain层定义端口（trait），不依赖外部
//! - Infrastructure层实现端口，负责数据存储
//! - Application层编排用例，只依赖trait

/// 应用状态模块
pub mod state;

/// 接口层 - HTTP/gRPC入口
pub mod interface;

/// 应用层 - 用例编排
pub mod application;

/// 领域层 - 核心业务模型和端口定义
pub mod domain;

/// 基础设施层 - 外部服务适配器
pub mod infrastructure;
