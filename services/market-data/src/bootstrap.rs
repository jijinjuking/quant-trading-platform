//! # 依赖注入模块 (Bootstrap)
//!
//! 唯一允许组装 Adapter 的地方。
//!
//! ## 规则
//! - ✅ 使用 Arc<T> 共享 Adapter
//! - ✅ 在此处 new Adapter
//! - ❌ 不在 service 内 new adapter

use std::sync::Arc;

use tracing::info;

use crate::application::MarketDataService;
use crate::infrastructure::exchange::BinanceWebSocket;
use crate::infrastructure::messaging::KafkaProducer;
use crate::infrastructure::storage::ClickHouseStorage;
use crate::state::MarketDataConfig;

/// 构建行情数据服务
///
/// 完成依赖注入，返回可运行的服务实例。
pub fn build(
    config: MarketDataConfig,
) -> anyhow::Result<MarketDataService<Arc<BinanceWebSocket>, Arc<KafkaProducer>, Arc<ClickHouseStorage>>> {
    // 创建 Adapter（只在这里 new）
    let exchange = Arc::new(BinanceWebSocket::new(config.ws_url, config.proxy_url));
    let message = Arc::new(KafkaProducer::new(config.kafka_brokers, config.kafka_topic)?);

    // 创建存储（如果启用）
    let storage = if config.storage_enabled {
        match ClickHouseStorage::new(
            &config.clickhouse_url,
            &config.clickhouse_database,
            &config.clickhouse_table,
        ) {
            Ok(s) => {
                info!("ClickHouse 存储已启用");
                Some(Arc::new(s))
            }
            Err(e) => {
                tracing::warn!("ClickHouse 存储初始化失败，将禁用存储: {}", e);
                None
            }
        }
    } else {
        info!("ClickHouse 存储已禁用");
        None
    };

    // 注入到 Service
    Ok(MarketDataService::new(exchange, message, storage))
}
