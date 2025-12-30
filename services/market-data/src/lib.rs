//! # 行情数据服务库 (Market Data Library)
//!
//! ## 模块说明
//! - `application`: 应用层（用例编排）
//! - `domain`: 领域层（端口定义）
//! - `infrastructure`: 基础设施层（适配器实现）

pub mod state;
pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod bootstrap;
