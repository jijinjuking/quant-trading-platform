//! # 数据库连接池工厂
//!
//! 路径: services/trading-engine/src/bootstrap/database.rs
//!
//! ## 职责
//! 创建 PostgreSQL 连接池

use anyhow::Context;
use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

/// 创建 PostgreSQL 连接池
pub async fn create_postgres_pool() -> anyhow::Result<Pool> {
    let host = std::env::var("POSTGRES_HOST")
        .unwrap_or_else(|_| "localhost".to_string());
    let port: u16 = std::env::var("POSTGRES_PORT")
        .unwrap_or_else(|_| "5432".to_string())
        .parse()
        .unwrap_or(5432);
    let user = std::env::var("POSTGRES_USER")
        .unwrap_or_else(|_| "postgres".to_string());
    let password = std::env::var("POSTGRES_PASSWORD")
        .unwrap_or_else(|_| "password".to_string());
    let dbname = std::env::var("POSTGRES_DATABASE")
        .unwrap_or_else(|_| "trading_db".to_string());

    let mut cfg = Config::new();
    cfg.host = Some(host);
    cfg.port = Some(port);
    cfg.user = Some(user);
    cfg.password = Some(password);
    cfg.dbname = Some(dbname);

    let pool = cfg
        .create_pool(Some(Runtime::Tokio1), NoTls)
        .context("创建 PostgreSQL 连接池失败")?;

    // 测试连接
    let _client = pool.get().await.context("无法连接到 PostgreSQL")?;
    tracing::info!("PostgreSQL 连接池创建成功");

    Ok(pool)
}
