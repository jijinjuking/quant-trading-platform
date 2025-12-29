//! # 应用状态模块
//!
//! 本模块定义 API Gateway 服务的全局应用状态。
//!
//! ## 职责
//! - 管理服务配置信息
//! - 提供共享资源的线程安全访问
//! - 初始化服务依赖
//!
//! ## 使用方式
//! ```ignore
//! let state = AppState::new().await?;
//! let app = Router::new().with_state(state);
//! ```

use anyhow::Result;
use std::sync::Arc;

/// 应用状态
///
/// 包含服务运行所需的所有共享状态和配置。
/// 使用 `Arc` 包装以支持多线程安全共享。
///
/// # 字段
/// - `config`: 应用配置信息
#[derive(Clone)]
#[allow(dead_code)]
pub struct AppState {
    /// 应用配置（线程安全共享）
    pub config: Arc<AppConfig>,
}

/// 应用配置
///
/// 存储服务运行所需的配置参数。
/// 支持从环境变量读取，提供默认值。
///
/// # 字段
/// - `jwt_secret`: JWT 签名密钥
/// - `redis_url`: Redis 连接地址
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct AppConfig {
    /// JWT 签名密钥，用于 Token 验证
    pub jwt_secret: String,
    /// Redis 连接 URL，用于缓存和限流
    pub redis_url: String,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// 从环境变量读取配置，如果环境变量不存在则使用默认值。
    ///
    /// # 环境变量
    /// - `JWT_SECRET`: JWT 密钥（默认: "secret"）
    /// - `REDIS_URL`: Redis 地址（默认: "redis://localhost:6379"）
    ///
    /// # 返回值
    /// - `Ok(AppState)` - 成功创建应用状态
    /// - `Err(anyhow::Error)` - 初始化失败
    ///
    /// # 示例
    /// ```ignore
    /// let state = AppState::new().await?;
    /// ```
    pub async fn new() -> Result<Self> {
        // 从环境变量读取配置，提供默认值
        let config = AppConfig {
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
        };
        
        Ok(Self {
            config: Arc::new(config),
        })
    }
}
