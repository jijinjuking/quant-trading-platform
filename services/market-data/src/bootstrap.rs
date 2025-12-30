//! # 依赖注入模块 (Bootstrap)
//!
//! 唯一允许组装 Adapter 的地方。
//!
//! ## 规则
//! - ✅ 使用 Arc<T> 共享 Adapter
//! - ✅ 在此处 new Adapter
//! - ❌ 不在 service 内 new adapter

use std::sync::Arc;

use crate::state::MarketDataConfig;
use crate::application::MarketDataService;
use crate::infrastructure::exchange::BinanceWebSocket;
use crate::infrastructure::messaging::KafkaProducer;

/// 构建行情数据服务
///
/// 完成依赖注入，返回可运行的服务实例。
pub fn build() -> MarketDataService<Arc<BinanceWebSocket>, Arc<KafkaProducer>> {
    // 加载配置
    let config = MarketDataConfig::from_env();

    // 创建 Adapter（只在这里 new）
    let exchange = Arc::new(BinanceWebSocket::new(config.ws_url));
    let message = Arc::new(KafkaProducer::new(config.kafka_brokers, config.kafka_topic));

    // 注入到 Service
    MarketDataService::new(exchange, message)
}
