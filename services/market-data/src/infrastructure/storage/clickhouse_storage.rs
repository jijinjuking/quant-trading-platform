//! # ClickHouse 存储适配器 (ClickHouse Storage Adapter)
//!
//! 实现 MarketStoragePort，将行情数据存储到 ClickHouse。

use anyhow::{Context, Result};
use async_trait::async_trait;
use clickhouse::Client;
use serde::Serialize;
use tracing::{debug, error, info};

use crate::domain::port::MarketStoragePort;
use shared::event::market_event::{MarketEvent, MarketEventData};

/// ClickHouse 行情记录
#[derive(Debug, Clone, Serialize, clickhouse::Row)]
struct TradeRow {
    /// 交易所
    exchange: String,
    /// 交易对
    symbol: String,
    /// 成交 ID
    trade_id: String,
    /// 成交价格
    price: f64,
    /// 成交数量
    quantity: f64,
    /// 买方是否为 maker
    is_buyer_maker: bool,
    /// 事件时间（毫秒时间戳）
    event_time: i64,
    /// 插入时间
    insert_time: i64,
}

/// ClickHouse 存储适配器
pub struct ClickHouseStorage {
    client: Client,
    table_name: String,
}

impl ClickHouseStorage {
    /// 创建 ClickHouse 存储适配器
    ///
    /// # 参数
    /// - `url`: ClickHouse HTTP URL，如 `http://localhost:8123`
    /// - `database`: 数据库名
    /// - `table_name`: 表名
    pub fn new(url: &str, database: &str, table_name: &str) -> Result<Self> {
        let client = Client::default()
            .with_url(url)
            .with_database(database);

        info!(
            "ClickHouse 存储初始化: url={}, db={}, table={}",
            url, database, table_name
        );

        Ok(Self {
            client,
            table_name: table_name.to_string(),
        })
    }

    /// 从环境变量创建
    pub fn from_env() -> Result<Self> {
        let url = std::env::var("CLICKHOUSE_URL")
            .unwrap_or_else(|_| "http://localhost:8123".to_string());
        let database = std::env::var("CLICKHOUSE_DATABASE")
            .unwrap_or_else(|_| "market_data".to_string());
        let table_name = std::env::var("CLICKHOUSE_TABLE")
            .unwrap_or_else(|_| "trades".to_string());

        Self::new(&url, &database, &table_name)
    }

    /// 初始化表结构（如果不存在）
    pub async fn init_table(&self) -> Result<()> {
        let create_sql = format!(
            r#"
            CREATE TABLE IF NOT EXISTS {} (
                exchange String,
                symbol String,
                trade_id String,
                price Float64,
                quantity Float64,
                is_buyer_maker UInt8,
                event_time DateTime64(3),
                insert_time DateTime64(3)
            ) ENGINE = MergeTree()
            PARTITION BY toYYYYMMDD(event_time)
            ORDER BY (symbol, event_time, trade_id)
            "#,
            self.table_name
        );

        self.client
            .query(&create_sql)
            .execute()
            .await
            .context("创建 ClickHouse 表失败")?;

        info!("ClickHouse 表 {} 初始化完成", self.table_name);
        Ok(())
    }

    /// 将 MarketEvent 转换为 TradeRow
    fn event_to_row(&self, event: &MarketEvent) -> Option<TradeRow> {
        match &event.data {
            MarketEventData::Trade(trade) => {
                let price = trade.price.to_string().parse::<f64>().ok()?;
                let quantity = trade.quantity.to_string().parse::<f64>().ok()?;

                Some(TradeRow {
                    exchange: event.exchange.clone(),
                    symbol: event.symbol.clone(),
                    trade_id: trade.trade_id.clone(),
                    price,
                    quantity,
                    is_buyer_maker: trade.is_buyer_maker,
                    event_time: event.timestamp.timestamp_millis(),
                    insert_time: chrono::Utc::now().timestamp_millis(),
                })
            }
            _ => None,
        }
    }
}

#[async_trait]
impl MarketStoragePort for ClickHouseStorage {
    /// 存储单条行情事件
    async fn save_event(&self, event: &MarketEvent) -> Result<()> {
        let row = match self.event_to_row(event) {
            Some(r) => r,
            None => {
                debug!("跳过非 Trade 事件");
                return Ok(());
            }
        };

        let mut insert = self.client.insert(&self.table_name)?;
        insert.write(&row).await.context("写入行数据失败")?;
        insert.end().await.context("提交插入失败")?;

        debug!("存储行情: {} @ {}", row.symbol, row.price);
        Ok(())
    }

    /// 批量存储行情事件
    async fn save_events(&self, events: &[MarketEvent]) -> Result<()> {
        if events.is_empty() {
            return Ok(());
        }

        let rows: Vec<TradeRow> = events
            .iter()
            .filter_map(|e| self.event_to_row(e))
            .collect();

        if rows.is_empty() {
            return Ok(());
        }

        let mut insert = self.client.insert(&self.table_name)?;
        for row in &rows {
            insert.write(row).await.context("写入行数据失败")?;
        }
        insert.end().await.context("提交批量插入失败")?;

        debug!("批量存储 {} 条行情", rows.len());
        Ok(())
    }
}
