//! # 风控基础设施适配器
//!
//! 路径: services/trading-engine/src/infrastructure/risk/mod.rs
//!
//! ## 职责
//! 实现 OrderRiskPort 和 RiskStatePort 的适配器。
//!
//! ## 架构说明
//! - OrderRiskAdapter: 实盘级本地风控适配器（推荐）
//! - RemoteRiskAdapter: 生产环境使用，调用 risk-management 服务
//! - MockRiskAdapter: 测试/开发环境使用，基于 RiskStatePort 本地实现
//! - InMemoryRiskStateAdapter: 内存风控状态，用于测试

pub mod inmemory_risk_state;
pub mod mock_risk_adapter;
pub mod order_risk_adapter;
pub mod remote_risk_adapter;
pub mod risk_limits;

pub use inmemory_risk_state::InMemoryRiskStateAdapter;
pub use mock_risk_adapter::{MockRiskAdapter, MockRiskConfig};
pub use order_risk_adapter::OrderRiskAdapter;
pub use remote_risk_adapter::RemoteRiskAdapter;
pub use risk_limits::{OrderRiskConfig, RiskLimits};
