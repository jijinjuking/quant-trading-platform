# 📋 策略引擎 HTTP API 开发任务书

> **任务类型**: HTTP API 实现
> **负责服务**: `strategy-engine` (8083)
> **验收人**: Kiro（架构师）
> **必读文档**: `TEAM_DEVELOPMENT_GUIDE.md`
> **优先级**: 🔴 高（trading-engine 需要调用）

---

## 一、任务概述

为 strategy-engine 添加 HTTP API，供 trading-engine 调用进行策略评估。

```
trading-engine → HTTP POST /api/v1/strategy/evaluate → strategy-engine
                                                            ↓
                                                      返回 OrderIntent
```

> ⚠️ **重要**: Strategy 是被动服务，不主动消费行情，只提供 HTTP API 供 Trading Engine 调用

---

## 二、当前状态

```
services/strategy-engine/src/
├── domain/logic/
│   ├── strategy_trait.rs      # ✅ 统一策略 Trait
│   ├── strategy_registry.rs   # ✅ 策略注册表
│   ├── spot/                  # ✅ 现货策略
│   └── futures/               # ✅ 合约策略
│
├── interface/http/
│   ├── routes.rs              # ⚠️ 骨架
│   └── handlers/              # ❌ 空，需要实现
│
└── application/service/       # ⚠️ 需要完善
```

---

## 三、待开发任务清单

### 任务 S1: 创建 DTO 模型

**文件**: `services/strategy-engine/src/interface/http/dto/mod.rs` (新建)

```rust
pub mod evaluate;
```

**文件**: `services/strategy-engine/src/interface/http/dto/evaluate.rs` (新建)

```rust
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 策略评估请求
#[derive(Debug, Clone, Deserialize)]
pub struct EvaluateRequest {
    /// 策略实例 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 当前价格
    pub price: Decimal,
    /// 成交量
    pub quantity: Decimal,
    /// 时间戳（毫秒）
    pub timestamp: i64,
    /// 是否买方主动
    pub is_buyer_maker: bool,
}

/// 策略评估响应
#[derive(Debug, Clone, Serialize)]
pub struct EvaluateResponse {
    /// 是否生成交易意图
    pub has_intent: bool,
    /// 交易意图（可选）
    pub intent: Option<OrderIntentDto>,
}

/// 交易意图 DTO
#[derive(Debug, Clone, Serialize)]
pub struct OrderIntentDto {
    /// 意图 ID
    pub id: Uuid,
    /// 策略 ID
    pub strategy_id: Uuid,
    /// 交易对
    pub symbol: String,
    /// 方向: "buy" / "sell"
    pub side: String,
    /// 数量
    pub quantity: Decimal,
    /// 价格（限价单）
    pub price: Option<Decimal>,
    /// 订单类型: "market" / "limit"
    pub order_type: String,
    /// 置信度 (0.0 - 1.0)
    pub confidence: f64,
    /// 创建时间
    pub created_at: i64,
}

/// 策略列表响应
#[derive(Debug, Clone, Serialize)]
pub struct StrategyListResponse {
    pub strategies: Vec<StrategyInfoDto>,
}

/// 策略信息 DTO
#[derive(Debug, Clone, Serialize)]
pub struct StrategyInfoDto {
    pub instance_id: Uuid,
    pub strategy_type: String,
    pub market_type: String,
    pub symbol: String,
    pub is_active: bool,
}

/// 创建策略请求
#[derive(Debug, Clone, Deserialize)]
pub struct CreateStrategyRequest {
    /// 策略类型: "spot_grid", "spot_mean", "futures_grid", etc.
    pub strategy_type: String,
    /// 市场类型: "spot", "usdt_futures", "coin_futures"
    pub market_type: String,
    /// 交易对
    pub symbol: String,
    /// 策略配置（JSON）
    pub config: serde_json::Value,
}

/// 创建策略响应
#[derive(Debug, Clone, Serialize)]
pub struct CreateStrategyResponse {
    pub instance_id: Uuid,
    pub message: String,
}

/// 通用响应
#[derive(Debug, Clone, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}
```

---

### 任务 S2: 实现评估 Handler

**文件**: `services/strategy-engine/src/interface/http/handlers/evaluate.rs` (新建)

```rust
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::interface::http::dto::evaluate::{
    ApiResponse, EvaluateRequest, EvaluateResponse, OrderIntentDto,
};
use crate::state::AppState;

/// POST /api/v1/strategy/evaluate
/// 
/// 评估策略，根据行情生成交易意图
pub async fn evaluate_strategy(
    State(state): State<AppState>,
    Json(req): Json<EvaluateRequest>,
) -> Json<ApiResponse<EvaluateResponse>> {
    tracing::debug!("收到策略评估请求: {:?}", req);

    // 1. 从注册表获取策略实例
    let registry = state.strategy_registry.read().await;
    
    let strategy = match registry.get(&req.strategy_id) {
        Some(s) => s,
        None => {
            return Json(ApiResponse::err(format!(
                "策略实例不存在: {}",
                req.strategy_id
            )));
        }
    };

    // 2. 构造 MarketEvent
    let market_event = shared::event::market_event::MarketEvent {
        event_type: shared::event::market_event::MarketEventType::Trade,
        exchange: "binance".to_string(),
        symbol: req.symbol.clone(),
        timestamp: chrono::Utc::now(),
        data: shared::event::market_event::MarketEventData::Trade(
            shared::event::market_event::TradeData {
                trade_id: 0,
                price: req.price,
                quantity: req.quantity,
                buyer_order_id: 0,
                seller_order_id: 0,
                trade_time: req.timestamp,
                is_buyer_maker: req.is_buyer_maker,
            },
        ),
    };

    // 3. 调用策略评估
    let mut strategy = strategy.clone();
    let signal = strategy.on_market_event(&market_event);

    // 4. 转换为响应
    let response = match signal {
        Some(sig) => EvaluateResponse {
            has_intent: true,
            intent: Some(OrderIntentDto {
                id: Uuid::new_v4(),
                strategy_id: req.strategy_id,
                symbol: req.symbol,
                side: match sig.signal_type {
                    crate::domain::model::signal::SignalType::Buy => "buy".to_string(),
                    crate::domain::model::signal::SignalType::Sell => "sell".to_string(),
                    crate::domain::model::signal::SignalType::Hold => "hold".to_string(),
                },
                quantity: sig.quantity,
                price: sig.price,
                order_type: "limit".to_string(),
                confidence: sig.confidence,
                created_at: chrono::Utc::now().timestamp_millis(),
            }),
        },
        None => EvaluateResponse {
            has_intent: false,
            intent: None,
        },
    };

    Json(ApiResponse::ok(response))
}
```

---

### 任务 S3: 实现策略管理 Handler

**文件**: `services/strategy-engine/src/interface/http/handlers/strategies.rs` (新建)

```rust
use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::interface::http::dto::evaluate::{
    ApiResponse, CreateStrategyRequest, CreateStrategyResponse,
    StrategyInfoDto, StrategyListResponse,
};
use crate::state::AppState;

/// GET /api/v1/strategies
/// 
/// 获取所有策略实例列表
pub async fn list_strategies(
    State(state): State<AppState>,
) -> Json<ApiResponse<StrategyListResponse>> {
    let registry = state.strategy_registry.read().await;
    
    let strategies: Vec<StrategyInfoDto> = registry
        .iter()
        .map(|(id, strategy)| {
            let meta = strategy.meta();
            StrategyInfoDto {
                instance_id: *id,
                strategy_type: meta.strategy_type.clone(),
                market_type: format!("{:?}", meta.market_type),
                symbol: meta.symbol.clone(),
                is_active: meta.is_active,
            }
        })
        .collect();

    Json(ApiResponse::ok(StrategyListResponse { strategies }))
}

/// GET /api/v1/strategies/{id}
/// 
/// 获取单个策略实例信息
pub async fn get_strategy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<StrategyInfoDto>> {
    let registry = state.strategy_registry.read().await;
    
    match registry.get(&id) {
        Some(strategy) => {
            let meta = strategy.meta();
            Json(ApiResponse::ok(StrategyInfoDto {
                instance_id: id,
                strategy_type: meta.strategy_type.clone(),
                market_type: format!("{:?}", meta.market_type),
                symbol: meta.symbol.clone(),
                is_active: meta.is_active,
            }))
        }
        None => Json(ApiResponse::err(format!("策略实例不存在: {}", id))),
    }
}

/// POST /api/v1/strategies
/// 
/// 创建新策略实例
pub async fn create_strategy(
    State(state): State<AppState>,
    Json(req): Json<CreateStrategyRequest>,
) -> Json<ApiResponse<CreateStrategyResponse>> {
    tracing::info!("创建策略: type={}, symbol={}", req.strategy_type, req.symbol);

    // TODO: 根据 strategy_type 创建对应的策略实例
    // 这里需要根据 req.config 解析配置并创建策略
    
    let instance_id = Uuid::new_v4();
    
    // 暂时返回成功，实际需要实现策略创建逻辑
    Json(ApiResponse::ok(CreateStrategyResponse {
        instance_id,
        message: "策略创建成功（待实现）".to_string(),
    }))
}

/// POST /api/v1/strategies/{id}/start
/// 
/// 启动策略
pub async fn start_strategy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<String>> {
    let mut registry = state.strategy_registry.write().await;
    
    match registry.get_mut(&id) {
        Some(strategy) => {
            strategy.start();
            Json(ApiResponse::ok("策略已启动".to_string()))
        }
        None => Json(ApiResponse::err(format!("策略实例不存在: {}", id))),
    }
}

/// POST /api/v1/strategies/{id}/stop
/// 
/// 停止策略
pub async fn stop_strategy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<String>> {
    let mut registry = state.strategy_registry.write().await;
    
    match registry.get_mut(&id) {
        Some(strategy) => {
            strategy.stop();
            Json(ApiResponse::ok("策略已停止".to_string()))
        }
        None => Json(ApiResponse::err(format!("策略实例不存在: {}", id))),
    }
}

/// DELETE /api/v1/strategies/{id}
/// 
/// 删除策略实例
pub async fn delete_strategy(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<String>> {
    let mut registry = state.strategy_registry.write().await;
    
    match registry.remove(&id) {
        Some(_) => Json(ApiResponse::ok("策略已删除".to_string())),
        None => Json(ApiResponse::err(format!("策略实例不存在: {}", id))),
    }
}
```

---

### 任务 S4: 更新 Handlers mod.rs

**文件**: `services/strategy-engine/src/interface/http/handlers/mod.rs`

```rust
pub mod evaluate;
pub mod strategies;

pub use evaluate::*;
pub use strategies::*;
```

---

### 任务 S5: 更新路由

**文件**: `services/strategy-engine/src/interface/http/routes.rs`

```rust
use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::interface::http::handlers;
use crate::state::AppState;

/// 创建路由
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // 策略评估（核心 API）
        .route("/api/v1/strategy/evaluate", post(handlers::evaluate_strategy))
        
        // 策略管理
        .route("/api/v1/strategies", get(handlers::list_strategies))
        .route("/api/v1/strategies", post(handlers::create_strategy))
        .route("/api/v1/strategies/:id", get(handlers::get_strategy))
        .route("/api/v1/strategies/:id", delete(handlers::delete_strategy))
        .route("/api/v1/strategies/:id/start", post(handlers::start_strategy))
        .route("/api/v1/strategies/:id/stop", post(handlers::stop_strategy))
        
        // 健康检查
        .route("/health", get(health_check))
        
        .with_state(state)
}

/// 健康检查
async fn health_check() -> &'static str {
    "OK"
}
```

---

### 任务 S6: 更新 AppState

**文件**: `services/strategy-engine/src/state.rs`

```rust
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::domain::logic::strategy_trait::Strategy;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    /// 策略注册表（策略实例 ID -> 策略实例）
    pub strategy_registry: Arc<RwLock<HashMap<Uuid, Box<dyn Strategy>>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            strategy_registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### 任务 S7: 更新 Main

**文件**: `services/strategy-engine/src/main.rs`

```rust
use anyhow::Result;
use std::net::SocketAddr;
use tracing_subscriber;

mod application;
mod domain;
mod infrastructure;
mod interface;
mod bootstrap;
mod state;

use interface::http::routes::create_routes;
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载环境变量
    dotenv::dotenv().ok();

    tracing::info!("Strategy Engine 启动中...");

    // 创建应用状态
    let state = AppState::new();

    // 创建路由
    let app = create_routes(state);

    // 获取端口
    let port: u16 = std::env::var("STRATEGY_ENGINE_PORT")
        .unwrap_or_else(|_| "8083".to_string())
        .parse()
        .unwrap_or(8083);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Strategy Engine 监听: {}", addr);

    // 启动服务
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

---

## 四、API 接口汇总

| 方法 | 路径 | 说明 |
|------|------|------|
| POST | `/api/v1/strategy/evaluate` | ⭐ 策略评估（核心） |
| GET | `/api/v1/strategies` | 获取策略列表 |
| POST | `/api/v1/strategies` | 创建策略实例 |
| GET | `/api/v1/strategies/{id}` | 获取策略详情 |
| DELETE | `/api/v1/strategies/{id}` | 删除策略实例 |
| POST | `/api/v1/strategies/{id}/start` | 启动策略 |
| POST | `/api/v1/strategies/{id}/stop` | 停止策略 |
| GET | `/health` | 健康检查 |

---

## 五、环境变量

```env
STRATEGY_ENGINE_PORT=8083
```

---

## 六、禁止事项（红线）

| 禁止项 | 说明 |
|--------|------|
| ❌ `unwrap()` | 用 `?` 或 `ok_or()` |
| ❌ `expect()` | 同上 |
| ❌ `panic!()` | 用 `anyhow::bail!()` |
| ❌ 消费 Kafka | Strategy 不能消费行情 |
| ❌ 发送 Kafka | Strategy 不能发消息 |
| ❌ 直接下单 | Strategy 只返回意图 |
| ❌ 无限循环 | Strategy 是被动调用 |

---

## 七、验收标准

### 7.1 编译检查
```bash
cargo check -p strategy-engine
```

### 7.2 功能验收
- [ ] POST /api/v1/strategy/evaluate 能正常响应
- [ ] 策略 CRUD API 能正常工作
- [ ] 健康检查返回 OK
- [ ] 日志输出清晰

### 7.3 测试方法
```bash
# 启动服务
cargo run -p strategy-engine

# 测试健康检查
curl http://localhost:8083/health

# 测试策略列表
curl http://localhost:8083/api/v1/strategies

# 测试策略评估
curl -X POST http://localhost:8083/api/v1/strategy/evaluate \
  -H "Content-Type: application/json" \
  -d '{
    "strategy_id": "00000000-0000-0000-0000-000000000001",
    "symbol": "BTCUSDT",
    "price": "50000.00",
    "quantity": "0.001",
    "timestamp": 1704067200000,
    "is_buyer_maker": false
  }'
```

---

**有问题先问，不要猜！**
