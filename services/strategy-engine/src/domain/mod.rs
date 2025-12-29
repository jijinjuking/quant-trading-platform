//! # 领域层 (Domain Layer)
//! 
//! 策略引擎的核心领域层。

/// 领域模型 - 策略、信号
pub mod model;

/// 领域逻辑 - 策略算法
pub mod logic;

/// 领域服务 - 跨模型规则
pub mod service;

/// 端口定义 - 抽象接口
pub mod port;
