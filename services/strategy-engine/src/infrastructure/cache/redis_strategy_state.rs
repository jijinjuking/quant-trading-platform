//! # Redis 策略状态适配器 (Redis Strategy State Adapter)
//!
//! 使用 Redis 存储策略运行状态。
//!
//! ## Key 格式
//! - 网格策略: `strategy:grid:{strategy_id}`
//! - 均值回归: `strategy:mean:{strategy_id}`
//!
//! ## TTL
//! 默认 24 小时过期，可配置

use anyhow::{Context, Result};
use async_trait::async_trait;
use redis::AsyncCommands;

use crate::domain::port::strategy_state_port::{
    GridStateData, MeanReversionStateData, StrategyStatePort,
};

/// Redis 策略状态适配器
pub struct RedisStrategyStateAdapter {
    /// Redis 连接池
    client: redis::Client,
    /// Key 前缀
    key_prefix: String,
    /// TTL（秒）
    ttl_seconds: u64,
}

impl RedisStrategyStateAdapter {
    /// 创建 Redis 策略状态适配器
    ///
    /// # 参数
    /// - `redis_url`: Redis 连接 URL (如 `redis://localhost:6379`)
    /// - `key_prefix`: Key 前缀（默认 `strategy`）
    /// - `ttl_seconds`: TTL 秒数（默认 86400 = 24小时）
    pub fn new(redis_url: &str, key_prefix: Option<String>, ttl_seconds: Option<u64>) -> Result<Self> {
        let client = redis::Client::open(redis_url)
            .context("创建 Redis 客户端失败")?;

        Ok(Self {
            client,
            key_prefix: key_prefix.unwrap_or_else(|| "strategy".to_string()),
            ttl_seconds: ttl_seconds.unwrap_or(86400),
        })
    }

    /// 从环境变量创建
    pub fn from_env() -> Result<Self> {
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost:6379".to_string());
        let key_prefix = std::env::var("STRATEGY_STATE_KEY_PREFIX").ok();
        let ttl_seconds = std::env::var("STRATEGY_STATE_TTL_SECONDS")
            .ok()
            .and_then(|s| s.parse().ok());

        Self::new(&redis_url, key_prefix, ttl_seconds)
    }

    /// 构建网格策略 Key
    fn grid_key(&self, strategy_id: &str) -> String {
        format!("{}:grid:{}", self.key_prefix, strategy_id)
    }

    /// 构建均值回归策略 Key
    fn mean_key(&self, strategy_id: &str) -> String {
        format!("{}:mean:{}", self.key_prefix, strategy_id)
    }

    /// 获取异步连接
    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection> {
        self.client
            .get_multiplexed_async_connection()
            .await
            .context("获取 Redis 连接失败")
    }
}

#[async_trait]
impl StrategyStatePort for RedisStrategyStateAdapter {
    async fn get_grid_state(&self, strategy_id: &str) -> Result<Option<GridStateData>> {
        let mut conn = self.get_connection().await?;
        let key = self.grid_key(strategy_id);

        let value: Option<String> = conn
            .get(&key)
            .await
            .context("Redis GET 失败")?;

        match value {
            Some(json) => {
                let state: GridStateData = serde_json::from_str(&json)
                    .context("反序列化 GridStateData 失败")?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    async fn save_grid_state(&self, strategy_id: &str, state: &GridStateData) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let key = self.grid_key(strategy_id);

        let json = serde_json::to_string(state)
            .context("序列化 GridStateData 失败")?;

        let _: () = conn.set_ex(&key, &json, self.ttl_seconds)
            .await
            .context("Redis SETEX 失败")?;

        Ok(())
    }

    async fn get_mean_reversion_state(
        &self,
        strategy_id: &str,
    ) -> Result<Option<MeanReversionStateData>> {
        let mut conn = self.get_connection().await?;
        let key = self.mean_key(strategy_id);

        let value: Option<String> = conn
            .get(&key)
            .await
            .context("Redis GET 失败")?;

        match value {
            Some(json) => {
                let state: MeanReversionStateData = serde_json::from_str(&json)
                    .context("反序列化 MeanReversionStateData 失败")?;
                Ok(Some(state))
            }
            None => Ok(None),
        }
    }

    async fn save_mean_reversion_state(
        &self,
        strategy_id: &str,
        state: &MeanReversionStateData,
    ) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let key = self.mean_key(strategy_id);

        let json = serde_json::to_string(state)
            .context("序列化 MeanReversionStateData 失败")?;

        let _: () = conn.set_ex(&key, &json, self.ttl_seconds)
            .await
            .context("Redis SETEX 失败")?;

        Ok(())
    }

    async fn delete_state(&self, strategy_id: &str) -> Result<()> {
        let mut conn = self.get_connection().await?;
        let grid_key = self.grid_key(strategy_id);
        let mean_key = self.mean_key(strategy_id);

        // 删除两种类型的 Key
        let _: () = conn
            .del(&[&grid_key, &mean_key])
            .await
            .context("Redis DEL 失败")?;

        Ok(())
    }
}
