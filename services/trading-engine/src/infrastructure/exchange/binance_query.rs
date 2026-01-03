//! # 币安查询适配器 (Binance Query Adapter)
//!
//! 路径: services/trading-engine/src/infrastructure/exchange/binance_query.rs
//!
//! ## 职责
//! 实现 ExchangeQueryPort，调用币安 REST API 查询账户信息。
//!
//! ## 架构位置
//! - 所属层级: Infrastructure Layer
//! - 实现端口: domain/port/exchange_query_port.rs

use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use hmac::{Hmac, Mac};
use reqwest::Client;
use rust_decimal::Decimal;
use sha2::Sha256;
use std::str::FromStr;
use tracing::{debug, error};

use crate::domain::port::exchange_query_port::{
    AccountBalance, CancelOrderResult, ExchangeOrder, ExchangeOrderStatus,
    ExchangeQueryPort, Position,
};

type HmacSha256 = Hmac<Sha256>;

/// 币安查询适配器
pub struct BinanceQueryAdapter {
    api_key: String,
    secret_key: String,
    base_url: String,
    client: Client,
}

impl BinanceQueryAdapter {
    /// 创建适配器实例
    pub fn new(api_key: String, secret_key: String, base_url: String) -> Self {
        Self {
            api_key,
            secret_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            client: Client::new(),
        }
    }

    /// 从环境变量创建
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var("BINANCE_API_KEY")
            .context("BINANCE_API_KEY not set")?;
        let secret_key = std::env::var("BINANCE_SECRET_KEY")
            .context("BINANCE_SECRET_KEY not set")?;
        let base_url = std::env::var("BINANCE_BASE_URL")
            .unwrap_or_else(|_| "https://testnet.binance.vision".to_string());

        Ok(Self::new(api_key, secret_key, base_url))
    }

    /// HMAC-SHA256 签名
    fn sign(&self, query: &str) -> Result<String> {
        let mut mac = HmacSha256::new_from_slice(self.secret_key.as_bytes())
            .context("invalid binance secret key")?;
        mac.update(query.as_bytes());
        Ok(hex::encode(mac.finalize().into_bytes()))
    }

    /// 获取当前时间戳（毫秒）
    fn now_millis() -> Result<u128> {
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("system time before unix epoch")?;
        Ok(duration.as_millis())
    }

    /// 发送签名 GET 请求
    async fn signed_get(&self, endpoint: &str, params: &str) -> Result<serde_json::Value> {
        let timestamp = Self::now_millis()?;
        let query = if params.is_empty() {
            format!("timestamp={}", timestamp)
        } else {
            format!("{}&timestamp={}", params, timestamp)
        };
        let signature = self.sign(&query)?;
        let url = format!("{}/{}?{}&signature={}", self.base_url, endpoint, query, signature);

        debug!(url = %url, "Binance signed GET request");

        let response = self
            .client
            .get(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .context("binance request failed")?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if !status.is_success() {
            error!(status = %status, body = %body, "Binance API error");
            return Err(anyhow!("binance API error: status={} body={}", status, body));
        }

        serde_json::from_str(&body).context("failed to parse binance response")
    }

    /// 发送签名 DELETE 请求
    async fn signed_delete(&self, endpoint: &str, params: &str) -> Result<serde_json::Value> {
        let timestamp = Self::now_millis()?;
        let query = format!("{}&timestamp={}", params, timestamp);
        let signature = self.sign(&query)?;
        let url = format!("{}/{}?{}&signature={}", self.base_url, endpoint, query, signature);

        debug!(url = %url, "Binance signed DELETE request");

        let response = self
            .client
            .delete(&url)
            .header("X-MBX-APIKEY", &self.api_key)
            .send()
            .await
            .context("binance request failed")?;

        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if !status.is_success() {
            error!(status = %status, body = %body, "Binance API error");
            return Err(anyhow!("binance API error: status={} body={}", status, body));
        }

        serde_json::from_str(&body).context("failed to parse binance response")
    }

    /// 解析 Decimal
    fn parse_decimal(value: &serde_json::Value) -> Decimal {
        value
            .as_str()
            .and_then(|s| Decimal::from_str(s).ok())
            .unwrap_or(Decimal::ZERO)
    }
}

#[async_trait]
impl ExchangeQueryPort for BinanceQueryAdapter {
    /// 查询现货账户余额
    async fn get_spot_balances(&self) -> Result<Vec<AccountBalance>> {
        let data = self.signed_get("api/v3/account", "").await?;

        let balances = data["balances"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|b| {
                        let free = Self::parse_decimal(&b["free"]);
                        let locked = Self::parse_decimal(&b["locked"]);
                        // 只返回有余额的资产
                        if free > Decimal::ZERO || locked > Decimal::ZERO {
                            Some(AccountBalance {
                                asset: b["asset"].as_str().unwrap_or("").to_string(),
                                free,
                                locked,
                            })
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        Ok(balances)
    }

    /// 查询合约持仓
    async fn get_futures_positions(&self) -> Result<Vec<Position>> {
        // 注意：合约 API 端点不同，这里用现货测试网可能不支持
        // 实际合约需要用 fapi/v2/positionRisk
        let data = self.signed_get("fapi/v2/positionRisk", "").await;

        match data {
            Ok(data) => {
                let positions = data
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|p| {
                                let qty = Self::parse_decimal(&p["positionAmt"]);
                                // 只返回有持仓的
                                if qty.abs() > Decimal::ZERO {
                                    Some(Position {
                                        symbol: p["symbol"].as_str().unwrap_or("").to_string(),
                                        side: if qty > Decimal::ZERO { "LONG" } else { "SHORT" }.to_string(),
                                        quantity: qty.abs(),
                                        entry_price: Self::parse_decimal(&p["entryPrice"]),
                                        mark_price: Self::parse_decimal(&p["markPrice"]),
                                        unrealized_pnl: Self::parse_decimal(&p["unRealizedProfit"]),
                                        leverage: p["leverage"].as_str()
                                            .and_then(|s| s.parse().ok())
                                            .unwrap_or(1),
                                        margin_type: p["marginType"].as_str().unwrap_or("cross").to_string(),
                                    })
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                Ok(positions)
            }
            Err(_) => {
                // 现货测试网不支持合约，返回空
                Ok(vec![])
            }
        }
    }

    /// 查询单个订单状态
    async fn get_order(&self, symbol: &str, order_id: &str) -> Result<Option<ExchangeOrder>> {
        let params = format!("symbol={}&orderId={}", symbol.to_uppercase(), order_id);
        let data = self.signed_get("api/v3/order", &params).await;

        match data {
            Ok(data) => Ok(Some(Self::parse_order(&data))),
            Err(e) => {
                // 订单不存在返回 None
                if e.to_string().contains("-2013") {
                    Ok(None)
                } else {
                    Err(e)
                }
            }
        }
    }

    /// 查询未完成订单
    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<ExchangeOrder>> {
        let params = symbol
            .map(|s| format!("symbol={}", s.to_uppercase()))
            .unwrap_or_default();

        let data = self.signed_get("api/v3/openOrders", &params).await?;

        let orders = data
            .as_array()
            .map(|arr| arr.iter().map(Self::parse_order).collect())
            .unwrap_or_default();

        Ok(orders)
    }

    /// 撤销订单
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<CancelOrderResult> {
        let params = format!("symbol={}&orderId={}", symbol.to_uppercase(), order_id);

        match self.signed_delete("api/v3/order", &params).await {
            Ok(_) => Ok(CancelOrderResult {
                order_id: order_id.to_string(),
                symbol: symbol.to_string(),
                success: true,
                error: None,
            }),
            Err(e) => Ok(CancelOrderResult {
                order_id: order_id.to_string(),
                symbol: symbol.to_string(),
                success: false,
                error: Some(e.to_string()),
            }),
        }
    }

    /// 撤销某交易对所有订单
    async fn cancel_all_orders(&self, symbol: &str) -> Result<Vec<CancelOrderResult>> {
        let params = format!("symbol={}", symbol.to_uppercase());

        match self.signed_delete("api/v3/openOrders", &params).await {
            Ok(data) => {
                let results = data
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .map(|o| CancelOrderResult {
                                order_id: o["orderId"].to_string(),
                                symbol: symbol.to_string(),
                                success: true,
                                error: None,
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                Ok(results)
            }
            Err(e) => Ok(vec![CancelOrderResult {
                order_id: "all".to_string(),
                symbol: symbol.to_string(),
                success: false,
                error: Some(e.to_string()),
            }]),
        }
    }
}

impl BinanceQueryAdapter {
    /// 解析订单数据
    fn parse_order(data: &serde_json::Value) -> ExchangeOrder {
        ExchangeOrder {
            order_id: data["orderId"].to_string().trim_matches('"').to_string(),
            client_order_id: data["clientOrderId"].as_str().map(|s| s.to_string()),
            symbol: data["symbol"].as_str().unwrap_or("").to_string(),
            side: data["side"].as_str().unwrap_or("").to_string(),
            order_type: data["type"].as_str().unwrap_or("").to_string(),
            status: ExchangeOrderStatus::from_str(data["status"].as_str().unwrap_or("")),
            price: Self::parse_decimal(&data["price"]),
            quantity: Self::parse_decimal(&data["origQty"]),
            executed_qty: Self::parse_decimal(&data["executedQty"]),
            avg_price: Self::parse_decimal(&data["avgPrice"]),
            created_at: data["time"].as_i64().unwrap_or(0),
            updated_at: data["updateTime"].as_i64().unwrap_or(0),
        }
    }
}
