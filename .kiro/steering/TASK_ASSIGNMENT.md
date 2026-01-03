# 📋 任务分配表（2026-01-01）

> **架构师**: Kiro（负责验收）
> **规范文档**: `.kiro/steering/TEAM_DEVELOPMENT_GUIDE.md`（必读）

---

## 当前状态

| 服务 | 骨架 | 数据流 | 业务逻辑 | 状态 |
|------|------|--------|----------|------|
| market-data | ✅ | ⚠️ 骨架 | - | 需要真实 WebSocket |
| strategy-engine | ✅ | ✅ | ❌ 空 | 需要策略实现 |
| trading-engine | ✅ | ✅ | ⚠️ 风控有 | 需要真实执行 |
| risk-management | ✅ | - | ⚠️ 部分 | 需要完善 |

---

## 任务分配

### 🔵 AI-1: 行情数据工程师
**负责服务**: `market-data`

**任务清单**:
1. 实现真实币安 WebSocket 连接
2. 解析币安消息格式（Trade/AggTrade）
3. 转换为 `MarketEvent` 发送到 Kafka
4. 实现断线重连机制
5. 支持代理配置（`MARKET_DATA_PROXY`）

**修改范围**:
```
services/market-data/src/
├── infrastructure/exchange/binance_ws.rs  # 主要修改
├── infrastructure/messaging/kafka_producer.rs
├── application/market_data_service.rs
└── Cargo.toml  # 添加依赖
```

**需要添加的依赖**:
```toml
tokio-tungstenite = { version = "0.21", features = ["native-tls"] }
futures-util = "0.3"
url = "2"
```

**验收标准**:
- [ ] 能连接币安 WebSocket（通过代理）
- [ ] 能收到 Trade 数据并打印日志
- [ ] 数据能正确转换为 `MarketEvent`
- [ ] 数据能发送到 Kafka `market-events` topic
- [ ] 断线能自动重连
- [ ] 无 `unwrap/expect/todo!`

**禁止做**:
- ❌ 修改 `domain/port/` 中的 trait
- ❌ 修改 `shared/` 中的事件结构
- ❌ 添加 HTTP API
- ❌ 存储数据

---

### 🟢 AI-2: 策略工程师
**负责服务**: `strategy-engine`

**任务清单**:
1. 实现网格策略（`domain/logic/grid.rs`）
2. 实现均值回归策略（`domain/logic/mean.rs`）
3. 完善 `signal_generator.rs` 调用策略
4. 策略配置模型（`domain/model/strategy_config.rs`）

**修改范围**:
```
services/strategy-engine/src/
├── domain/logic/
│   ├── signal_generator.rs  # 修改：调用策略
│   ├── grid.rs              # 新建：网格策略
│   └── mean.rs              # 新建：均值回归
├── domain/model/
│   └── strategy_config.rs   # 新建：策略配置
└── domain/logic/mod.rs      # 追加模块导出
```

**策略实现规范**:
```rust
// domain/logic/grid.rs
use shared::event::market_event::MarketEvent;
use crate::domain::model::signal::Signal;

/// 网格策略配置
pub struct GridConfig {
    pub grid_count: u32,      // 网格数量
    pub upper_price: Decimal, // 上边界
    pub lower_price: Decimal, // 下边界
    pub quantity: Decimal,    // 每格数量
}

/// 网格策略状态
pub struct GridState {
    pub current_grid: i32,    // 当前所在网格
    pub last_price: Decimal,  // 上次价格
}

/// 计算网格信号
pub fn calculate_grid_signal(
    event: &MarketEvent,
    config: &GridConfig,
    state: &mut GridState,
) -> Option<Signal> {
    // 业务逻辑写这里
}
```

**验收标准**:
- [ ] 网格策略能根据价格变化生成买卖信号
- [ ] 均值回归策略能根据偏离度生成信号
- [ ] 信号能正确发送到 Kafka `trading.signals` topic
- [ ] 业务逻辑全部在 `domain/logic/`
- [ ] 无 `unwrap/expect/todo!`

**禁止做**:
- ❌ 在 `infrastructure/` 写业务逻辑
- ❌ 在 `application/` 写业务逻辑
- ❌ 直接下单
- ❌ 修改 `shared/` 中的事件结构

---

### 🟠 AI-3: 交易执行工程师
**负责服务**: `trading-engine`

**任务清单**:
1. 实现币安 REST API 下单（`infrastructure/execution/binance_execution.rs`）
2. 订单状态管理
3. 完善风控规则（`domain/logic/risk_rules.rs`）
4. 持仓同步

**修改范围**:
```
services/trading-engine/src/
├── infrastructure/execution/
│   ├── mod.rs
│   ├── noop_execution.rs    # 已有
│   └── binance_execution.rs # 新建：真实下单
├── domain/logic/
│   └── risk_rules.rs        # 完善风控规则
├── domain/model/
│   └── position.rs          # 新建：持仓模型
└── Cargo.toml               # 添加依赖
```

**需要添加的依赖**:
```toml
reqwest = { version = "0.11", features = ["json"] }
hmac = "0.12"
sha2 = "0.10"
hex = "0.4"
```

**币安下单实现规范**:
```rust
// infrastructure/execution/binance_execution.rs
pub struct BinanceExecution {
    api_key: String,
    secret_key: String,
    base_url: String,
    client: reqwest::Client,
}

#[async_trait]
impl ExecutionPort for BinanceExecution {
    async fn execute(&self, command: &ExecutionCommand) -> Result<()> {
        // 1. 构建请求参数
        // 2. 签名
        // 3. 发送请求
        // 4. 处理响应
    }
}
```

**验收标准**:
- [ ] 能调用币安 REST API 下单（测试网）
- [ ] 签名正确
- [ ] 错误处理完善
- [ ] 风控规则能正确拦截不合规信号
- [ ] 无 `unwrap/expect/todo!`

**禁止做**:
- ❌ 在 `infrastructure/` 写风控判断逻辑（只能调用 domain）
- ❌ 策略计算
- ❌ 行情采集

---

### 🔴 AI-4: 风控工程师
**负责服务**: `risk-management`

**任务清单**:
1. 完善杠杆检查（`domain/logic/leverage.rs`）
2. 完善回撤检查（`domain/logic/drawdown.rs`）
3. 添加持仓限制检查
4. 风控配置管理 API

**修改范围**:
```
services/risk-management/src/
├── domain/logic/
│   ├── leverage.rs          # 完善
│   ├── drawdown.rs          # 完善
│   └── position_limit.rs    # 新建
├── domain/model/
│   └── risk_profile.rs      # 完善
├── interface/http/handlers/
│   └── risk_config.rs       # 新建：配置 API
└── application/service/
    └── risk_check_service.rs # 完善
```

**验收标准**:
- [ ] 杠杆检查能正确计算并拦截
- [ ] 回撤检查能正确计算并拦截
- [ ] 持仓限制能正确检查
- [ ] 有 HTTP API 管理风控配置
- [ ] 无 `unwrap/expect/todo!`

---

## 环境配置

所有 AI 开始前，确保项目根目录有 `.env` 文件：

```env
# 代理（必须）
HTTP_PROXY=http://127.0.0.1:4780
HTTPS_PROXY=http://127.0.0.1:4780
MARKET_DATA_PROXY=http://127.0.0.1:4780

# Kafka
KAFKA_BROKERS=localhost:9092
KAFKA_MARKET_TOPIC=market-events
KAFKA_SIGNAL_TOPIC=trading.signals

# 数据库
DATABASE_URL=postgres://postgres:password@localhost:5432/trading

# 币安 API（测试网）
BINANCE_API_KEY=your_api_key
BINANCE_SECRET_KEY=your_secret_key
BINANCE_BASE_URL=https://testnet.binance.vision

# 行情
BINANCE_WS_URL=wss://stream.binance.com:9443/ws
MARKET_DATA_SYMBOLS=btcusdt,ethusdt
```

---

## 工作流程

1. **领取任务**: 确认自己负责的服务
2. **阅读规范**: 必读 `TEAM_DEVELOPMENT_GUIDE.md`
3. **开发**: 按规范实现
4. **自检**: 对照验收标准检查
5. **提交**: 通知架构师验收

---

## 验收流程

1. AI 完成任务后通知架构师
2. 架构师检查：
   - 编译是否通过
   - 架构分层是否正确
   - 是否有禁止项违规
   - 功能是否正常
3. 通过 → 合并
4. 不通过 → 打回修改

---

## 优先级

1. 🔵 **AI-1 行情数据** - 最高优先级（数据源）
2. 🟢 **AI-2 策略** - 高优先级（核心功能）
3. 🟠 **AI-3 交易执行** - 中优先级
4. 🔴 **AI-4 风控** - 中优先级

---

**开始干活！有问题先问架构师。**
