//! # 端口模块 (Domain Ports)
//!
//! 这是 Hexagonal（六边形）架构的核心部分。
//!
//! ## 什么是端口？
//! 端口是 Domain 层定义的抽象接口（trait），
//! 定义了 Domain 需要的外部能力，但不关心具体实现。
//!
//! ## 端口规则（强制）
//! - 只允许定义 trait
//! - 入参/出参只能是 Domain 对象或基础类型
//! - ❌ 禁止出现 HTTP/DB/SDK/Redis/Kafka 类型

/// 执行端口 - 定义执行指令的抽象接口
pub mod execution_port;

/// 行情事件端口 - 消费行情事件
pub mod market_event_port;

/// 策略端口 - 调用策略计算
pub mod strategy_port;

/// 订单风控端口 - 检查 OrderIntent
pub mod order_risk_port;

/// 订单执行端口 - 执行 OrderIntent
pub mod order_execution_port;

/// 订单仓储端口 - 订单持久化
pub mod order_repository_port;

/// 交易审计端口 - 记录风控决策和执行结果
pub mod trade_audit_port;

/// 交易所查询端口 - 账户/持仓/订单查询（HTTP API 用）
pub mod exchange_query_port;

/// 风控状态端口 - 为风控适配器提供账户状态
pub mod risk_state_port;
