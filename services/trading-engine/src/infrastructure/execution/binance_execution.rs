//! # 币安真实下单执行器 (Binance Execution Adapter)
//!
//! 负责调用币安 REST API 进行下单。
//! 仅做协议适配与外部交互，不包含任何业务规则。

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use reqwest::Client;
use sha2::Sha256;
use tracing::info;

use crate::domain::port::execution_port::{ExecutionCommand, ExecutionPort};

type HmacSha256 = Hmac<Sha256>;

/// 币安真实下单执行器
pub struct BinanceExecution {
    api_key: String,
    secret_key: String,
    base_url: String,
    client: Client,
}

impl BinanceExecution {
    /// 创建执行器实例
    pub fn new(api_key: String, secret_key: String, base_url: String) -> Self {
        Self {
            api_key,
            secret_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }

    /// HMAC-SHA256 签名
    fn sign(&self, query: &str) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .context("invalid binance secret key")?;
        mac.update(query.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    fn now_millis() -> Result<u128> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("system time before unix epoch")?;
        Ok(duration.as_millis())
    }
}

#[async_trait]
impl ExecutionPort for BinanceExecution {
    async fn execute(&self, command: &ExecutionCommand) -> Result<()> {
        println!(
            "[BinanceExecution] Executing: symbol={}, side={}, qty={}",
            command.symbol, command.side, command.quantity
        );
        let symbol = command.symbol.trim().to_uppercase();
        let side = match command.side.trim().to_lowercase().as_str() {
            "buy" => "BUY",
            "sell" => "SELL",
            value => return Err(anyhow!("unsupported side: {}", value)),
        };

        let quantity = command.quantity.trim();
        if quantity.is_empty() {
            return Err(anyhow!("quantity is empty"));
        }

        let timestamp = Self::now_millis()?;
        let query = format!(
            "symbol={}&side={}&type=MARKET&quantity={}&timestamp={}",
            symbol, side, quantity, timestamp
        );
        let signature = self.sign(&query)?;
        let url = format!("{}/api/v3/order?{}&signature={}", self.base_url, query, signature);

        let response = self
            .client
            .post(url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .context("binance order request failed")?;

        let status = response.status();
        let body = match response.text().await {
            Ok(text) => text,
            Err(err) => format!("<failed to read response body: {}>", err),
        };

        if !status.is_success() {
            println!(
                "[BinanceExecution] ❌ Order FAILED: symbol={}, side={}, status={}, body={}",
                symbol, side, status, body
            );
            return Err(anyhow!(
                "binance order failed: status={} body={}",
                status,
                body
            ));
        }

        println!(
            "[BinanceExecution] ✅ Order ACCEPTED: symbol={}, side={}, qty={}",
            symbol, side, quantity
        );
        info!(symbol = %symbol, side = %side, "Binance order accepted");
        Ok(())
    }
}
