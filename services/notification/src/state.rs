//! # 应用状态模块
//!
//! 本文件定义通知服务的全局应用状态，包含配置信息和共享资源。
//!
//! ## 职责
//! - 管理服务配置（Redis连接等）
//! - 提供线程安全的状态共享

use anyhow::Result;
use std::sync::Arc;

/// 应用状态
///
/// 包含服务运行所需的所有共享状态，通过 `Arc` 实现线程安全共享。
/// 在 Axum 路由中作为 State 传递给各个处理器。
#[allow(dead_code)]
#[derive(Clone)]
pub struct AppState {
    /// 应用配置（线程安全引用）
    pub config: Arc<AppConfig>,
}

/// 应用配置
///
/// 存储服务运行所需的配置参数，从环境变量加载。
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Redis 连接 URL，用于消息队列和缓存
    pub redis_url: String,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// 从环境变量加载配置，初始化所有必要的连接和资源。
    ///
    /// # 返回值
    /// - `Ok(AppState)`: 成功创建的应用状态
    /// - `Err`: 初始化过程中发生错误
    pub async fn new() -> Result<Self> {
        // 从环境变量加载 Redis URL，默认使用本地地址
        let config = AppConfig {
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        };
        
        Ok(Self {
            config: Arc::new(config),
        })
    }
}
