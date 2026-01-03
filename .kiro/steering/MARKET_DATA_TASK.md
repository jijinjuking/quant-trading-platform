# 📋 行情数据服务开发任务书

> **任务类型**: 行情采集实现
> **负责服务**: `market-data` (8082)
> **验收人**: Kiro（架构师）
> **必读文档**: `TEAM_DEVELOPMENT_GUIDE.md`
> **优先级**: 🔴 最高（数据源，整个链路依赖它）

---

## 一、任务概述

实现真实的币安 WebSocket 行情采集，将行情数据标准化后发送到 Kafka，供 trading-engine 消费。

```
币安 WebSocket → market-data → Kafka (market-events) → trading-engine
```

---

## 二、当前状态

```
services/market-data/src/
├── main.rs                    # ✅ 启动入口
├── lib.rs
├── state.rs
├── bootstrap.rs               # ⚠️ 需要完善
│
├── application/
│   └── market_data_service.rs # ⚠️ 骨架
│
├── domain/
│   └── port/
│       ├── market_exchange_port.rs  # ⚠️ 骨架
│       └── message_port.rs          # ⚠️ 骨架
│
└── infrastructure/
    ├── exchange/
    │   └── binance_ws.rs      # ❌ 需要实现
    └── messaging/
        └── kafka_producer.rs  # ⚠️ 骨架
```

---

## 三、待开发任务清单

### 任务 M1: 实现币安 WebSocket 连接

**文件**: `services/market-data/src/infrastructure/exchange/binance_ws.rs`

**需求**:
- 连接币安 WebSocket（支持代理）
- 订阅 Trade/AggTrade 数据流
- 解析币安消息格式
- 转换为 `MarketEvent`
- 支持断线重连

**接口设计**:
```rust
use async_trait::async_trait;
use anyhow::Result;
use shared::event::market_event::MarketEvent;
use tokio::sync::mpsc;

/// 币安 WebSocket 客户端
pub struct BinanceWsClient {
    /// WebSocket URL
    ws_url: String,
    /// 代理地址（可选）
    proxy: Option<String>,
    /// 订阅的交易对
    symbols: Vec<String>,
    /// 事件发送通道
    event_tx: mpsc::Sender<MarketEvent>,
}

impl BinanceWsClient {
    /// 创建客户端
    pub fn new(
        ws_url: String,
        proxy: Option<String>,
        symbols: Vec<String>,
        event_tx: mpsc::Sender<MarketEvent>,
    ) -> Self {
        Self { ws_url, proxy, symbols, event_tx }
    }

    /// 启动连接（阻塞，内部循环）
    pub async fn run(&self) -> Result<()> {
        loop {
            match self.connect_and_subscribe().await {
                Ok(_) => {
                    tracing::info!("WebSocket 连接正常关闭，准备重连...");
                }
                Err(e) => {
                    tracing::error!("WebSocket 错误: {}, 5秒后重连...", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    /// 连接并订阅
    async fn connect_and_subscribe(&self) -> Result<()> {
        // 1. 建立 WebSocket 连接（支持代理）
        // 2. 发送订阅消息
        // 3. 循环读取消息
        // 4. 解析并转换为 MarketEvent
        // 5. 发送到 channel
        todo!()
    }

    /// 解析币安 Trade 消息
    fn parse_trade(&self, msg: &str) -> Result<MarketEvent> {
        // 解析 JSON，转换为 MarketEvent
        todo!()
    }
}
```

**币安消息格式参考**:
```json
// Trade 消息
{
  "e": "trade",
  "E": 1672515782136,
  "s": "BTCUSDT",
  "t": 12345,
  "p": "16500.00",
  "q": "0.001",
  "b": 88,
  "a": 50,
  "T": 1672515782136,
  "m": true,
  "M": true
}

// AggTrade 消息
{
  "e": "aggTrade",
  "E": 1672515782136,
  "s": "BTCUSDT",
  "a": 12345,
  "p": "16500.00",
  "q": "0.001",
  "f": 100,
  "l": 105,
  "T": 1672515782136,
  "m": true,
  "M": true
}
```

**需要添加的依赖** (Cargo.toml):
```toml
tokio-tungstenite = { version = "0.21", features = ["native-tls"] }
futures-util = "0.3"
url = "2"
```

---

### 任务 M2: 实现 Kafka 生产者

**文件**: `services/market-data/src/infrastructure/messaging/kafka_producer.rs`

**需求**:
- 连接 Kafka
- 发送 `MarketEvent` 到 `market-events` topic
- 序列化为 JSON

**接口设计**:
```rust
use async_trait::async_trait;
use anyhow::Result;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::ClientConfig;
use shared::event::market_event::MarketEvent;
use crate::domain::port::message_port::MessagePort;

/// Kafka 生产者
pub struct KafkaMarketProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaMarketProducer {
    /// 创建生产者
    pub fn new(brokers: &str, topic: &str) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("message.timeout.ms", "5000")
            .create()
            .context("创建 Kafka 生产者失败")?;

        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }
}

#[async_trait]
impl MessagePort for KafkaMarketProducer {
    async fn send_market_event(&self, event: &MarketEvent) -> Result<()> {
        let payload = serde_json::to_string(event)
            .context("序列化 MarketEvent 失败")?;

        let record = FutureRecord::to(&self.topic)
            .key(&event.symbol)
            .payload(&payload);

        self.producer
            .send(record, std::time::Duration::from_secs(5))
            .await
            .map_err(|(e, _)| anyhow::anyhow!("发送 Kafka 消息失败: {}", e))?;

        Ok(())
    }
}
```

**需要添加的依赖** (Cargo.toml):
```toml
rdkafka = { version = "0.36", features = ["cmake-build"] }
```

---

### 任务 M3: 完善 Domain Port

**文件**: `services/market-data/src/domain/port/market_exchange_port.rs`

```rust
use async_trait::async_trait;
use anyhow::Result;

/// 交易所行情端口
#[async_trait]
pub trait MarketExchangePort: Send + Sync {
    /// 启动行情采集（阻塞）
    async fn start(&self) -> Result<()>;
    
    /// 停止行情采集
    async fn stop(&self) -> Result<()>;
}
```

**文件**: `services/market-data/src/domain/port/message_port.rs`

```rust
use async_trait::async_trait;
use anyhow::Result;
use shared::event::market_event::MarketEvent;

/// 消息发送端口
#[async_trait]
pub trait MessagePort: Send + Sync {
    /// 发送行情事件
    async fn send_market_event(&self, event: &MarketEvent) -> Result<()>;
}
```

---

### 任务 M4: 完善 Application Service

**文件**: `services/market-data/src/application/market_data_service.rs`

```rust
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::mpsc;
use shared::event::market_event::MarketEvent;
use crate::domain::port::message_port::MessagePort;

/// 行情数据服务
pub struct MarketDataService<M: MessagePort> {
    message_port: Arc<M>,
}

impl<M: MessagePort> MarketDataService<M> {
    pub fn new(message_port: Arc<M>) -> Self {
        Self { message_port }
    }

    /// 启动行情转发（从 channel 读取，发送到 Kafka）
    pub async fn run(&self, mut rx: mpsc::Receiver<MarketEvent>) -> Result<()> {
        tracing::info!("MarketDataService 启动，等待行情数据...");

        while let Some(event) = rx.recv().await {
            if let Err(e) = self.message_port.send_market_event(&event).await {
                tracing::error!("发送行情事件失败: {}", e);
            } else {
                tracing::debug!("发送行情: {} @ {}", event.symbol, event.timestamp);
            }
        }

        Ok(())
    }
}
```

---

### 任务 M5: 完善 Bootstrap 和 Main

**文件**: `services/market-data/src/bootstrap.rs`

```rust
use std::sync::Arc;
use anyhow::Result;
use tokio::sync::mpsc;
use shared::event::market_event::MarketEvent;

use crate::infrastructure::exchange::binance_ws::BinanceWsClient;
use crate::infrastructure::messaging::kafka_producer::KafkaMarketProducer;
use crate::application::market_data_service::MarketDataService;

/// 创建行情采集组件
pub fn create_market_data_components() -> Result<(
    BinanceWsClient,
    MarketDataService<KafkaMarketProducer>,
    mpsc::Receiver<shared::event::market_event::MarketEvent>,
)> {
    // 从环境变量读取配置
    let ws_url = std::env::var("BINANCE_WS_URL")
        .unwrap_or_else(|_| "wss://stream.binance.com:9443/ws".to_string());
    let proxy = std::env::var("MARKET_DATA_PROXY").ok();
    let symbols: Vec<String> = std::env::var("MARKET_DATA_SYMBOLS")
        .unwrap_or_else(|_| "btcusdt,ethusdt".to_string())
        .split(',')
        .map(|s| s.trim().to_lowercase())
        .collect();
    let kafka_brokers = std::env::var("KAFKA_BROKERS")
        .unwrap_or_else(|_| "localhost:9092".to_string());
    let kafka_topic = std::env::var("KAFKA_MARKET_TOPIC")
        .unwrap_or_else(|_| "market-events".to_string());

    // 创建 channel
    let (tx, rx) = mpsc::channel::<MarketEvent>(10000);

    // 创建 WebSocket 客户端
    let ws_client = BinanceWsClient::new(ws_url, proxy, symbols, tx);

    // 创建 Kafka 生产者
    let kafka_producer = KafkaMarketProducer::new(&kafka_brokers, &kafka_topic)?;

    // 创建服务
    let service = MarketDataService::new(Arc::new(kafka_producer));

    Ok((ws_client, service, rx))
}
```

**文件**: `services/market-data/src/main.rs`

```rust
use anyhow::Result;
use tracing_subscriber;

mod application;
mod domain;
mod infrastructure;
mod bootstrap;
mod state;
mod lib;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载环境变量
    dotenv::dotenv().ok();

    tracing::info!("Market Data Service 启动中...");

    // 创建组件
    let (ws_client, service, rx) = bootstrap::create_market_data_components()?;

    // 启动两个任务
    let ws_handle = tokio::spawn(async move {
        if let Err(e) = ws_client.run().await {
            tracing::error!("WebSocket 任务异常: {}", e);
        }
    });

    let service_handle = tokio::spawn(async move {
        if let Err(e) = service.run(rx).await {
            tracing::error!("Service 任务异常: {}", e);
        }
    });

    // 等待任务
    tokio::select! {
        _ = ws_handle => tracing::warn!("WebSocket 任务结束"),
        _ = service_handle => tracing::warn!("Service 任务结束"),
    }

    Ok(())
}
```

---

## 四、环境变量

```env
# 币安 WebSocket
BINANCE_WS_URL=wss://stream.binance.com:9443/ws
MARKET_DATA_SYMBOLS=btcusdt,ethusdt

# 代理（国内必须）
MARKET_DATA_PROXY=http://127.0.0.1:4780

# Kafka
KAFKA_BROKERS=localhost:9092
KAFKA_MARKET_TOPIC=market-events

# 服务端口
MARKET_DATA_PORT=8082
```

---

## 五、禁止事项（红线）

| 禁止项 | 说明 |
|--------|------|
| ❌ `unwrap()` | 用 `?` 或 `ok_or()` |
| ❌ `expect()` | 同上 |
| ❌ `panic!()` | 用 `anyhow::bail!()` |
| ❌ `todo!()` | 返回 `Err` 或空实现 |
| ❌ 存储数据 | 只转发，不存储 |
| ❌ 业务判断 | 只做格式转换 |
| ❌ HTTP API | 本服务不需要 HTTP |
| ❌ 修改 shared/ | 不能改共享库 |

---

## 六、验收标准

### 6.1 编译检查
```bash
cargo check -p market-data
```
必须无错误通过。

### 6.2 功能验收
- [ ] 能连接币安 WebSocket（通过代理）
- [ ] 能收到 Trade 数据并打印日志
- [ ] 数据能正确转换为 `MarketEvent`
- [ ] 数据能发送到 Kafka `market-events` topic
- [ ] 断线能自动重连（5秒间隔）
- [ ] 日志输出清晰

### 6.3 代码检查
- [ ] 无禁止项违规
- [ ] 有完整文档注释
- [ ] 架构分层正确

---

## 七、参考文件

开发前请先阅读：

1. `shared/src/event/market_event.rs` - MarketEvent 定义
2. `services/trading-engine/src/infrastructure/messaging/market_event_consumer.rs` - 消费者参考

---

## 八、测试方法

```bash
# 1. 启动 Kafka
docker-compose up -d kafka

# 2. 启动 market-data
cargo run -p market-data

# 3. 查看 Kafka 消息
kafka-console-consumer --bootstrap-server localhost:9092 --topic market-events
```

---

**有问题先问，不要猜！**
