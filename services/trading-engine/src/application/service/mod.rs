//! # 应用服务模块
//!
//! ## 功能层级: 【应用层 Application】
//! ## 职责: 包含所有用例编排服务

/// 交易主链路调度服务 - 统一调度 Strategy → Risk → Execution
pub mod execution_service;

/// 行情事件消费服务 - 消费行情并转发给 ExecutionService
pub mod market_event_consumer_service;

/// 风控状态初始化服务 - 启动时从交易所同步状态到 RiskStatePort
pub mod risk_state_initializer;

/// 风控状态协调器 - 统一管理 RiskState 生命周期 (v1.1 集成重构)
pub mod risk_state_coordinator;

/// 成交回报处理服务 - 处理成交事件驱动 RiskState 修正
pub mod fill_processor;

/// 订单生命周期服务 - 订单超时检测与处理 (v1.1 安全修补)
pub mod order_lifecycle_service;
