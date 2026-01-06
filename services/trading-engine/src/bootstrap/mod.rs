//! # 依赖注入模块 (Bootstrap)
//!
//! 路径: services/trading-engine/src/bootstrap/mod.rs
//!
//! ## 职责
//! 组装应用层服务，完成依赖注入。
//! 在六边形架构中，bootstrap 负责：
//! - 创建 infrastructure adapter 实例
//! - 将 adapter 注入到 application service
//! - 启动后台服务
//!
//! ## 模块结构
//! - database: PostgreSQL 连接池
//! - strategy: 策略端口工厂
//! - risk: 风控端口工厂
//! - execution: 执行端口工厂
//! - lifecycle: 订单生命周期服务工厂 (v1.1)
//! - consumer: 行情消费服务组装
//! - background: 后台服务启动 (v1.1 集成重构)

pub mod database;
pub mod strategy;
pub mod risk;
pub mod execution;
pub mod lifecycle;
pub mod consumer;
pub mod background;

// 重新导出常用函数，保持向后兼容
pub use consumer::create_market_event_consumer_compat as create_market_event_consumer_legacy;
pub use consumer::{ConsumerConfig, ConsumerConfigWithState, create_market_event_consumer_with_state};
pub use lifecycle::create_order_lifecycle_service;
pub use risk::create_risk_state;
pub use background::{start_background_services, BackgroundHandles};
