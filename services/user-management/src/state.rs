//! # 应用状态模块
//!
//! 本文件定义用户管理服务的全局应用状态。
//!
//! ## 所属层
//! 跨层共享 - 被所有层使用的状态容器
//!
//! ## 职责
//! - 管理应用配置
//! - 提供数据库连接信息
//! - 管理 JWT 密钥等安全配置

#![allow(dead_code)]

use anyhow::Result;
use std::sync::Arc;

/// 应用状态
///
/// 包含服务运行所需的所有共享状态，通过 `Arc` 实现线程安全的共享。
/// 在 Axum 路由中作为状态注入到各个处理器中。
#[derive(Clone)]
pub struct AppState {
    /// 应用配置（使用 Arc 包装以支持多线程共享）
    pub config: Arc<AppConfig>,
}

/// 应用配置
///
/// 包含服务运行所需的配置项，从环境变量中读取。
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 数据库连接 URL
    pub database_url: String,
    /// JWT 签名密钥
    pub jwt_secret: String,
}

impl AppState {
    /// 创建新的应用状态
    ///
    /// 从环境变量中读取配置，如果环境变量不存在则使用默认值。
    ///
    /// # 返回值
    /// - `Ok(AppState)`: 成功创建的应用状态
    /// - `Err`: 初始化失败时的错误
    ///
    /// # 环境变量
    /// - `DATABASE_URL`: 数据库连接字符串
    /// - `JWT_SECRET`: JWT 签名密钥
    pub async fn new() -> Result<Self> {
        let config = AppConfig {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/trading".to_string()),
            jwt_secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "secret".to_string()),
        };
        
        Ok(Self {
            config: Arc::new(config),
        })
    }
}
