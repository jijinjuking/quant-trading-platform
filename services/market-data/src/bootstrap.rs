//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use crate::infrastructure::exchange::binance_ws::BinanceWebSocket;
use crate::infrastructure::messaging::kafka_producer::KafkaProducer;
use crate::application::service::market_service::MarketService;

/// 创建行情服务实例
///
/// # 参数
/// - `ws_url`: WebSocket 连接地址
/// - `kafka_brokers`: Kafka broker 地址
#[allow(dead_code)]
pub fn create_market_service(
    ws_url: String,
    kafka_brokers: String,
) -> MarketService<BinanceWebSocket, KafkaProducer> {
    let exchange = BinanceWebSocket::new(ws_url);
    let messenger = KafkaProducer::new(kafka_brokers);
    MarketService::new(exchange, messenger)
}
