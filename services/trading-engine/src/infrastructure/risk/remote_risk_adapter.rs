//! # 远程风控适配器 (Remote Risk Adapter)
//!
//! 通过 HTTP 调用独立的 risk-management 服务进行风控检查。
//!
//! ## 架构说明
//! ```text
//! Trading Engine (大脑) → HTTP → Risk Management (8085)
//! ```
//!
//! ## 使用方式
//! 在 bootstrap.rs 中根据 RISK_MODE 环境变量选择使用：
//! - `remote`: 使用 RemoteRiskAdapter 调用远程服务
//! - `local` 或其他: 使用 OrderRiskAdapter 本地检查

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tracing::{debug, info, warn};
use uuid::Uuid;

use crate::domain::model::order_intent::OrderIntent;
use crate::domain::port::order_risk_port::OrderRiskPort;

/// 远程风控请求
#[derive(Debug, Clone, Serialize)]
struct RemoteRiskCheckRequest {
    strategy_id: Uuid,
    symbol: String,
    side: String,
    quantity: Decimal,
    price: Decimal,
    order_type: String,
}

/// 远程风控响应
#[derive(Debug, Clone, Deserialize)]
struct RemoteRiskCheckResponse {
    approved: bool,
    reason: Option<String>,
}

/// 远程风控适配器
///
/// 通过 HTTP 调用 risk-management 服务进行风控检查。
pub struct RemoteRiskAdapter {
    /// risk-management 服务地址
    base_url: String,
    /// HTTP 客户端
    client: Client,
    /// 本地持仓缓存（用于 update_position）
    positions: Arc<RwLock<std::collections::HashMap<String, Decimal>>>,
}

impl RemoteRiskAdapter {
    /// 创建远程风控适配器
    ///
    /// # 参数
    /// - `base_url`: risk-management 服务地址，如 "http://localhost:8085"
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap_or_default();

        info!(base_url = %base_url, "创建远程风控适配器");

        Self {
            base_url,
            client,
            positions: Arc::new(RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// 从环境变量创建
    pub fn from_env() -> anyhow::Result<Self> {
        let base_url = std::env::var("RISK_MANAGEMENT_URL")
            .unwrap_or_else(|_| "http://localhost:8085".to_string());
        Ok(Self::new(base_url))
    }
}

#[async_trait]
impl OrderRiskPort for RemoteRiskAdapter {
    /// 调用远程风控服务检查交易意图
    async fn check(&self, intent: &OrderIntent) -> anyhow::Result<()> {
        let url = format!("{}/api/v1/risk/check", self.base_url);

        let request = RemoteRiskCheckRequest {
            strategy_id: intent.strategy_id,
            symbol: intent.symbol.clone(),
            side: format!("{:?}", intent.side).to_lowercase(),
            quantity: intent.quantity,
            price: intent.price.unwrap_or(Decimal::ZERO),
            order_type: if intent.price.is_some() { "limit" } else { "market" }.to_string(),
        };

        debug!(
            url = %url,
            symbol = %request.symbol,
            side = %request.side,
            quantity = %request.quantity,
            "调用远程风控服务"
        );

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                warn!(error = %e, "远程风控服务调用失败");
                anyhow::anyhow!("远程风控服务调用失败: {}", e)
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            warn!(status = %status, body = %body, "远程风控服务返回错误");
            return Err(anyhow::anyhow!("远程风控服务返回错误: {} - {}", status, body));
        }

        let result: RemoteRiskCheckResponse = response.json().await.map_err(|e| {
            warn!(error = %e, "解析远程风控响应失败");
            anyhow::anyhow!("解析远程风控响应失败: {}", e)
        })?;

        if result.approved {
            debug!(symbol = %intent.symbol, "远程风控检查通过");
            Ok(())
        } else {
            let reason = result.reason.unwrap_or_else(|| "未知原因".to_string());
            info!(
                symbol = %intent.symbol,
                reason = %reason,
                "远程风控检查拒绝"
            );
            Err(anyhow::anyhow!("风控拒绝: {}", reason))
        }
    }

    /// 更新持仓（本地缓存）
    async fn update_position(&self, symbol: &str, delta: Decimal) {
        let symbol = symbol.trim().to_uppercase();
        let mut positions = self.positions.write().await;

        let current = positions.get(&symbol).copied().unwrap_or(Decimal::ZERO);
        let new_position = current + delta;

        if new_position.is_zero() {
            positions.remove(&symbol);
        } else {
            positions.insert(symbol.clone(), new_position);
        }

        debug!(
            symbol = %symbol,
            delta = %delta,
            new_position = %new_position,
            "本地持仓缓存已更新"
        );
    }

    /// 记录下单时间（远程模式下为空操作）
    async fn record_order_time(&self, symbol: &str) {
        debug!(symbol = %symbol, "记录下单时间（远程模式）");
    }
}
