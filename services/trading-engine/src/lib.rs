//! # 交易引擎服务 - 库文件
//! 
//! ## 功能层级: 【入口层】
//! ## 职责: 对外暴露模块，供其他crate引用

// ============================================================
// 公开模块导出
// ============================================================

/// 应用状态模块 - 包含AppState和配置
pub mod state;

/// 接口层 - HTTP/gRPC入口点
pub mod interface;

/// 应用层 - 用例编排，只依赖trait
pub mod application;

/// 领域层 - 核心业务逻辑，不依赖外部框架
pub mod domain;

/// 基础设施层 - 实现domain::port中的trait
pub mod infrastructure;

/// 依赖注入模块 - 组装应用层服务
pub mod bootstrap;
