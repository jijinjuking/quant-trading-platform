//! # 币安 WebSocket 适配器 (Binance WebSocket Adapter)
//!
//! 实现 MarketExchangePort trait。

use async_trait::async_trait;
use shared::event::market_event::MarketEvent;
use crate::domain::port::MarketExchangePort;

/// 币安 WebSocket 适配器
pub struct BinanceWebSocket {
    #[allow(dead_code)]
    url: String,
    // TODO: WebSocket client
}

impl BinanceWebSocket {
    pub fn new(url: String) -> Self {
        Self { url }
    }
}

#[async_trait]
impl MarketExchangePort for BinanceWebSocket {
    async fn connect(&self) -> anyhow::Result<()> {
        // TODO: 建立 WebSocket 连接
        Ok(())
    }

    async fn subscribe_spot(&self, _symbols: Vec<String>) -> anyhow::Result<()> {
        // TODO: 订阅现货
        Ok(())
    }

    async fn subscribe_futures(&self, _symbols: Vec<String>) -> anyhow::Result<()> {
        // TODO: 订阅合约
        Ok(())
    }

    async fn next_event(&self) -> anyhow::Result<MarketEvent> {
        // TODO: 读取 WebSocket 消息并转换
        todo!("实现 WebSocket 消息读取")
    }
}
