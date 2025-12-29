//! # 策略引擎库 (Strategy Engine Library)
//! 
//! 对外暴露策略引擎服务的所有模块。

/// 应用状态
pub mod state;

/// 接口层 - HTTP API
pub mod interface;

/// 应用层 - 用例编排
pub mod application;

/// 领域层 - 策略模型和算法
pub mod domain;

/// 基础设施层 - 适配器实现
pub mod infrastructure;
