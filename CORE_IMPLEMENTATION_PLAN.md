# 量化交易平台核心功能实施计划

**制定时间**: 2026-01-23
**目标**: 完成3个核心缺失功能，让系统真正运行起来

---

## 一、策略实施计划

### 1.1 现货策略（5个）

| 序号 | 策略名称 | 文件路径 | 核心逻辑 | 优先级 |
|------|---------|---------|---------|--------|
| 1 | 网格策略 | `spot/grid.rs` | 价格区间网格，上涨卖出，下跌买入 | 🔴 最高 |
| 2 | 均值回归策略 | `spot/mean.rs` | 移动平均+标准差，偏离均值时反向交易 | 🔴 最高 |
| 3 | MACD策略 | `spot/macd.rs` | MACD指标，金叉买入，死叉卖出 | 🟡 高 |
| 4 | 布林带策略 | `spot/bollinger.rs` | 布林带上下轨，突破交易 | 🟡 高 |
| 5 | RSI策略 | `spot/rsi.rs` | RSI超买超卖，逆势交易 | 🟢 中 |

---

### 1.2 合约策略（10个）

| 序号 | 策略名称 | 文件路径 | 核心逻辑 | 优先级 |
|------|---------|---------|---------|--------|
| 1 | 合约网格策略 | `futures/grid.rs` | 双向网格，支持杠杆 | 🔴 最高 |
| 2 | 合约均值回归 | `futures/mean.rs` | 均值回归+杠杆 | 🔴 最高 |
| 3 | 资金费率套利 | `futures/funding_arb.rs` | 正负费率套利 | 🔴 最高 |
| 4 | 趋势跟踪策略 | `futures/trend_following.rs` | 突破+移动止损 | 🟡 高 |
| 5 | 动量策略 | `futures/momentum.rs` | 价格动量+成交量确认 | 🟡 高 |
| 6 | 反转策略 | `futures/reversal.rs` | 超买超卖反转 | 🟡 高 |
| 7 | 套利策略 | `futures/arbitrage.rs` | 期现套利、跨期套利 | 🟡 高 |
| 8 | 波动率策略 | `futures/volatility.rs` | ATR波动率交易 | 🟢 中 |
| 9 | 突破策略 | `futures/breakout.rs` | 关键位突破 | 🟢 中 |
| 10 | 对冲策略 | `futures/hedging.rs` | 现货对冲、Delta中性 | 🟢 中 |

---

### 1.3 跨平台套利策略

| 序号 | 策略名称 | 文件路径 | 核心逻辑 | 优先级 |
|------|---------|---------|---------|--------|
| 1 | 跨交易所套利 | `arbitrage/cross_exchange.rs` | 币安vs其他交易所价差套利 | 🔴 最高 |
| 2 | 三角套利 | `arbitrage/triangular.rs` | BTC/USDT, ETH/USDT, ETH/BTC | 🟡 高 |
| 3 | 期现套利 | `arbitrage/spot_futures.rs` | 现货vs合约价差 | 🟡 高 |

---

## 二、策略实现技术规范

### 2.1 统一接口

所有策略必须实现 `StrategyExecutorPort` trait：

```rust
pub trait StrategyExecutorPort: Send + Sync {
    /// 执行策略计算
    fn execute(&self, request: &ExecutionRequest) -> Result<ExecutionResult>;

    /// 重置策略状态
    fn reset(&self) -> Result<()>;

    /// 获取状态快照
    fn state_snapshot(&self) -> Result<serde_json::Value>;
}
```

### 2.2 策略结构

```rust
pub struct XxxStrategy {
    // 元数据
    instance_id: Uuid,
    symbol: String,

    // 配置（不可变）
    config: XxxConfig,

    // 状态（可变，使用RwLock）
    state: Arc<RwLock<XxxState>>,
}
```

### 2.3 配置结构

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XxxConfig {
    // 策略参数
    pub param1: Decimal,
    pub param2: u32,
    // ...
}
```

### 2.4 状态结构

```rust
#[derive(Debug, Clone)]
pub struct XxxState {
    // 历史数据
    pub price_history: VecDeque<Decimal>,

    // 指标缓存
    pub indicator_cache: HashMap<String, Decimal>,

    // 上次信号
    pub last_signal: Option<Signal>,
}
```

---

## 三、币安真实下单实施计划

### 3.1 功能清单

**文件**: `services/trading-engine/src/infrastructure/execution/binance_execution.rs`

```rust
pub struct BinanceExecution {
    api_key: String,
    api_secret: String,
    base_url: String,
    client: reqwest::Client,
}

impl BinanceExecution {
    // 1. 签名相关
    fn generate_signature(&self, query_string: &str) -> String;
    fn build_headers(&self) -> HeaderMap;

    // 2. 下单接口
    async fn place_market_order(&self, symbol: &str, side: OrderSide, quantity: Decimal) -> Result<Order>;
    async fn place_limit_order(&self, symbol: &str, side: OrderSide, price: Decimal, quantity: Decimal) -> Result<Order>;
    async fn place_stop_loss_order(&self, symbol: &str, side: OrderSide, stop_price: Decimal, quantity: Decimal) -> Result<Order>;

    // 3. 订单管理
    async fn query_order(&self, symbol: &str, order_id: i64) -> Result<Order>;
    async fn cancel_order(&self, symbol: &str, order_id: i64) -> Result<()>;
    async fn cancel_all_orders(&self, symbol: &str) -> Result<Vec<i64>>;

    // 4. 账户查询
    async fn get_account_balance(&self) -> Result<Vec<Balance>>;
    async fn get_positions(&self) -> Result<Vec<Position>>;

    // 5. 错误处理
    fn handle_api_error(&self, status: StatusCode, body: &str) -> TradingError;
    async fn retry_with_backoff<F, T>(&self, f: F) -> Result<T>;
}
```

### 3.2 API端点

```
现货API:
├─ POST /api/v3/order              # 下单
├─ GET  /api/v3/order              # 查询订单
├─ DELETE /api/v3/order            # 撤单
├─ GET  /api/v3/account            # 账户信息
└─ GET  /api/v3/myTrades           # 交易历史

合约API:
├─ POST /fapi/v1/order             # 下单
├─ GET  /fapi/v1/order             # 查询订单
├─ DELETE /fapi/v1/order           # 撤单
├─ GET  /fapi/v2/account           # 账户信息
└─ GET  /fapi/v1/positionRisk      # 持仓信息
```

### 3.3 签名算法

```rust
// HMAC-SHA256签名
fn generate_signature(secret: &str, query_string: &str) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(query_string.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}
```

### 3.4 限流控制

```rust
// 权重管理
pub struct RateLimiter {
    // 每分钟权重限制
    weight_per_minute: u32,
    // 当前权重
    current_weight: Arc<RwLock<u32>>,
    // 重置时间
    reset_at: Arc<RwLock<Instant>>,
}

impl RateLimiter {
    async fn check_and_consume(&self, weight: u32) -> Result<()>;
    async fn reset_if_needed(&self);
}
```

---

## 四、策略调度器实施计划

### 4.1 架构设计

```
┌─────────────────────────────────────────────────────────┐
│                  Strategy Scheduler                      │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐      ┌──────────────┐                │
│  │ Kafka        │      │ Strategy     │                │
│  │ Consumer     │─────▶│ Registry     │                │
│  │ (market-     │      │              │                │
│  │  events)     │      └──────────────┘                │
│  └──────────────┘              │                        │
│                                 │                        │
│                                 ▼                        │
│                        ┌──────────────┐                 │
│                        │ Strategy     │                 │
│                        │ Executor     │                 │
│                        └──────────────┘                 │
│                                 │                        │
│                                 ▼                        │
│                        ┌──────────────┐                 │
│                        │ Signal       │                 │
│                        │ Aggregator   │                 │
│                        └──────────────┘                 │
│                                 │                        │
│                                 ▼                        │
│  ┌──────────────┐      ┌──────────────┐                │
│  │ Kafka        │◀─────│ Signal       │                │
│  │ Producer     │      │ Publisher    │                │
│  │ (strategy-   │      │              │                │
│  │  signals)    │      └──────────────┘                │
│  └──────────────┘                                       │
│                                                          │
└─────────────────────────────────────────────────────────┘
```

### 4.2 核心组件

**文件**: `services/strategy-engine/src/application/scheduler/`

```rust
// 1. 策略调度器
pub struct StrategyScheduler {
    // 策略注册表
    registry: Arc<StrategyRegistry>,

    // Kafka消费者
    market_consumer: Arc<KafkaConsumer>,

    // Kafka生产者
    signal_producer: Arc<KafkaProducer>,

    // 配置
    config: SchedulerConfig,
}

impl StrategyScheduler {
    // 启动调度器
    pub async fn run(&self) -> Result<()>;

    // 处理行情事件
    async fn handle_market_event(&self, event: MarketEvent) -> Result<()>;

    // 路由到策略
    async fn route_to_strategies(&self, event: &MarketEvent) -> Vec<(Uuid, ExecutionRequest)>;

    // 执行策略
    async fn execute_strategies(&self, requests: Vec<(Uuid, ExecutionRequest)>) -> Vec<ExecutionResult>;

    // 发布信号
    async fn publish_signals(&self, results: Vec<ExecutionResult>) -> Result<()>;
}

// 2. 策略加载器
pub struct StrategyLoader {
    registry: Arc<StrategyRegistry>,
    db_pool: Arc<PgPool>,
}

impl StrategyLoader {
    // 从数据库加载策略配置
    pub async fn load_strategies(&self) -> Result<Vec<StrategyConfig>>;

    // 创建策略实例
    pub async fn create_strategy(&self, config: &StrategyConfig) -> Result<Arc<StrategyHandle>>;

    // 注册策略
    pub async fn register_strategy(&self, handle: Arc<StrategyHandle>) -> Result<Uuid>;
}

// 3. 策略生命周期管理器
pub struct StrategyLifecycleManager {
    registry: Arc<StrategyRegistry>,
}

impl StrategyLifecycleManager {
    // 启动策略
    pub async fn start_strategy(&self, instance_id: Uuid) -> Result<()>;

    // 停止策略
    pub async fn stop_strategy(&self, instance_id: Uuid) -> Result<()>;

    // 暂停策略
    pub async fn pause_strategy(&self, instance_id: Uuid) -> Result<()>;

    // 恢复策略
    pub async fn resume_strategy(&self, instance_id: Uuid) -> Result<()>;

    // 健康检查
    pub async fn health_check(&self) -> Result<Vec<StrategyHealth>>;
}
```

### 4.3 数据流

```
1. Market Data Service
   ↓ Kafka: market-events

2. Strategy Scheduler (Consumer)
   ↓ 消费行情事件

3. Strategy Registry
   ↓ 路由到对应策略

4. Strategy Executor
   ↓ 执行策略计算

5. Signal Aggregator
   ↓ 聚合信号

6. Signal Publisher
   ↓ Kafka: strategy-signals

7. Trading Engine (Consumer)
   ↓ 消费信号并下单
```

---

## 五、实施顺序

### Phase 1: 现货策略（3-4天）

**Day 1-2**: 核心策略
- ✅ 网格策略（grid.rs）
- ✅ 均值回归策略（mean.rs）

**Day 3**: 技术指标策略
- ✅ MACD策略（macd.rs）
- ✅ 布林带策略（bollinger.rs）

**Day 4**: RSI策略
- ✅ RSI策略（rsi.rs）

---

### Phase 2: 合约策略（4-5天）

**Day 5-6**: 核心合约策略
- ✅ 合约网格策略
- ✅ 合约均值回归
- ✅ 资金费率套利

**Day 7-8**: 趋势和动量策略
- ✅ 趋势跟踪策略
- ✅ 动量策略
- ✅ 反转策略

**Day 9**: 套利和波动率策略
- ✅ 套利策略
- ✅ 波动率策略
- ✅ 突破策略
- ✅ 对冲策略

---

### Phase 3: 跨平台套利（2天）

**Day 10-11**: 套利策略
- ✅ 跨交易所套利
- ✅ 三角套利
- ✅ 期现套利

---

### Phase 4: 币安真实下单（2-3天）

**Day 12-13**: 下单功能
- ✅ HMAC-SHA256签名
- ✅ 现货下单API
- ✅ 合约下单API
- ✅ 订单管理
- ✅ 账户查询

**Day 14**: 错误处理和限流
- ✅ 错误处理
- ✅ 重试机制
- ✅ 限流控制

---

### Phase 5: 策略调度器（2-3天）

**Day 15-16**: 调度器核心
- ✅ Kafka消费者
- ✅ 策略路由
- ✅ 信号聚合
- ✅ Kafka生产者

**Day 17**: 生命周期管理
- ✅ 策略加载器
- ✅ 生命周期管理器
- ✅ 健康检查

---

### Phase 6: 集成测试（2天）

**Day 18-19**: 端到端测试
- ✅ 行情→策略→下单 完整链路
- ✅ 修复发现的问题
- ✅ 性能测试

---

## 六、技术要点

### 6.1 策略实现要点

1. **状态管理**
   - 使用 `Arc<RwLock<State>>` 实现内部可变性
   - 避免死锁（读写锁顺序一致）

2. **错误处理**
   - 禁止 `unwrap/expect/panic!`
   - 使用 `Result<T, anyhow::Error>`
   - 记录详细错误日志

3. **性能优化**
   - 使用 `VecDeque` 存储历史数据（固定大小）
   - 缓存计算结果
   - 避免重复计算

4. **测试覆盖**
   - 单元测试（每个策略）
   - 集成测试（策略+调度器）
   - 回测验证

### 6.2 下单实现要点

1. **安全性**
   - API密钥加密存储
   - 签名算法正确实现
   - HTTPS通信

2. **可靠性**
   - 指数退避重试
   - 幂等性保证
   - 超时处理

3. **限流**
   - 权重管理
   - 请求队列
   - 优先级调度

### 6.3 调度器实现要点

1. **并发控制**
   - 策略并发执行
   - 信号聚合去重
   - 背压处理

2. **容错性**
   - 策略失败隔离
   - 自动重启
   - 降级策略

3. **监控**
   - 执行延迟监控
   - 信号生成统计
   - 错误率告警

---

## 七、预期成果

完成后系统将具备：

1. ✅ **18个策略**
   - 5个现货策略
   - 10个合约策略
   - 3个跨平台套利策略

2. ✅ **真实交易能力**
   - 币安现货下单
   - 币安合约下单
   - 订单管理

3. ✅ **自动化运行**
   - 策略自动调度
   - 信号自动生成
   - 订单自动执行

4. ✅ **完整数据流**
   ```
   行情采集 → 策略计算 → 信号生成 → 订单执行 → 成交回报
   ```

---

## 八、风险控制

1. **测试网先行**
   - 所有策略先在币安测试网验证
   - 小资金实盘测试
   - 逐步放大规模

2. **风控限制**
   - 单笔订单限额
   - 日内交易次数限制
   - 最大持仓限制

3. **监控告警**
   - 异常交易告警
   - 策略失败告警
   - 资金变动告警

---

**制定人**: AI开发团队
**预计完成时间**: 19天
**下次更新**: 完成Phase 1后
