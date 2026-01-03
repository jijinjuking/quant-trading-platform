//! # 应用状态模块
//!
//! 本模块定义 API Gateway 服务的全局应用状态。
//!
//! ## 职责
//! - 管理服务配置信息
//! - 管理后端服务地址
//! - 提供 HTTP 客户端共享

use anyhow::Result;
use std::sync::Arc;

/// 应用状态
///
/// 包含服务运行所需的所有共享状态和配置。
#[derive(Clone)]
pub struct AppState {
    /// 应用配置
    pub config: Arc<AppConfig>,
    /// HTTP 客户端（用于代理请求）
    pub http_client: reqwest::Client,
}

/// 应用配置
///
/// 存储服务运行所需的配置参数。
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// JWT 签名密钥
    pub jwt_secret: String,
    /// Redis 连接 URL
    pub redis_url: String,
    /// 后端服务地址
    pub services: ServiceEndpoints,
}

/// 后端服务端点配置
#[derive(Debug, Clone)]
pub struct ServiceEndpoints {
    /// 策略引擎地址
    pub strategy_engine: String,
    /// 交易引擎地址
    pub trading_engine: String,
    /// 行情服务地址
    pub market_data: String,
    /// 用户管理地址
    pub user_management: String,
    /// 风控服务地址
    pub risk_management: String,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// 从环境变量读取配置
    pub async fn new() -> Result<Self> {
        let config = AppConfig {
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret".to_string()),
            redis_url: std::env::var("REDIS_URL")
                .unwrap_or_else(|_| "redis://localhost:6379".to_string()),
            services: ServiceEndpoints {
                strategy_engine: std::env::var("STRATEGY_ENGINE_URL")
                    .unwrap_or_else(|_| "http://localhost:8083".to_string()),
                trading_engine: std::env::var("TRADING_ENGINE_URL")
                    .unwrap_or_else(|_| "http://localhost:8081".to_string()),
                market_data: std::env::var("MARKET_DATA_URL")
                    .unwrap_or_else(|_| "http://localhost:8082".to_string()),
                user_management: std::env::var("USER_MANAGEMENT_URL")
                    .unwrap_or_else(|_| "http://localhost:8084".to_string()),
                risk_management: std::env::var("RISK_MANAGEMENT_URL")
                    .unwrap_or_else(|_| "http://localhost:8085".to_string()),
            },
        };
        
        // 创建 HTTP 客户端
        let http_client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(|e| anyhow::anyhow!("创建 HTTP 客户端失败: {}", e))?;
        
        Ok(Self {
            config: Arc::new(config),
            http_client,
        })
    }
}
