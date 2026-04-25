//! # API 限流器 (Rate Limiter)
//!
//! 实现币安 API 限流控制，防止超过 API 调用限制。
//!
//! ## 币安 API 限流规则
//! - 现货 REST API: 1200 请求/分钟 (20 请求/秒)
//! - 订单权重: 每个订单消耗 1 权重
//! - 查询权重: 根据接口不同，消耗 1-40 权重
//!
//! ## 实现策略
//! - 使用令牌桶算法 (Token Bucket)
//! - 每秒补充 20 个令牌
//! - 桶容量 100 个令牌（允许短时突发）

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{debug, warn};

/// 限流器配置
#[derive(Debug, Clone)]
pub struct RateLimiterConfig {
    /// 每秒补充的令牌数
    pub tokens_per_second: u32,
    /// 令牌桶容量
    pub bucket_capacity: u32,
}

impl Default for RateLimiterConfig {
    fn default() -> Self {
        Self {
            // 币安现货 API 限制约 20 请求/秒，我们设置为 15 留出余量
            tokens_per_second: 15,
            // 桶容量 100，允许短时突发
            bucket_capacity: 100,
        }
    }
}

/// 令牌桶限流器
pub struct RateLimiter {
    config: RateLimiterConfig,
    state: Arc<Mutex<RateLimiterState>>,
}

struct RateLimiterState {
    /// 当前令牌数
    tokens: f64,
    /// 上次补充令牌的时间
    last_refill: Instant,
}

impl RateLimiter {
    /// 创建限流器
    pub fn new(config: RateLimiterConfig) -> Self {
        Self {
            state: Arc::new(Mutex::new(RateLimiterState {
                tokens: config.bucket_capacity as f64,
                last_refill: Instant::now(),
            })),
            config,
        }
    }

    /// 使用默认配置创建
    pub fn default() -> Self {
        Self::new(RateLimiterConfig::default())
    }

    /// 获取令牌（阻塞直到有令牌可用）
    ///
    /// # 参数
    /// - `weight`: 请求权重（通常为 1）
    ///
    /// # 返回
    /// - 等待的时间（用于日志）
    pub async fn acquire(&self, weight: u32) -> Duration {
        let start = Instant::now();
        let mut state = self.state.lock().await;

        loop {
            // 补充令牌
            self.refill_tokens(&mut state);

            // 检查是否有足够的令牌
            if state.tokens >= weight as f64 {
                state.tokens -= weight as f64;
                let wait_time = start.elapsed();

                if wait_time > Duration::from_millis(100) {
                    debug!(
                        weight = weight,
                        wait_ms = wait_time.as_millis(),
                        remaining_tokens = state.tokens,
                        "Rate limiter acquired token after waiting"
                    );
                }

                return wait_time;
            }

            // 计算需要等待的时间
            let tokens_needed = weight as f64 - state.tokens;
            let wait_time = Duration::from_secs_f64(
                tokens_needed / self.config.tokens_per_second as f64
            );

            warn!(
                weight = weight,
                tokens_needed = tokens_needed,
                wait_ms = wait_time.as_millis(),
                "Rate limit reached, waiting for tokens"
            );

            // 释放锁并等待
            drop(state);
            tokio::time::sleep(wait_time).await;
            state = self.state.lock().await;
        }
    }

    /// 尝试获取令牌（非阻塞）
    ///
    /// # 参数
    /// - `weight`: 请求权重
    ///
    /// # 返回
    /// - `true`: 获取成功
    /// - `false`: 令牌不足
    pub async fn try_acquire(&self, weight: u32) -> bool {
        let mut state = self.state.lock().await;
        self.refill_tokens(&mut state);

        if state.tokens >= weight as f64 {
            state.tokens -= weight as f64;
            true
        } else {
            false
        }
    }

    /// 补充令牌
    fn refill_tokens(&self, state: &mut RateLimiterState) {
        let now = Instant::now();
        let elapsed = now.duration_since(state.last_refill).as_secs_f64();

        if elapsed > 0.0 {
            let new_tokens = elapsed * self.config.tokens_per_second as f64;
            state.tokens = (state.tokens + new_tokens).min(self.config.bucket_capacity as f64);
            state.last_refill = now;
        }
    }

    /// 获取当前令牌数（用于监控）
    pub async fn available_tokens(&self) -> f64 {
        let mut state = self.state.lock().await;
        self.refill_tokens(&mut state);
        state.tokens
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter_basic() {
        let limiter = RateLimiter::new(RateLimiterConfig {
            tokens_per_second: 10,
            bucket_capacity: 10,
        });

        // 第一次请求应该立即成功
        let wait_time = limiter.acquire(1).await;
        assert!(wait_time < Duration::from_millis(10));

        // 连续请求 10 次应该都成功
        for _ in 0..9 {
            limiter.acquire(1).await;
        }

        // 此时令牌应该用完，下一次请求需要等待
        let wait_time = limiter.acquire(1).await;
        assert!(wait_time >= Duration::from_millis(90));
    }

    #[tokio::test]
    async fn test_rate_limiter_try_acquire() {
        let limiter = RateLimiter::new(RateLimiterConfig {
            tokens_per_second: 10,
            bucket_capacity: 5,
        });

        // 前 5 次应该成功
        for _ in 0..5 {
            assert!(limiter.try_acquire(1).await);
        }

        // 第 6 次应该失败
        assert!(!limiter.try_acquire(1).await);

        // 等待一段时间后应该恢复
        tokio::time::sleep(Duration::from_millis(200)).await;
        assert!(limiter.try_acquire(1).await);
    }

    #[tokio::test]
    async fn test_rate_limiter_refill() {
        let limiter = RateLimiter::new(RateLimiterConfig {
            tokens_per_second: 10,
            bucket_capacity: 10,
        });

        // 用完所有令牌
        for _ in 0..10 {
            limiter.acquire(1).await;
        }

        // 等待 0.5 秒，应该补充 5 个令牌
        tokio::time::sleep(Duration::from_millis(500)).await;

        let tokens = limiter.available_tokens().await;
        assert!(tokens >= 4.5 && tokens <= 5.5);
    }
}
