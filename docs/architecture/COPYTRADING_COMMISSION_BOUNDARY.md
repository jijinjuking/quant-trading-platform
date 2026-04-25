# CopyTrading 与分佣系统 - 边界定义

> **文档状态**: 🔒 冻结
> **生效日期**: 2026-01-07

---

## 一、与 Strategy Engine 的边界

### 1.1 Strategy Engine 不变

| Strategy Engine 职责 | 不变 |
|---------------------|------|
| 执行策略计算 | ✅ |
| 输出 StrategyResult | ✅ |
| 管理策略生命周期 | ✅ |

### 1.2 Strategy Engine 不感知

| Strategy Engine 不感知 | 说明 |
|-----------------------|------|
| ❌ 谁在跟单 | 不知道有 Follower |
| ❌ 分佣比例 | 不参与分佣计算 |
| ❌ CopyTrading 存在 | 完全解耦 |

### 1.3 接口不变

```rust
// Strategy Engine 输出（不修改）
// 位置: shared/src/copytrading/result.rs
pub struct StrategyResult {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub owner_id: Uuid,           // = Leader ID
    pub status: StrategyResultStatus,
    pub intent: Option<TradeIntent>,
    pub trigger_event: Option<TriggerEvent>,
    pub execution_time_us: u64,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
}
```

---

## 二、与 Trading Engine 的边界

### 2.1 Trading Engine 不变

| Trading Engine 职责 | 不变 |
|--------------------|------|
| 消费 ExecutionDraft | ✅ |
| 执行风控检查 | ✅ |
| 执行下单 | ✅ |
| 输出 ExecutionResult | ✅ |

### 2.2 Trading Engine 不感知

| Trading Engine 不感知 | 说明 |
|----------------------|------|
| ❌ 订单来源 | 不区分直接/跟单 |
| ❌ 分佣逻辑 | 不参与分佣 |
| ❌ Leader-Follower 关系 | 只看 user_id |

### 2.3 接口扩展（兼容）

```rust
// Trading Engine 输入（新增，兼容 OrderIntent）
// 位置: shared/src/copytrading/draft.rs
pub struct ExecutionDraft {
    pub id: Uuid,
    pub source: DraftSource,      // Strategy / CopyTrading
    pub user_id: Uuid,            // 实际执行者
    pub strategy_id: Uuid,
    pub symbol: String,
    pub side: OrderSide,
    pub quantity: Decimal,
    pub price: Option<Decimal>,
    pub order_type: OrderType,
    pub confidence: f64,
    pub copytrading_meta: Option<CopyTradingMeta>,  // 可选
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}
```

**兼容性说明**:
- `copytrading_meta` 为 `Option`，非跟单订单为 `None`
- Trading Engine 可以忽略 `copytrading_meta`
- 风控检查基于 `user_id`，与来源无关

---

## 三、Kafka Topic 定义

| Topic | 生产者 | 消费者 | 数据结构 |
|-------|--------|--------|----------|
| `strategy.results` | Strategy Engine | CopyTrading Processor | `StrategyResult` |
| `execution.drafts` | CopyTrading Processor | Trading Engine | `ExecutionDraft` |
| `execution.results` | Trading Engine | Commission Processor | `ExecutionResult` |
| `commission.records` | Commission Processor | Accounting Service | `CommissionRecord` |

---

## 四、服务职责划分

### 4.1 CopyTrading Processor（新服务）

**端口**: 8089（建议）

**职责**:
1. 消费 `strategy.results`
2. 查询 Leader-Follower 关系
3. 为每个 Follower 计算缩放
4. 应用 Follower 风控覆盖
5. 生成 `ExecutionDraft`
6. 发布到 `execution.drafts`

**不做**:
- ❌ 策略计算
- ❌ 风控检查（由 Trading Engine 做）
- ❌ 下单执行
- ❌ 分佣计算

### 4.2 Commission Processor（新服务）

**端口**: 8090（建议）

**职责**:
1. 消费 `execution.results`
2. 识别 CopyTrading 订单
3. 计算分佣（高水位线）
4. 生成 `CommissionRecord`
5. 发布到 `commission.records`

**不做**:
- ❌ 策略计算
- ❌ 跟单复制
- ❌ 账户余额操作（由 Accounting 做）

### 4.3 Accounting Service（扩展）

**端口**: 8091（建议）

**职责**:
1. 消费 `commission.records`
2. 执行转账（Follower → Leader / Platform）
3. 更新账户余额
4. 生成结算报表

---

## 五、数据流时序

```
┌─────────────────┐
│ Strategy Engine │
└────────┬────────┘
         │ StrategyResult
         ▼
┌─────────────────────┐
│ CopyTrading         │
│ Processor           │
│                     │
│ 1. 查 Leader 配置   │
│ 2. 查所有 Follower  │
│ 3. 并行生成 Draft   │
└────────┬────────────┘
         │ ExecutionDraft × N
         ▼
┌─────────────────┐
│ Trading Engine  │
│                 │
│ 1. 风控检查     │
│ 2. 执行下单     │
└────────┬────────┘
         │ ExecutionResult × N
         ▼
┌─────────────────────┐
│ Commission          │
│ Processor           │
│                     │
│ 1. 识别跟单订单     │
│ 2. 计算分佣         │
│ 3. 生成记录         │
└────────┬────────────┘
         │ CommissionRecord × N
         ▼
┌─────────────────┐
│ Accounting      │
│ Service         │
│                 │
│ 1. 执行转账     │
│ 2. 更新余额     │
└─────────────────┘
```

---

## 六、故障隔离

### 6.1 CopyTrading Processor 故障

- Strategy Engine 不受影响
- Trading Engine 不受影响
- 直接策略订单正常执行
- 跟单订单暂停，待恢复后补发

### 6.2 Commission Processor 故障

- 交易正常执行
- 分佣记录暂存 Kafka
- 恢复后重新消费计算

### 6.3 Accounting Service 故障

- 交易正常执行
- 分佣记录已生成
- 转账延迟，待恢复后执行

---

## 七、扩展预留

### 7.1 多账户

```rust
pub struct FollowerConfig {
    // ... 现有字段
    pub account_id: Option<Uuid>,  // 预留：指定跟单账户
}
```

### 7.2 策略组合

```rust
pub struct FollowerConfig {
    // ... 现有字段
    pub allocation_weight: Option<Decimal>,  // 预留：资金分配权重
}
```

### 7.3 社交功能

```rust
pub struct LeaderStats {
    // ... 现有字段
    pub rating: Option<Decimal>,      // 预留：评分
    pub review_count: Option<u32>,    // 预留：评价数
}
```

---

**—— 文档结束 ——**
