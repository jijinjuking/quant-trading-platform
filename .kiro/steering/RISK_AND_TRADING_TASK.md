# 📋 风控与交易执行开发任务书

> **任务类型**: 风控完善 + 交易执行完善
> **验收人**: Kiro（架构师）
> **必读文档**: `TEAM_DEVELOPMENT_GUIDE.md`
> **优先级**: 🔴 高

---

## 一、任务概述

完善风控服务（risk-management）和交易执行服务（trading-engine），使整个交易链路可以真正跑通：

```
strategy-engine → Kafka → trading-engine → risk-management → 币安API
     信号              消费信号        风控检查         真实下单
```

---

## 二、当前状态

### 2.1 risk-management (8085) 风控服务

```
services/risk-management/src/
├── domain/
│   ├── logic/
│   │   ├── leverage.rs      # ⚠️ 骨架，只有简单比较
│   │   └── drawdown.rs      # ⚠️ 骨架，只有简单比较
│   ├── model/
│   │   └── risk_profile.rs  # ⚠️ 基础模型
│   └── service/
│       └── risk_evaluator.rs # ⚠️ 骨架，直接返回 Approved
├── application/service/
│   └── risk_check_service.rs # ⚠️ 骨架
└── interface/http/
    └── handlers/             # ❌ 空，没有 API
```

### 2.2 trading-engine (8081) 交易执行服务

```
services/trading-engine/src/
├── domain/
│   ├── logic/
│   │   ├── risk_rules.rs        # ✅ 基础风控规则
│   │   └── execution_algo.rs    # ⚠️ 骨架
│   └── model/
│       ├── order.rs             # ⚠️ 基础模型
│       └── trade.rs             # ⚠️ 基础模型
├── infrastructure/
│   ├── execution/
│   │   ├── binance_execution.rs # ✅ 真实下单已实现
│   │   └── noop_execution.rs    # ✅ 空实现
│   └── exchange/
│       └── binance.rs           # ⚠️ 骨架
└── application/service/
    └── signal_consumer_service.rs # ✅ 信号消费流程
```

---

## 三、待开发任务清单

### 3.1 risk-management 风控服务

#### 任务 R1: 完善杠杆检查 (leverage.rs)

**文件**: `services/risk-management/src/domain/logic/leverage.rs`

**需求**:
- 计算实际杠杆 = 持仓价值 / 账户净值
- 检查是否超过用户设定的最大杠杆
- 支持不同交易对的杠杆限制

**接口设计**:
```rust
/// 杠杆检查配置
pub struct LeverageCheckConfig {
    /// 最大允许杠杆
    pub max_leverage: Decimal,
    /// 是否启用
    pub enabled: bool,
}

/// 杠杆检查上下文
pub struct LeverageContext {
    /// 当前持仓价值（USDT）
    pub position_value: Decimal,
    /// 账户净值（USDT）
    pub account_equity: Decimal,
    /// 新订单价值（USDT）
    pub new_order_value: Decimal,
}

/// 检查杠杆是否合规
pub fn check_leverage(
    config: &LeverageCheckConfig,
    context: &LeverageContext,
) -> Result<LeverageCheckResult> {
    // 实现逻辑
}

pub enum LeverageCheckResult {
    Pass,
    Reject { current: Decimal, max: Decimal, reason: String },
}
```

---

#### 任务 R2: 完善回撤检查 (drawdown.rs)

**文件**: `services/risk-management/src/domain/logic/drawdown.rs`

**需求**:
- 计算当前回撤 = (历史最高净值 - 当前净值) / 历史最高净值
- 检查是否超过最大允许回撤
- 支持日回撤、周回撤、总回撤

**接口设计**:
```rust
/// 回撤检查配置
pub struct DrawdownCheckConfig {
    /// 最大日回撤
    pub max_daily_drawdown: Option<Decimal>,
    /// 最大周回撤
    pub max_weekly_drawdown: Option<Decimal>,
    /// 最大总回撤
    pub max_total_drawdown: Option<Decimal>,
    /// 是否启用
    pub enabled: bool,
}

/// 回撤检查上下文
pub struct DrawdownContext {
    /// 当前净值
    pub current_equity: Decimal,
    /// 日初净值
    pub daily_start_equity: Decimal,
    /// 周初净值
    pub weekly_start_equity: Decimal,
    /// 历史最高净值
    pub peak_equity: Decimal,
}

/// 检查回撤是否合规
pub fn check_drawdown(
    config: &DrawdownCheckConfig,
    context: &DrawdownContext,
) -> Result<DrawdownCheckResult> {
    // 实现逻辑
}

pub enum DrawdownCheckResult {
    Pass,
    Reject { 
        drawdown_type: String,  // "daily" / "weekly" / "total"
        current: Decimal, 
        max: Decimal, 
        reason: String 
    },
}
```

---

#### 任务 R3: 新增持仓限制检查 (position_limit.rs)

**文件**: `services/risk-management/src/domain/logic/position_limit.rs` (新建)

**需求**:
- 单个交易对最大持仓
- 总持仓最大价值
- 单笔订单最大价值

**接口设计**:
```rust
/// 持仓限制配置
pub struct PositionLimitConfig {
    /// 单交易对最大持仓（USDT）
    pub max_position_per_symbol: Option<Decimal>,
    /// 总持仓最大价值（USDT）
    pub max_total_position: Option<Decimal>,
    /// 单笔订单最大价值（USDT）
    pub max_order_value: Option<Decimal>,
    /// 是否启用
    pub enabled: bool,
}

/// 持仓限制上下文
pub struct PositionLimitContext {
    /// 交易对
    pub symbol: String,
    /// 当前该交易对持仓价值
    pub current_symbol_position: Decimal,
    /// 当前总持仓价值
    pub current_total_position: Decimal,
    /// 新订单价值
    pub new_order_value: Decimal,
}

/// 检查持仓限制
pub fn check_position_limit(
    config: &PositionLimitConfig,
    context: &PositionLimitContext,
) -> Result<PositionLimitResult> {
    // 实现逻辑
}
```

---

#### 任务 R4: 新增每日亏损限额 (daily_loss.rs)

**文件**: `services/risk-management/src/domain/logic/daily_loss.rs` (新建)

**需求**:
- 跟踪每日已实现亏损
- 超过限额后禁止开新仓
- 每日 UTC 0 点重置

**接口设计**:
```rust
/// 每日亏损限额配置
pub struct DailyLossConfig {
    /// 最大每日亏损（USDT）
    pub max_daily_loss: Decimal,
    /// 是否启用
    pub enabled: bool,
}

/// 每日亏损上下文
pub struct DailyLossContext {
    /// 今日已实现亏损（正数表示亏损）
    pub realized_loss_today: Decimal,
    /// 今日已实现盈利
    pub realized_profit_today: Decimal,
}

/// 检查每日亏损限额
pub fn check_daily_loss(
    config: &DailyLossConfig,
    context: &DailyLossContext,
) -> Result<DailyLossResult> {
    // 实现逻辑
}
```

---

#### 任务 R5: 完善风险评估器 (risk_evaluator.rs)

**文件**: `services/risk-management/src/domain/service/risk_evaluator.rs`

**需求**:
- 整合所有风控检查
- 按顺序执行检查
- 任一检查失败则拒绝

**接口设计**:
```rust
pub struct RiskEvaluator {
    leverage_config: LeverageCheckConfig,
    drawdown_config: DrawdownCheckConfig,
    position_limit_config: PositionLimitConfig,
    daily_loss_config: DailyLossConfig,
}

impl RiskEvaluator {
    /// 综合风险评估
    pub fn evaluate(&self, request: &RiskCheckRequest) -> Result<RiskDecision> {
        // 1. 杠杆检查
        // 2. 回撤检查
        // 3. 持仓限制检查
        // 4. 每日亏损检查
        // 全部通过才返回 Approved
    }
}

pub struct RiskCheckRequest {
    pub user_id: Uuid,
    pub symbol: String,
    pub side: String,           // "buy" / "sell"
    pub order_value: Decimal,
    pub leverage_context: LeverageContext,
    pub drawdown_context: DrawdownContext,
    pub position_context: PositionLimitContext,
    pub daily_loss_context: DailyLossContext,
}
```

---

#### 任务 R6: 风控配置 HTTP API

**文件**: `services/risk-management/src/interface/http/handlers/risk_config.rs` (新建)

**需求**:
- GET /api/v1/risk/config/{user_id} - 获取用户风控配置
- PUT /api/v1/risk/config/{user_id} - 更新用户风控配置
- POST /api/v1/risk/check - 执行风控检查（供 trading-engine 调用）

**DTO 设计**:
```rust
#[derive(Serialize, Deserialize)]
pub struct RiskConfigDto {
    pub user_id: Uuid,
    pub max_leverage: Decimal,
    pub max_daily_drawdown: Decimal,
    pub max_total_drawdown: Decimal,
    pub max_position_per_symbol: Decimal,
    pub max_total_position: Decimal,
    pub max_daily_loss: Decimal,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RiskCheckRequestDto {
    pub user_id: Uuid,
    pub symbol: String,
    pub side: String,
    pub quantity: Decimal,
    pub price: Decimal,
}

#[derive(Serialize, Deserialize)]
pub struct RiskCheckResponseDto {
    pub approved: bool,
    pub reason: Option<String>,
    pub checks: Vec<RiskCheckDetail>,
}
```

---

### 3.2 trading-engine 交易执行服务

#### 任务 T1: 订单查询接口

**文件**: `services/trading-engine/src/infrastructure/exchange/binance.rs`

**需求**:
- 查询单个订单状态
- 查询所有未完成订单
- 查询历史订单

**接口设计**:
```rust
#[async_trait]
pub trait ExchangePort: Send + Sync {
    /// 查询订单状态
    async fn get_order(&self, symbol: &str, order_id: &str) -> Result<OrderInfo>;
    
    /// 查询未完成订单
    async fn get_open_orders(&self, symbol: Option<&str>) -> Result<Vec<OrderInfo>>;
    
    /// 查询历史订单
    async fn get_order_history(
        &self, 
        symbol: &str, 
        start_time: Option<i64>,
        end_time: Option<i64>,
        limit: Option<u32>,
    ) -> Result<Vec<OrderInfo>>;
}

pub struct OrderInfo {
    pub order_id: String,
    pub client_order_id: String,
    pub symbol: String,
    pub side: String,
    pub order_type: String,
    pub status: OrderStatus,
    pub price: Decimal,
    pub quantity: Decimal,
    pub executed_qty: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub enum OrderStatus {
    New,
    PartiallyFilled,
    Filled,
    Canceled,
    Rejected,
    Expired,
}
```

---

#### 任务 T2: 撤单功能

**文件**: `services/trading-engine/src/infrastructure/exchange/binance.rs`

**需求**:
- 撤销单个订单
- 撤销某交易对所有订单

**接口设计**:
```rust
#[async_trait]
pub trait ExchangePort: Send + Sync {
    // ... 之前的方法
    
    /// 撤销订单
    async fn cancel_order(&self, symbol: &str, order_id: &str) -> Result<CancelResult>;
    
    /// 撤销所有订单
    async fn cancel_all_orders(&self, symbol: &str) -> Result<Vec<CancelResult>>;
}

pub struct CancelResult {
    pub order_id: String,
    pub symbol: String,
    pub status: String,
    pub success: bool,
}
```

---

#### 任务 T3: 持仓同步

**文件**: `services/trading-engine/src/infrastructure/exchange/binance.rs`

**需求**:
- 从币安获取当前持仓
- 获取账户余额
- 定时同步（可选）

**接口设计**:
```rust
#[async_trait]
pub trait ExchangePort: Send + Sync {
    // ... 之前的方法
    
    /// 获取账户余额
    async fn get_account_balance(&self) -> Result<AccountBalance>;
    
    /// 获取持仓（合约）
    async fn get_positions(&self) -> Result<Vec<Position>>;
    
    /// 获取现货余额
    async fn get_spot_balances(&self) -> Result<Vec<SpotBalance>>;
}

pub struct AccountBalance {
    pub total_balance: Decimal,
    pub available_balance: Decimal,
    pub unrealized_pnl: Decimal,
}

pub struct Position {
    pub symbol: String,
    pub side: String,           // "LONG" / "SHORT"
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub mark_price: Decimal,
    pub unrealized_pnl: Decimal,
    pub leverage: u32,
    pub margin_type: String,    // "isolated" / "cross"
}

pub struct SpotBalance {
    pub asset: String,
    pub free: Decimal,
    pub locked: Decimal,
}
```

---

#### 任务 T4: 订单管理 HTTP API

**文件**: `services/trading-engine/src/interface/http/handlers/orders.rs`

**需求**:
- GET /api/v1/orders/{order_id} - 查询订单
- GET /api/v1/orders?symbol=xxx - 查询订单列表
- DELETE /api/v1/orders/{order_id} - 撤销订单
- DELETE /api/v1/orders?symbol=xxx - 撤销所有订单

---

#### 任务 T5: 持仓查询 HTTP API

**文件**: `services/trading-engine/src/interface/http/handlers/positions.rs`

**需求**:
- GET /api/v1/positions - 获取所有持仓
- GET /api/v1/positions/{symbol} - 获取指定交易对持仓
- GET /api/v1/account/balance - 获取账户余额

---

#### 任务 T6: 调用风控服务

**文件**: `services/trading-engine/src/infrastructure/risk/remote_risk.rs` (新建)

**需求**:
- 在执行前调用 risk-management 服务
- 实现 RiskPort trait
- HTTP 调用 risk-management 的 /api/v1/risk/check

**接口设计**:
```rust
pub struct RemoteRiskClient {
    base_url: String,
    client: reqwest::Client,
}

impl RemoteRiskClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl RiskPort for RemoteRiskClient {
    async fn check(&self, signal: &SignalEvent) -> Result<()> {
        // 调用 risk-management 服务
        // POST http://risk-management:8085/api/v1/risk/check
    }
}
```

---

## 四、文件位置汇总

### risk-management 需要修改/新建的文件

```
services/risk-management/src/
├── domain/
│   ├── logic/
│   │   ├── mod.rs              # 修改：添加新模块导出
│   │   ├── leverage.rs         # 修改：完善实现
│   │   ├── drawdown.rs         # 修改：完善实现
│   │   ├── position_limit.rs   # 新建
│   │   └── daily_loss.rs       # 新建
│   ├── model/
│   │   ├── mod.rs              # 修改：添加新模型
│   │   └── risk_config.rs      # 新建：风控配置模型
│   └── service/
│       └── risk_evaluator.rs   # 修改：完善实现
├── interface/http/
│   ├── handlers/
│   │   ├── mod.rs              # 修改：添加新 handler
│   │   └── risk_config.rs      # 新建
│   └── routes.rs               # 修改：添加路由
└── application/service/
    └── risk_check_service.rs   # 修改：完善实现
```

### trading-engine 需要修改/新建的文件

```
services/trading-engine/src/
├── domain/
│   └── port/
│       └── exchange_port.rs    # 修改：添加新方法
├── infrastructure/
│   ├── exchange/
│   │   └── binance.rs          # 修改：实现新方法
│   └── risk/
│       ├── mod.rs              # 修改：添加新模块
│       └── remote_risk.rs      # 新建：远程风控调用
├── interface/http/
│   ├── handlers/
│   │   ├── mod.rs              # 修改
│   │   ├── orders.rs           # 新建/修改
│   │   └── positions.rs        # 新建/修改
│   └── routes.rs               # 修改：添加路由
└── bootstrap.rs                # 修改：注入新依赖
```

---

## 五、开发规范（必须遵守）

### 5.1 禁止事项

| 禁止项 | 说明 |
|--------|------|
| ❌ `unwrap()` | 用 `?` 或 `ok_or()` |
| ❌ `expect()` | 同上 |
| ❌ `panic!()` | 用 `anyhow::bail!()` |
| ❌ `todo!()` | 返回 `Err` 或空实现 |
| ❌ 硬编码 URL/端口 | 从环境变量读取 |
| ❌ 单文件超过 800 行 | 必须拆分 |
| ❌ Domain 层依赖框架 | 只用 std 和 shared |

### 5.2 必须遵守

| 要求 | 说明 |
|------|------|
| ✅ 所有 public 结构体有文档注释 | `///` |
| ✅ 错误处理用 `anyhow::Result` | 加 `.context()` |
| ✅ 异步函数返回 `Result` | 不要返回裸值 |
| ✅ 配置从环境变量读取 | `std::env::var()` |
| ✅ Adapter 只在 bootstrap.rs 创建 | 依赖注入 |

### 5.3 环境变量

```env
# risk-management
RISK_MANAGEMENT_PORT=8085

# trading-engine
TRADING_ENGINE_PORT=8081
RISK_MANAGEMENT_URL=http://localhost:8085

# 币安 API
BINANCE_API_KEY=xxx
BINANCE_SECRET_KEY=xxx
BINANCE_BASE_URL=https://testnet.binance.vision      # 现货测试网
BINANCE_FUTURES_URL=https://testnet.binancefuture.com # 合约测试网
```

---

## 六、开发顺序建议

### Phase 1: 风控基础 (risk-management)
1. R1 杠杆检查
2. R2 回撤检查
3. R3 持仓限制
4. R4 每日亏损
5. R5 风险评估器整合

### Phase 2: 风控 API (risk-management)
6. R6 HTTP API

### Phase 3: 交易所接口 (trading-engine)
7. T1 订单查询
8. T2 撤单功能
9. T3 持仓同步

### Phase 4: 交易 API (trading-engine)
10. T4 订单管理 API
11. T5 持仓查询 API
12. T6 远程风控调用

---

## 七、验收标准

### 7.1 编译检查
```bash
cargo check -p risk-management
cargo check -p trading-engine
```
必须无错误通过。

### 7.2 功能验收

**risk-management**:
- [ ] 杠杆检查能正确计算和拦截
- [ ] 回撤检查能正确计算和拦截
- [ ] 持仓限制能正确检查
- [ ] 每日亏损限额能正确检查
- [ ] HTTP API 能正常调用
- [ ] 风控配置能 CRUD

**trading-engine**:
- [ ] 能查询订单状态
- [ ] 能撤销订单
- [ ] 能获取持仓信息
- [ ] 能获取账户余额
- [ ] 能调用远程风控服务
- [ ] HTTP API 能正常调用

### 7.3 代码检查
- [ ] 无禁止项违规
- [ ] 有完整文档注释
- [ ] 架构分层正确
- [ ] 依赖方向正确

---

## 八、参考文件

开发前请先阅读：

1. `services/trading-engine/src/infrastructure/execution/binance_execution.rs` - 币安下单实现
2. `services/trading-engine/src/domain/logic/risk_rules.rs` - 现有风控规则
3. `services/trading-engine/src/application/service/signal_consumer_service.rs` - 信号消费流程
4. `shared/src/event/signal_event.rs` - 信号事件定义

---

## 九、币安 API 参考

### 现货 API
- 下单: POST /api/v3/order
- 查询订单: GET /api/v3/order
- 撤单: DELETE /api/v3/order
- 账户信息: GET /api/v3/account

### 合约 API
- 下单: POST /fapi/v1/order
- 查询订单: GET /fapi/v1/order
- 撤单: DELETE /fapi/v1/order
- 持仓: GET /fapi/v2/positionRisk
- 账户: GET /fapi/v2/account

### 签名方式
所有需要签名的请求都需要：
1. 添加 `timestamp` 参数
2. 用 HMAC-SHA256 签名整个 query string
3. 添加 `signature` 参数
4. Header 添加 `X-MBX-APIKEY`

参考现有实现: `binance_execution.rs`

---

**有问题先问，不要猜！**
