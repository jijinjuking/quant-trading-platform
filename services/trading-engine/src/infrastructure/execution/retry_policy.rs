//! # 请求重试策略 (Retry Policy)
//!
//! 实现智能重试机制，处理网络错误和临时性失败。
//!
//! ## 重试策略
//! - 指数退避 (Exponential Backoff)
//! - 最大重试次数: 3 次
//! - 初始延迟: 100ms
//! - 最大延迟: 5s
//! - 抖动 (Jitter): ±25%
//!
//! ## 可重试的错误
//! - 网络超时
//! - 连接错误
//! - 5xx 服务器错误
//! - 429 限流错误
//!
//! ## 不可重试的错误
//! - 4xx 客户端错误（除了 429）
//! - 认证错误
//! - 参数错误

use std::time::Duration;
use anyhow::Result;
use rand::Rng;
use tracing::{debug, warn};

/// 重试策略配置
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_retries: u32,
    /// 初始延迟（毫秒）
    pub initial_delay_ms: u64,
    /// 最大延迟（毫秒）
    pub max_delay_ms: u64,
    /// 抖动比例（0.0 - 1.0）
    pub jitter_ratio: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay_ms: 100,
            max_delay_ms: 5000,
            jitter_ratio: 0.25,
        }
    }
}

/// 重试策略
pub struct RetryPolicy {
    config: RetryConfig,
}

impl RetryPolicy {
    /// 创建重试策略
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// 使用默认配置创建
    pub fn default() -> Self {
        Self::new(RetryConfig::default())
    }

    /// 执行带重试的操作
    ///
    /// # 参数
    /// - `operation`: 要执行的异步操作
    /// - `operation_name`: 操作名称（用于日志）
    ///
    /// # 返回
    /// - `Ok(T)`: 操作成功
    /// - `Err`: 所有重试都失败
    pub async fn execute_with_retry<F, Fut, T>(
        &self,
        mut operation: F,
        operation_name: &str,
    ) -> Result<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut attempt = 0;

        loop {
            attempt += 1;

            match operation().await {
                Ok(result) => {
                    if attempt > 1 {
                        debug!(
                            operation = operation_name,
                            attempt = attempt,
                            "Operation succeeded after retry"
                        );
                    }
                    return Ok(result);
                }
                Err(err) => {
                    // 检查是否应该重试
                    if attempt >= self.config.max_retries {
                        warn!(
                            operation = operation_name,
                            attempt = attempt,
                            error = ?err,
                            "Operation failed after max retries"
                        );
                        return Err(err);
                    }

                    // 检查错误是否可重试
                    if !self.is_retryable_error(&err) {
                        warn!(
                            operation = operation_name,
                            attempt = attempt,
                            error = ?err,
                            "Non-retryable error, aborting"
                        );
                        return Err(err);
                    }

                    // 计算延迟时间
                    let delay = self.calculate_delay(attempt);

                    warn!(
                        operation = operation_name,
                        attempt = attempt,
                        delay_ms = delay.as_millis(),
                        error = ?err,
                        "Operation failed, retrying"
                    );

                    // 等待后重试
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    /// 判断错误是否可重试
    fn is_retryable_error(&self, error: &anyhow::Error) -> bool {
        let error_str = error.to_string().to_lowercase();

        // 网络相关错误
        if error_str.contains("timeout")
            || error_str.contains("connection")
            || error_str.contains("network")
            || error_str.contains("dns")
        {
            return true;
        }

        // HTTP 状态码相关
        if error_str.contains("status=429") || error_str.contains("rate limit") {
            // 限流错误，可重试
            return true;
        }

        if error_str.contains("status=5") {
            // 5xx 服务器错误，可重试
            return true;
        }

        if error_str.contains("status=4") {
            // 4xx 客户端错误（除了 429），不可重试
            return false;
        }

        // 币安特定错误码
        if error_str.contains("-1003") {
            // Too many requests，可重试
            return true;
        }

        if error_str.contains("-1021") {
            // Timestamp out of sync，不可重试
            return false;
        }

        if error_str.contains("-2010") || error_str.contains("-2011") {
            // 余额不足、订单被拒绝，不可重试
            return false;
        }

        // 默认不重试
        false
    }

    /// 计算延迟时间（指数退避 + 抖动）
    fn calculate_delay(&self, attempt: u32) -> Duration {
        // 指数退避: delay = initial_delay * 2^(attempt - 1)
        let base_delay = self.config.initial_delay_ms * 2_u64.pow(attempt - 1);
        let delay = base_delay.min(self.config.max_delay_ms);

        // 添加抖动: delay ± (delay * jitter_ratio)
        let jitter_range = (delay as f64 * self.config.jitter_ratio) as u64;
        let mut rng = rand::thread_rng();
        let jitter = rng.gen_range(0..=jitter_range * 2);
        let final_delay = delay.saturating_sub(jitter_range).saturating_add(jitter);

        Duration::from_millis(final_delay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_success_on_first_attempt() {
        let policy = RetryPolicy::default();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let result = policy
            .execute_with_retry(
                || {
                    let c = counter_clone.clone();
                    async move {
                        c.fetch_add(1, Ordering::SeqCst);
                        Ok::<_, anyhow::Error>(42)
                    }
                },
                "test_operation",
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let policy = RetryPolicy::new(RetryConfig {
            max_retries: 3,
            initial_delay_ms: 10,
            max_delay_ms: 100,
            jitter_ratio: 0.0,
        });

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let result = policy
            .execute_with_retry(
                || {
                    let c = counter_clone.clone();
                    async move {
                        let count = c.fetch_add(1, Ordering::SeqCst);
                        if count < 2 {
                            Err(anyhow!("timeout error"))
                        } else {
                            Ok::<_, anyhow::Error>(42)
                        }
                    }
                },
                "test_operation",
            )
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_max_retries_exceeded() {
        let policy = RetryPolicy::new(RetryConfig {
            max_retries: 2,
            initial_delay_ms: 10,
            max_delay_ms: 100,
            jitter_ratio: 0.0,
        });

        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let result = policy
            .execute_with_retry(
                || {
                    let c = counter_clone.clone();
                    async move {
                        c.fetch_add(1, Ordering::SeqCst);
                        Err::<i32, _>(anyhow!("timeout error"))
                    }
                },
                "test_operation",
            )
            .await;

        assert!(result.is_err());
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn test_non_retryable_error() {
        let policy = RetryPolicy::default();
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();

        let result = policy
            .execute_with_retry(
                || {
                    let c = counter_clone.clone();
                    async move {
                        c.fetch_add(1, Ordering::SeqCst);
                        Err::<i32, _>(anyhow!("status=400 bad request"))
                    }
                },
                "test_operation",
            )
            .await;

        assert!(result.is_err());
        // 不可重试的错误应该只执行一次
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_is_retryable_error() {
        let policy = RetryPolicy::default();

        // 可重试的错误
        assert!(policy.is_retryable_error(&anyhow!("timeout error")));
        assert!(policy.is_retryable_error(&anyhow!("connection refused")));
        assert!(policy.is_retryable_error(&anyhow!("status=500 internal server error")));
        assert!(policy.is_retryable_error(&anyhow!("status=429 too many requests")));
        assert!(policy.is_retryable_error(&anyhow!("binance error -1003")));

        // 不可重试的错误
        assert!(!policy.is_retryable_error(&anyhow!("status=400 bad request")));
        assert!(!policy.is_retryable_error(&anyhow!("status=401 unauthorized")));
        assert!(!policy.is_retryable_error(&anyhow!("binance error -2010")));
        assert!(!policy.is_retryable_error(&anyhow!("binance error -1021")));
    }

    #[test]
    fn test_calculate_delay() {
        let policy = RetryPolicy::new(RetryConfig {
            max_retries: 5,
            initial_delay_ms: 100,
            max_delay_ms: 1000,
            jitter_ratio: 0.0, // 禁用抖动以便测试
        });

        // 第 1 次重试: 100ms
        let delay1 = policy.calculate_delay(1);
        assert_eq!(delay1.as_millis(), 100);

        // 第 2 次重试: 200ms
        let delay2 = policy.calculate_delay(2);
        assert_eq!(delay2.as_millis(), 200);

        // 第 3 次重试: 400ms
        let delay3 = policy.calculate_delay(3);
        assert_eq!(delay3.as_millis(), 400);

        // 第 4 次重试: 800ms
        let delay4 = policy.calculate_delay(4);
        assert_eq!(delay4.as_millis(), 800);

        // 第 5 次重试: 1000ms (达到最大值)
        let delay5 = policy.calculate_delay(5);
        assert_eq!(delay5.as_millis(), 1000);
    }
}
