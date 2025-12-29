//! # 领域逻辑 (Domain Logic)
//! 
//! 定义交易引擎的核心业务规则和算法。
//! 
//! ## 子模块
//! - `execution_algo`: 订单执行算法（TWAP/VWAP/冰山等）
//! 
//! ## 设计原则
//! - 纯函数实现，无副作用
//! - 不依赖外部服务
//! - 只操作 Domain 对象

/// 执行算法模块 - 定义订单执行策略
pub mod execution_algo;
