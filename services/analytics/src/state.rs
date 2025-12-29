//! # Application State - 应用状态管理
//!
//! ## 模块职责
//! 管理服务运行时的全局状态，包括：
//! - 配置信息（ClickHouse连接等）
//! - 共享资源（数据库连接池、缓存客户端等）
//!
//! ## 设计说明
//! - 使用 `Arc` 包装配置，支持多线程安全共享
//! - 状态通过 Axum 的 `with_state` 注入到路由中

use anyhow::Result;
use std::sync::Arc;

/// 应用状态
///
/// 包含服务运行所需的所有共享状态，
/// 通过 Axum 的状态管理机制注入到各个 Handler 中
#[derive(Clone)]
#[allow(dead_code)] // 骨架阶段，字段暂未使用
pub struct AppState {
    /// 应用配置（Arc包装，支持多线程共享）
    pub config: Arc<AppConfig>,
}

/// 应用配置
///
/// 存储服务运行所需的配置项，
/// 从环境变量或配置文件加载
#[derive(Debug, Clone)]
#[allow(dead_code)] // 骨架阶段，字段暂未使用
pub struct AppConfig {
    /// ClickHouse 数据库连接URL
    /// 用于时序数据存储和查询
    pub clickhouse_url: String,
}

impl AppState {
    /// 创建新的应用状态
    ///
    /// ## 初始化流程
    /// 1. 从环境变量加载配置
    /// 2. 初始化数据库连接（待实现）
    /// 3. 返回包装好的状态实例
    ///
    /// ## 环境变量
    /// - `CLICKHOUSE_URL`: ClickHouse连接地址，默认 `http://localhost:8123`
    pub async fn new() -> Result<Self> {
        // 从环境变量加载配置
        let config = AppConfig {
            clickhouse_url: std::env::var("CLICKHOUSE_URL")
                .unwrap_or_else(|_| "http://localhost:8123".to_string()),
        };
        
        Ok(Self {
            config: Arc::new(config),
        })
    }
}
