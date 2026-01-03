# 📋 策略开发任务书

> **任务类型**: 策略算法实现
> **验收人**: Kiro（架构师）
> **必读文档**: `TEAM_DEVELOPMENT_GUIDE.md`

---

## 一、当前状态

策略模块框架已搭建完成，目录结构如下：

```
services/strategy-engine/src/domain/logic/
├── strategy_trait.rs       # ✅ 统一策略 Trait（已完成）
├── strategy_registry.rs    # ✅ 策略注册表（已完成）
│
├── spot/                   # 现货策略目录
│   ├── mod.rs
│   ├── grid.rs             # ✅ 网格策略（已完成）
│   └── mean.rs             # ✅ 均值回归（已完成）
│
├── futures/                # 合约策略目录
│   ├── mod.rs
│   ├── grid.rs             # ✅ 网格策略（已完成）
│   ├── mean.rs             # ✅ 均值回归（已完成）
│   └── funding_arb.rs      # ✅ 资金费率套利（已完成）
│
├── ai/                     # AI 策略目录（与 ai-service 配合）
│   └── mod.rs              # 待实现
│
└── hft/                    # 高频策略目录（独立，低延迟要求）
    └── mod.rs              # 待实现
```

---

## 二、待开发策略清单

### 2.1 现货策略（放 `spot/` 目录）

| 策略名称 | 文件名 | 优先级 | 说明 |
|----------|--------|--------|------|
| MACD 策略 | `macd.rs` | 🔴 高 | 基于 MACD 指标的趋势跟踪 |
| RSI 策略 | `rsi.rs` | 🔴 高 | 基于 RSI 超买超卖信号 |
| 布林带策略 | `bollinger.rs` | 🟡 中 | 价格触及布林带边界时交易 |
| 双均线策略 | `dual_ma.rs` | 🟡 中 | 快慢均线金叉死叉 |
| 突破策略 | `breakout.rs` | 🟡 中 | 价格突破关键位置 |
| 动量策略 | `momentum.rs` | 🟢 低 | 基于价格动量 |

### 2.2 合约策略（放 `futures/` 目录）

| 策略名称 | 文件名 | 优先级 | 说明 |
|----------|--------|--------|------|
| MACD 策略 | `macd.rs` | 🔴 高 | 支持杠杆的 MACD |
| RSI 策略 | `rsi.rs` | 🔴 高 | 支持杠杆的 RSI |
| 布林带策略 | `bollinger.rs` | 🟡 中 | 支持杠杆的布林带 |
| 期现套利 | `basis_arb.rs` | 🟡 中 | 现货与合约价差套利 |
| 跨期套利 | `calendar_arb.rs` | 🟢 低 | 不同到期日合约套利 |

### 2.3 AI 策略（放 `ai/` 目录）

| 策略名称 | 文件名 | 优先级 | 说明 |
|----------|--------|--------|------|
| ML 信号策略 | `ml_signal.rs` | 🟡 中 | 机器学习模型预测信号 |
| 情绪分析策略 | `sentiment.rs` | 🟡 中 | 基于市场情绪分析 |
| 模式识别策略 | `pattern.rs` | 🟢 低 | K线形态识别 |
| 强化学习策略 | `reinforcement.rs` | 🟢 低 | RL 自适应策略 |

> **注意**: AI 策略需要与 `ai-service` (8087) 配合，通过 HTTP/gRPC 调用模型推理。

### 2.4 高频策略（放 `hft/` 目录）

| 策略名称 | 文件名 | 优先级 | 说明 |
|----------|--------|--------|------|
| 做市策略 | `market_making.rs` | 🔴 高 | 双边挂单赚取价差 |
| 剥头皮策略 | `scalping.rs` | 🔴 高 | 快进快出小利润 |
| 延迟套利 | `latency_arb.rs` | 🟡 中 | 利用交易所延迟差 |
| 订单流策略 | `order_flow.rs` | 🟡 中 | 基于订单簿分析 |

> **注意**: 高频策略对延迟要求极高，需要使用 `on_tick()` 方法而非 `on_market_event()`。

---

## 三、开发规范（必须遵守）

### 3.1 文件位置

```
# 现货策略
services/strategy-engine/src/domain/logic/spot/xxx.rs

# 合约策略
services/strategy-engine/src/domain/logic/futures/xxx.rs
```

### 3.2 必须实现的结构

每个策略文件必须包含以下结构：

```rust
// 1. 策略配置（用户可配置的参数）
pub struct XxxConfig {
    // 策略参数...
}

// 2. 策略状态（运行时状态）
pub struct XxxState {
    // 运行状态...
}

// 3. 策略实现
pub struct XxxStrategy {
    meta: StrategyMeta,
    config: XxxConfig,
    state: XxxState,
}

// 4. 必须实现 Strategy trait
impl Strategy for XxxStrategy {
    fn meta(&self) -> &StrategyMeta { ... }
    fn meta_mut(&mut self) -> &mut StrategyMeta { ... }
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<Signal> { ... }
    fn reset(&mut self) { ... }
}
```

### 3.3 代码模板

参考现有实现 `spot/macd.rs` 示例：

```rust
//! # 现货 MACD 策略 (Spot MACD Strategy)
//!
//! 基于 MACD 指标的趋势跟踪策略。

use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// MACD 策略配置
#[derive(Debug, Clone)]
pub struct SpotMacdConfig {
    /// 快线周期
    pub fast_period: usize,
    /// 慢线周期
    pub slow_period: usize,
    /// 信号线周期
    pub signal_period: usize,
    /// 交易数量
    pub quantity: Decimal,
}

/// MACD 策略状态
#[derive(Debug, Clone)]
pub struct SpotMacdState {
    /// 价格历史
    pub price_history: Vec<Decimal>,
    /// 快线 EMA
    pub fast_ema: Option<Decimal>,
    /// 慢线 EMA
    pub slow_ema: Option<Decimal>,
    /// MACD 线
    pub macd_line: Option<Decimal>,
    /// 信号线
    pub signal_line: Option<Decimal>,
    /// 上一次 MACD 柱状图值
    pub last_histogram: Option<Decimal>,
}

impl SpotMacdState {
    pub fn new() -> Self {
        Self {
            price_history: Vec::new(),
            fast_ema: None,
            slow_ema: None,
            macd_line: None,
            signal_line: None,
            last_histogram: None,
        }
    }
}

impl Default for SpotMacdState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货 MACD 策略
pub struct SpotMacdStrategy {
    meta: StrategyMeta,
    config: SpotMacdConfig,
    state: SpotMacdState,
}

impl SpotMacdStrategy {
    /// 创建策略实例
    pub fn new(instance_id: Uuid, symbol: String, config: SpotMacdConfig) -> Self {
        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "spot_macd".to_string(),
                market_type: MarketType::Spot,
                symbol,
                is_active: false,
            },
            config,
            state: SpotMacdState::new(),
        }
    }

    /// 计算 EMA
    fn calculate_ema(prices: &[Decimal], period: usize) -> Option<Decimal> {
        if prices.len() < period {
            return None;
        }
        // EMA 计算逻辑...
        // TODO: 实现 EMA 计算
        None
    }

    /// 计算信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        // 更新价格历史
        self.state.price_history.push(trade.price);

        // 计算 MACD 指标
        // TODO: 实现 MACD 计算逻辑

        // 判断金叉/死叉
        // TODO: 实现信号判断

        None // 暂时返回 None
    }
}

impl Strategy for SpotMacdStrategy {
    fn meta(&self) -> &StrategyMeta {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut StrategyMeta {
        &mut self.meta
    }

    fn on_market_event(&mut self, event: &MarketEvent) -> Option<Signal> {
        if !self.is_active() {
            return None;
        }
        self.calculate_signal(event)
    }

    fn reset(&mut self) {
        self.state = SpotMacdState::new();
    }
}
```

### 3.4 合约策略额外要求

合约策略需要额外支持：

```rust
use crate::domain::model::market_type::{LeverageConfig, MarketType, PositionSide};

pub struct FuturesXxxConfig {
    // ... 策略参数
    pub leverage: LeverageConfig,      // 杠杆配置
    pub position_side: PositionSide,   // 持仓方向
}
```

---

## 四、禁止事项（红线）

| 禁止项 | 说明 |
|--------|------|
| ❌ `unwrap()` | 用 `?` 或 `Option` |
| ❌ `expect()` | 同上 |
| ❌ `panic!()` | 用 `return None` |
| ❌ `todo!()` | 返回 `None` 或空实现 |
| ❌ 修改现有文件 | 只能新增文件 |
| ❌ 修改 `strategy_trait.rs` | 框架已定，不能改 |
| ❌ 修改 `strategy_registry.rs` | 框架已定，不能改 |
| ❌ 单文件超过 500 行 | 保持精简 |

---

## 五、完成后必须做的事

### 5.1 更新 mod.rs

在对应目录的 `mod.rs` 中添加导出：

```rust
// spot/mod.rs
pub mod macd;
pub use macd::SpotMacdStrategy;

// futures/mod.rs
pub mod macd;
pub use macd::FuturesMacdStrategy;
```

### 5.2 编译检查

```bash
cargo check -p strategy-engine
```

必须无错误通过。

### 5.3 提交验收

通知架构师（Kiro）验收，检查项：
- [ ] 文件位置正确
- [ ] 实现了 Strategy trait
- [ ] 无禁止项违规
- [ ] 编译通过
- [ ] 有完整文档注释

---

## 六、参考文件

开发前请先阅读以下已完成的策略实现：

1. `services/strategy-engine/src/domain/logic/spot/grid.rs` - 现货网格
2. `services/strategy-engine/src/domain/logic/spot/mean.rs` - 现货均值回归
3. `services/strategy-engine/src/domain/logic/futures/grid.rs` - 合约网格
4. `services/strategy-engine/src/domain/logic/strategy_trait.rs` - Strategy trait 定义

---

## 七、开发顺序建议

1. 先做现货 MACD (`spot/macd.rs`)
2. 再做现货 RSI (`spot/rsi.rs`)
3. 然后复制到合约版本，加上杠杆支持
4. 最后做其他策略

---

**有问题先问，不要猜！**
