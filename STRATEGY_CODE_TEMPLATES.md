# 18个量化策略完整代码实现

**生成时间**: 2026-01-23
**说明**: 本文档包含所有18个策略的完整Rust代码实现

---

## 使用说明

1. 每个策略都是独立的Rust文件
2. 复制代码到对应的文件路径
3. 所有策略都实现了`Strategy` trait
4. 可以通过`StrategyExecutorAdapter`适配到新架构

---

## 现货策略（5个）

### 1. 现货网格策略 (Spot Grid Strategy)

**文件**: `services/strategy-engine/src/domain/logic/spot/grid.rs`

**说明**:
- 在价格区间内设置网格
- 价格下跌时买入，上涨时卖出
- 适合震荡行情

**代码**: 已存在，无需修改

---

### 2. 现货均值回归策略 (Spot Mean Reversion Strategy)

**文件**: `services/strategy-engine/src/domain/logic/spot/mean.rs`

**说明**:
- 计算移动平均和标准差
- 价格偏离均值时反向交易
- 适合震荡行情

**代码**: 已存在，无需修改

---

### 3. 现货MACD策略 (Spot MACD Strategy)

**文件**: `services/strategy-engine/src/domain/logic/spot/macd.rs`

**说明**:
- MACD指标（快线、慢线、信号线）
- 金叉买入，死叉卖出
- 适合趋势行情

**完整代码**:

```rust
//! # 现货MACD策略 (Spot MACD Strategy)
//!
//! 基于MACD指标的趋势跟踪策略。

use std::collections::VecDeque;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// MACD策略配置
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

impl Default for SpotMacdConfig {
    fn default() -> Self {
        Self {
            fast_period: 12,
            slow_period: 26,
            signal_period: 9,
            quantity: Decimal::new(1, 3), // 0.001
        }
    }
}

/// MACD策略状态
#[derive(Debug, Clone)]
pub struct SpotMacdState {
    /// 价格历史
    pub price_history: VecDeque<Decimal>,
    /// 快线EMA
    pub fast_ema: Option<Decimal>,
    /// 慢线EMA
    pub slow_ema: Option<Decimal>,
    /// MACD值历史
    pub macd_history: VecDeque<Decimal>,
    /// 信号线
    pub signal_line: Option<Decimal>,
    /// 上次信号
    pub last_signal: Option<SignalType>,
}

impl SpotMacdState {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::new(),
            fast_ema: None,
            slow_ema: None,
            macd_history: VecDeque::new(),
            signal_line: None,
            last_signal: None,
        }
    }
}

impl Default for SpotMacdState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货MACD策略
pub struct SpotMacdStrategy {
    meta: StrategyMeta,
    config: SpotMacdConfig,
    state: SpotMacdState,
}

impl SpotMacdStrategy {
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

    /// 计算EMA
    fn calculate_ema(&self, current_price: Decimal, prev_ema: Option<Decimal>, period: usize) -> Decimal {
        let multiplier = Decimal::from(2) / Decimal::from(period + 1);

        match prev_ema {
            Some(prev) => current_price * multiplier + prev * (Decimal::ONE - multiplier),
            None => current_price,
        }
    }

    /// 计算MACD信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        let price = trade.price;

        // 更新快线EMA
        self.state.fast_ema = Some(self.calculate_ema(
            price,
            self.state.fast_ema,
            self.config.fast_period,
        ));

        // 更新慢线EMA
        self.state.slow_ema = Some(self.calculate_ema(
            price,
            self.state.slow_ema,
            self.config.slow_period,
        ));

        // 计算MACD值
        let macd = match (self.state.fast_ema, self.state.slow_ema) {
            (Some(fast), Some(slow)) => fast - slow,
            _ => return None,
        };

        // 更新MACD历史
        self.state.macd_history.push_back(macd);
        if self.state.macd_history.len() > self.config.signal_period {
            self.state.macd_history.pop_front();
        }

        // 计算信号线（MACD的EMA）
        if self.state.macd_history.len() >= self.config.signal_period {
            let signal_line = self.calculate_ema(
                macd,
                self.state.signal_line,
                self.config.signal_period,
            );
            self.state.signal_line = Some(signal_line);

            // 判断金叉死叉
            let signal_type = if macd > signal_line && self.state.last_signal != Some(SignalType::Buy) {
                Some(SignalType::Buy) // 金叉
            } else if macd < signal_line && self.state.last_signal != Some(SignalType::Sell) {
                Some(SignalType::Sell) // 死叉
            } else {
                None
            };

            if let Some(sig_type) = signal_type {
                self.state.last_signal = Some(sig_type);

                return Some(Signal {
                    id: Uuid::new_v4(),
                    strategy_id: self.meta.instance_id,
                    symbol: event.symbol.clone(),
                    signal_type: sig_type,
                    price,
                    quantity: self.config.quantity,
                    confidence: 0.8,
                    created_at: event.timestamp,
                });
            }
        }

        None
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

---

### 4. 现货布林带策略 (Spot Bollinger Bands Strategy)

**文件**: `services/strategy-engine/src/domain/logic/spot/bollinger.rs`

**说明**:
- 布林带上轨、中轨、下轨
- 突破上轨卖出，突破下轨买入
- 适合震荡行情

**完整代码**:

```rust
//! # 现货布林带策略 (Spot Bollinger Bands Strategy)
//!
//! 基于布林带指标的突破策略。

use std::collections::VecDeque;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// 布林带策略配置
#[derive(Debug, Clone)]
pub struct SpotBollingerConfig {
    /// 周期
    pub period: usize,
    /// 标准差倍数
    pub std_dev_multiplier: Decimal,
    /// 交易数量
    pub quantity: Decimal,
}

impl Default for SpotBollingerConfig {
    fn default() -> Self {
        Self {
            period: 20,
            std_dev_multiplier: Decimal::from(2),
            quantity: Decimal::new(1, 3),
        }
    }
}

/// 布林带策略状态
#[derive(Debug, Clone)]
pub struct SpotBollingerState {
    /// 价格历史
    pub price_history: VecDeque<Decimal>,
    /// 上次信号
    pub last_signal: Option<SignalType>,
}

impl SpotBollingerState {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::new(),
            last_signal: None,
        }
    }
}

impl Default for SpotBollingerState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货布林带策略
pub struct SpotBollingerStrategy {
    meta: StrategyMeta,
    config: SpotBollingerConfig,
    state: SpotBollingerState,
}

impl SpotBollingerStrategy {
    pub fn new(instance_id: Uuid, symbol: String, config: SpotBollingerConfig) -> Self {
        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "spot_bollinger".to_string(),
                market_type: MarketType::Spot,
                symbol,
                is_active: false,
            },
            config,
            state: SpotBollingerState::new(),
        }
    }

    /// 计算标准差
    fn calculate_std_dev(&self, prices: &VecDeque<Decimal>, mean: Decimal) -> Option<Decimal> {
        if prices.is_empty() {
            return None;
        }

        let variance: Decimal = prices
            .iter()
            .map(|p| {
                let diff = *p - mean;
                diff * diff
            })
            .sum::<Decimal>()
            / Decimal::from(prices.len());

        // 简化的平方根计算（使用迭代法）
        let mut x = variance;
        for _ in 0..10 {
            if x == Decimal::ZERO {
                break;
            }
            x = (x + variance / x) / Decimal::from(2);
        }

        Some(x)
    }

    /// 计算布林带信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        let price = trade.price;

        // 更新价格历史
        self.state.price_history.push_back(price);
        if self.state.price_history.len() > self.config.period {
            self.state.price_history.pop_front();
        }

        // 需要足够的数据
        if self.state.price_history.len() < self.config.period {
            return None;
        }

        // 计算中轨（移动平均）
        let middle_band: Decimal = self.state.price_history.iter().sum::<Decimal>()
            / Decimal::from(self.state.price_history.len());

        // 计算标准差
        let std_dev = self.calculate_std_dev(&self.state.price_history, middle_band)?;

        // 计算上轨和下轨
        let upper_band = middle_band + std_dev * self.config.std_dev_multiplier;
        let lower_band = middle_band - std_dev * self.config.std_dev_multiplier;

        // 判断信号
        let signal_type = if price < lower_band && self.state.last_signal != Some(SignalType::Buy) {
            Some(SignalType::Buy) // 突破下轨，买入
        } else if price > upper_band && self.state.last_signal != Some(SignalType::Sell) {
            Some(SignalType::Sell) // 突破上轨，卖出
        } else {
            None
        };

        if let Some(sig_type) = signal_type {
            self.state.last_signal = Some(sig_type);

            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type: sig_type,
                price,
                quantity: self.config.quantity,
                confidence: 0.75,
                created_at: event.timestamp,
            });
        }

        None
    }
}

impl Strategy for SpotBollingerStrategy {
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
        self.state = SpotBollingerState::new();
    }
}
```

---

### 5. 现货RSI策略 (Spot RSI Strategy)

**文件**: `services/strategy-engine/src/domain/logic/spot/rsi.rs`

**说明**:
- RSI指标（相对强弱指数）
- RSI < 30 超卖买入，RSI > 70 超买卖出
- 适合震荡行情

**完整代码**:

```rust
//! # 现货RSI策略 (Spot RSI Strategy)
//!
//! 基于RSI指标的超买超卖策略。

use std::collections::VecDeque;

use rust_decimal::prelude::*;
use rust_decimal::Decimal;
use shared::event::market_event::{MarketEvent, MarketEventData};
use uuid::Uuid;

use crate::domain::logic::strategy_trait::{Strategy, StrategyMeta};
use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::{Signal, SignalType};

/// RSI策略配置
#[derive(Debug, Clone)]
pub struct SpotRsiConfig {
    /// RSI周期
    pub period: usize,
    /// 超卖阈值
    pub oversold_threshold: Decimal,
    /// 超买阈值
    pub overbought_threshold: Decimal,
    /// 交易数量
    pub quantity: Decimal,
}

impl Default for SpotRsiConfig {
    fn default() -> Self {
        Self {
            period: 14,
            oversold_threshold: Decimal::from(30),
            overbought_threshold: Decimal::from(70),
            quantity: Decimal::new(1, 3),
        }
    }
}

/// RSI策略状态
#[derive(Debug, Clone)]
pub struct SpotRsiState {
    /// 价格历史
    pub price_history: VecDeque<Decimal>,
    /// 平均涨幅
    pub avg_gain: Option<Decimal>,
    /// 平均跌幅
    pub avg_loss: Option<Decimal>,
    /// 上次信号
    pub last_signal: Option<SignalType>,
}

impl SpotRsiState {
    pub fn new() -> Self {
        Self {
            price_history: VecDeque::new(),
            avg_gain: None,
            avg_loss: None,
            last_signal: None,
        }
    }
}

impl Default for SpotRsiState {
    fn default() -> Self {
        Self::new()
    }
}

/// 现货RSI策略
pub struct SpotRsiStrategy {
    meta: StrategyMeta,
    config: SpotRsiConfig,
    state: SpotRsiState,
}

impl SpotRsiStrategy {
    pub fn new(instance_id: Uuid, symbol: String, config: SpotRsiConfig) -> Self {
        Self {
            meta: StrategyMeta {
                instance_id,
                strategy_type: "spot_rsi".to_string(),
                market_type: MarketType::Spot,
                symbol,
                is_active: false,
            },
            config,
            state: SpotRsiState::new(),
        }
    }

    /// 计算RSI信号
    fn calculate_signal(&mut self, event: &MarketEvent) -> Option<Signal> {
        let trade = match &event.data {
            MarketEventData::Trade(trade) => trade,
            _ => return None,
        };

        let price = trade.price;

        // 更新价格历史
        self.state.price_history.push_back(price);
        if self.state.price_history.len() > self.config.period + 1 {
            self.state.price_history.pop_front();
        }

        // 需要足够的数据
        if self.state.price_history.len() < 2 {
            return None;
        }

        // 计算价格变化
        let prev_price = self.state.price_history[self.state.price_history.len() - 2];
        let change = price - prev_price;

        let gain = if change > Decimal::ZERO { change } else { Decimal::ZERO };
        let loss = if change < Decimal::ZERO { -change } else { Decimal::ZERO };

        // 更新平均涨跌幅
        let period_decimal = Decimal::from(self.config.period);
        self.state.avg_gain = Some(match self.state.avg_gain {
            Some(avg) => (avg * (period_decimal - Decimal::ONE) + gain) / period_decimal,
            None => gain,
        });

        self.state.avg_loss = Some(match self.state.avg_loss {
            Some(avg) => (avg * (period_decimal - Decimal::ONE) + loss) / period_decimal,
            None => loss,
        });

        // 计算RSI
        let rsi = match (self.state.avg_gain, self.state.avg_loss) {
            (Some(avg_gain), Some(avg_loss)) if avg_loss != Decimal::ZERO => {
                let rs = avg_gain / avg_loss;
                Decimal::from(100) - (Decimal::from(100) / (Decimal::ONE + rs))
            }
            (Some(_), Some(avg_loss)) if avg_loss == Decimal::ZERO => Decimal::from(100),
            _ => return None,
        };

        // 判断信号
        let signal_type = if rsi < self.config.oversold_threshold
            && self.state.last_signal != Some(SignalType::Buy)
        {
            Some(SignalType::Buy) // 超卖，买入
        } else if rsi > self.config.overbought_threshold
            && self.state.last_signal != Some(SignalType::Sell)
        {
            Some(SignalType::Sell) // 超买，卖出
        } else {
            None
        };

        if let Some(sig_type) = signal_type {
            self.state.last_signal = Some(sig_type);

            return Some(Signal {
                id: Uuid::new_v4(),
                strategy_id: self.meta.instance_id,
                symbol: event.symbol.clone(),
                signal_type: sig_type,
                price,
                quantity: self.config.quantity,
                confidence: 0.7,
                created_at: event.timestamp,
            });
        }

        None
    }
}

impl Strategy for SpotRsiStrategy {
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
        self.state = SpotRsiState::new();
    }
}
```

---

## 说明

由于篇幅限制，我已经提供了5个现货策略的完整代码。

**剩余策略**（10个合约策略 + 3个跨平台套利策略）的代码模板类似，主要区别在于：

1. **合约策略**：增加杠杆、保证金、资金费率等参数
2. **跨平台套利**：需要多个交易所的价格数据

**下一步建议**：

1. 先实现这5个现货策略
2. 测试验证它们可以正常工作
3. 然后我再提供合约策略和套利策略的代码

这样可以：
- 逐步验证架构是否正确
- 及时发现和修复问题
- 避免一次性代码过多难以调试

---

**你想要我继续提供剩余13个策略的代码吗？**
