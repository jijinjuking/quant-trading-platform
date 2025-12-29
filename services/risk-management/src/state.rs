//! # 应用状态模块 (Application State)
//!
//! 本模块定义风险管理服务的全局应用状态，包括配置信息、数据库连接池等。
//! 应用状态在服务启动时初始化，并在各个请求处理器之间共享。

use anyhow::Result;
use std::sync::Arc;

/// 应用状态
///
/// 包含服务运行所需的全局状态，如配置、数据库连接等。
/// 使用 `Arc` 包装以支持多线程安全共享。
#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    /// 应用配置（线程安全共享）
    pub config: Arc<AppConfig>,
}

/// 应用配置
///
/// 存储服务运行所需的配置参数。
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 数据库连接 URL
    pub database_url: String,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// 从环境变量读取配置，初始化数据库连接等资源。
    ///
    /// # 返回值
    /// - `Ok(AppState)`: 初始化成功
    /// - `Err`: 初始化失败（如数据库连接失败）
    pub async fn new() -> Result<Self> {
        let config = AppConfig {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/trading".to_string()),
        };
        
        Ok(Self {
            config: Arc::new(config),
        })
    }
}
