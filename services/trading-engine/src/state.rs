//! # 应用状态模块
//! 
//! ## 功能层级: 【入口层】
//! ## 职责: 管理应用级别的共享状态（配置、连接池等）

// ============================================================
// 外部依赖导入
// ============================================================
use anyhow::Result;       // 错误处理
use std::sync::Arc;       // 原子引用计数，用于跨线程共享

// ============================================================
// 应用状态结构体
// ============================================================

/// # AppState - 应用状态
/// 
/// ## 说明:
/// - 被所有HTTP handler共享
/// - 使用Arc包装以支持多线程
/// - Clone trait允许在handler间传递
#[derive(Clone)]
pub struct AppState {
    /// 应用配置（Arc包装，多线程安全）
    pub config: Arc<AppConfig>,
}

/// # AppConfig - 应用配置
/// 
/// ## 说明:
/// - 从环境变量读取配置
/// - Debug trait用于日志输出
/// - Clone trait用于复制
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 数据库连接URL
    pub database_url: String,
}

// ============================================================
// AppState 实现
// ============================================================

impl AppState {
    /// # 创建新的应用状态
    /// 
    /// ## 返回:
    /// - Ok(AppState) - 创建成功
    /// - Err - 创建失败
    /// 
    /// ## 执行流程:
    /// 1. 从环境变量读取配置
    /// 2. 创建AppConfig
    /// 3. 包装为Arc并返回
    pub async fn new() -> Result<Self> {
        // 从环境变量读取数据库URL，提供默认值
        let config = AppConfig {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/trading".to_string()),
        };
        
        // 返回AppState，config用Arc包装
        Ok(Self {
            config: Arc::new(config),
        })
    }
}
