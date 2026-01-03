//! # 领域模型 (Domain Models)
//!
//! 定义交易引擎的核心领域模型。
//!
//! ## 设计原则
//! - 所有模型都是纯数据结构
//! - 不包含任何外部依赖（HTTP/DB/SDK）
//! - 可被其他层安全引用

/// 审计事件模型 - 用于记录风控决策和执行结果
pub mod audit_event;

/// 交易意图模型 - 策略产生的交易意图
pub mod order_intent;

/// 订单模型
pub mod order;

/// 成交记录模型
pub mod trade;

/// 成交回报领域事件 - 用于驱动 RiskState 修正
pub mod execution_fill;
