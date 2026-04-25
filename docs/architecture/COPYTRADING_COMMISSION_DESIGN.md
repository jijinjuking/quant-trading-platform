# CopyTrading 与分佣系统 - 终局工程设计

> **文档状态**: 🔒 冻结
> **生效日期**: 2026-01-07
> **设计原则**: 不侵入 Strategy / Trading Engine
> **架构规范**: 严格遵循 DDD + Hexagonal

---

## 一、系统定位

### 1.1 核心约束

| 约束 | 说明 |
|------|------|
| ❌ 不侵入 Strategy Engine | CopyTrading 不修改策略逻辑 |
| ❌ 不侵入 Trading Engine | CopyTrading 不修改执行流程 |
| ✅ 发生在 Strategy 输出之后 | 监听 StrategyResult，生成 Follower 的 ExecutionDraft |
| ✅ 独立服务 | CopyTrading 是独立微服务 |

### 1.2 DDD 架构遵循

按照项目 DDD 规范：

**shared/ 只放**：
- `types/` - 纯数据结构（copytrading.rs, commission.rs）
- `event/` - Kafka 事件（copytrading_event.rs, commission_event.rs）
- `error/` - 错误定义
- `utils/` - 工具函数

**Port trait 放在各服务的 `domain/port/`**：
- CopyTrading Service 的 Port 放在 `services/copytrading/src/domain/port/`
- Commission Service 的 Port 放在 `services/commission/src/domain/port/`

### 1.3 数据流（终局）

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Strategy Engine                                    │
│                    （不感知 CopyTrading 存在）                                │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ StrategyResultEvent (Kafka: strategy.results)
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        CopyTrading Service (8089)                            │
│                                                                              │
│  1. 接收 Leader 的 StrategyResultEvent                                       │
│  2. 查询 Leader-Follower 关系                                                │
│  3. 为每个 Follower 生成 ExecutionDraftEvent                                 │
│  4. 应用 Follower 的 Scaling / Risk Override                                 │
│  5. 发送 ExecutionDraftEvent 到 Kafka                                        │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ ExecutionDraftEvent (Kafka: execution.drafts)
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                           Trading Engine (8081)                              │
│                    （不感知 CopyTrading 存在）                                │
│                                                                              │
│  - 统一消费 ExecutionDraftEvent                                              │
│  - 执行风控检查                                                              │
│  - 执行下单                                                                  │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ ExecutionResult (Kafka: execution.results)
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Commission Service (8090)                             │
│                                                                              │
│  1. 监听 ExecutionResult                                                     │
│  2. 识别 CopyTrading 订单                                                    │
│  3. 计算分佣（Leader / Platform / Follower）                                 │
│  4. 生成 CommissionRecordEvent                                               │
│  5. 发送到结算系统                                                           │
└─────────────────────────────────────────────────────────────────────────────┘
                                    │
                                    │ CommissionRecordEvent (Kafka: commission.records)
                                    ▼
┌─────────────────────────────────────────────────────────────────────────────┐
│                        Accounting Service (8091)                             │
│                                                                              │
│  - 结算处理                                                                  │
│  - 账户余额更新                                                              │
│  - 提现管理                                                                  │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 二、CopyTrading 终局模型

### 2.1 Leader（带单者）

```
Leader = 策略 Owner + 公开策略
```

| 属性 | 说明 |
|------|------|
| leader_id | 带单者 ID（= user_id） |
| strategy_id | 公开的策略实例 ID |
| commission_rate | 分佣比例（0.0 - 1.0） |
| max_followers | 最大跟单人数 |
| min_follow_amount | 最小跟单金额 |
| status | 状态（active / paused / closed） |

### 2.2 Follower（跟单者）

```
Follower = 用户 + 跟单配置
```

| 属性 | 说明 |
|------|------|
| follower_id | 跟单者 ID（= user_id） |
| leader_id | 跟随的 Leader ID |
| strategy_id | 跟随的策略 ID |
| scaling_config | 缩放配置 |
| risk_override | 风控覆盖配置 |
| status | 状态（active / paused / stopped） |

### 2.3 Scaling 配置

```
Scaling = 如何将 Leader 的交易映射到 Follower
```

| 模式 | 说明 |
|------|------|
| FixedRatio | 固定比例（如 Leader 1 BTC → Follower 0.1 BTC） |
| FixedAmount | 固定金额（如每笔最多 100 USDT） |
| ProportionalToCapital | 按资金比例（Follower 资金 / Leader 资金） |

### 2.4 Risk Override（风控覆盖）

```
Risk Override = Follower 可以覆盖的风控参数
```

| 参数 | 说明 |
|------|------|
| max_position_per_symbol | 单交易对最大持仓 |
| max_total_position | 总持仓上限 |
| max_single_order | 单笔订单上限 |
| allowed_symbols | 允许的交易对（白名单） |
| blocked_symbols | 禁止的交易对（黑名单） |

### 2.5 多 Follower 并行

```
1 Leader : N Followers（N ≤ max_followers）
```

- CopyTrading Processor 并行为所有 Follower 生成 ExecutionDraft
- 每个 Follower 的 ExecutionDraft 独立
- Trading Engine 并行处理所有 ExecutionDraft

---

## 三、分佣终局模型

### 3.1 分佣参与方

| 角色 | 说明 |
|------|------|
| Leader（策略 Owner） | 提供策略，获得跟单收益分成 |
| Platform | 平台，获得平台服务费 |
| Follower | 跟单者，支付分佣 |

### 3.2 分佣计算公式

```
Follower 盈利 = realized_pnl（已实现盈亏）

if realized_pnl > 0:
    leader_commission = realized_pnl × leader_rate
    platform_commission = realized_pnl × platform_rate
    follower_net = realized_pnl - leader_commission - platform_commission
else:
    leader_commission = 0
    platform_commission = 0
    follower_net = realized_pnl  # 亏损全部由 Follower 承担
```

### 3.3 结算时点

| 时点 | 触发条件 |
|------|----------|
| 实时结算 | 每笔平仓交易完成后立即计算 |
| 日结算 | 每日 UTC 0:00 汇总 |
| 周结算 | 每周日 UTC 0:00 汇总 |
| 月结算 | 每月 1 日 UTC 0:00 汇总 |
| 手动结算 | Follower 主动停止跟单时 |

### 3.4 高水位线（High Water Mark）

```
只有超过历史最高净值的盈利才计算分佣
```

- 防止 Leader 反复亏损后盈利重复收费
- 每个 Follower-Leader 关系独立维护高水位线

---

## 四、数据结构定义

### 4.1 Kafka Topics

| Topic | 生产者 | 消费者 | 说明 |
|-------|--------|--------|------|
| strategy.results | Strategy Engine | CopyTrading Processor | 策略执行结果 |
| execution.drafts | CopyTrading Processor | Trading Engine | 执行草稿 |
| execution.results | Trading Engine | Commission Processor | 执行结果 |
| commission.records | Commission Processor | Accounting Service | 分佣记录 |

### 4.2 核心数据结构

详见 `shared/src/copytrading/` 目录下的 Rust 定义。

---

## 五、与现有系统的边界

### 5.1 Strategy Engine 边界

```
Strategy Engine 不感知 CopyTrading 存在
```

| Strategy Engine 职责 | CopyTrading 职责 |
|---------------------|------------------|
| 执行策略计算 | 监听策略结果 |
| 输出 StrategyResult | 复制并转换为 ExecutionDraft |
| 不知道谁在跟单 | 管理 Leader-Follower 关系 |

### 5.2 Trading Engine 边界

```
Trading Engine 不感知 CopyTrading 存在
```

| Trading Engine 职责 | CopyTrading 职责 |
|--------------------|------------------|
| 消费 ExecutionDraft | 生成 ExecutionDraft |
| 执行风控检查 | 应用 Follower 风控覆盖 |
| 执行下单 | 不参与下单 |
| 输出 ExecutionResult | 监听 ExecutionResult 计算分佣 |

### 5.3 接口冻结

**Strategy Engine 输出（不可修改）**:
- `StrategyResult`（现有 `OrderIntent` 的包装）

**Trading Engine 输入（不可修改）**:
- `ExecutionDraft`（新增，兼容现有 `OrderIntent`）

**Trading Engine 输出（不可修改）**:
- `ExecutionResult`（现有）

---

## 六、工程约束

### 6.1 不绑定数据库

- 所有数据结构只定义接口
- Repository trait 由 infrastructure 层实现
- 支持 PostgreSQL / Redis / 内存实现

### 6.2 不写结算算法

- 只定义分佣计算的输入输出
- 具体算法由 Commission Processor 实现
- 支持多种结算策略

### 6.3 只冻结接口与数据模型

- 本文档定义的数据结构为终局版本
- 接口签名冻结，不允许修改
- 实现细节可以迭代

---

## 七、后续扩展预留

### 7.1 多账户支持

- Follower 可以选择跟单账户
- 一个用户可以有多个跟单账户

### 7.2 策略组合

- Follower 可以同时跟随多个 Leader
- 资金按比例分配

### 7.3 社交功能

- Leader 排行榜
- 历史收益展示
- 跟单者评价

---

**—— 文档结束 ——**
