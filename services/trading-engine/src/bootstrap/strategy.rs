//! # 策略端口工厂
//!
//! 路径: services/trading-engine/src/bootstrap/strategy.rs
//!
//! ## 职责
//! 根据配置创建 StrategyPort 实现

use std::sync::Arc;

use anyhow::anyhow;

use crate::domain::port::strategy_port::StrategyPort;
use crate::infrastructure::strategy::{NoopStrategy, RemoteStrategy};

/// 创建策略端口
///
/// # 参数
/// - `mode`: 策略模式 ("remote" | "noop")
/// - `url`: 远程策略服务 URL（remote 模式必需）
///
/// # 返回
/// - `Arc<dyn StrategyPort>`: 策略端口实例
pub fn create_strategy_port(
    mode: &str,
    url: Option<String>,
) -> anyhow::Result<Arc<dyn StrategyPort>> {
    match mode.trim().to_lowercase().as_str() {
        "remote" => {
            let url = url
                .ok_or_else(|| anyhow!("STRATEGY_ENGINE_URL is required for remote strategy"))?;
            tracing::info!(url = %url, "使用远程策略服务");
            Ok(Arc::new(RemoteStrategy::new(url)))
        }
        _ => {
            tracing::info!("使用 Noop 策略（不生成信号）");
            Ok(Arc::new(NoopStrategy::new()))
        }
    }
}
