//! # 应用状态模块
//!
//! 管理 AI 服务的全局状态和配置信息。
//! 包含 DeepSeek API 连接配置等关键信息。

use anyhow::Result;
use std::sync::Arc;

/// 应用全局状态
///
/// 在 Axum 路由中通过 `State` 提取器共享，
/// 包含服务运行所需的所有配置和连接信息。
#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    /// 应用配置（线程安全共享）
    pub config: Arc<AppConfig>,
}

/// 应用配置
///
/// 存储 AI 服务所需的外部服务配置信息。
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppConfig {
    /// DeepSeek API 密钥
    pub deepseek_api_key: String,
    /// DeepSeek API 基础 URL
    pub deepseek_base_url: String,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// 从环境变量加载配置：
    /// - `DEEPSEEK_API_KEY`: DeepSeek API 密钥
    /// - `DEEPSEEK_BASE_URL`: DeepSeek API 地址（默认 https://api.deepseek.com）
    ///
    /// # Returns
    /// 返回初始化完成的 `AppState` 实例
    ///
    /// # Errors
    /// 当前实现不会返回错误，但保留 Result 以便未来扩展
    pub async fn new() -> Result<Self> {
        let config = AppConfig {
            deepseek_api_key: std::env::var("DEEPSEEK_API_KEY")
                .unwrap_or_else(|_| "".to_string()),
            deepseek_base_url: std::env::var("DEEPSEEK_BASE_URL")
                .unwrap_or_else(|_| "https://api.deepseek.com".to_string()),
        };
        
        Ok(Self {
            config: Arc::new(config),
        })
    }
}
