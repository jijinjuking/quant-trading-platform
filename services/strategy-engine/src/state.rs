//! # 应用状态 (Application State)
//! 
//! 管理策略引擎服务的全局状态和配置。

// ============================================================================
// 外部依赖导入
// ============================================================================

use anyhow::Result;
use std::sync::Arc;

// ============================================================================
// 应用状态结构体
// ============================================================================

/// 应用状态 - 跨请求共享的全局状态
#[derive(Clone)]
pub struct AppState {
    /// 应用配置
    #[allow(dead_code)]
    pub config: Arc<AppConfig>,
}

/// 应用配置
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 数据库连接地址
    #[allow(dead_code)]
    pub database_url: String,
}

impl AppState {
    /// 创建新的应用状态实例
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
