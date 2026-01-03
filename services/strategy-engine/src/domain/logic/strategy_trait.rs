//! # 统一策略 Trait (Strategy Trait)
//!
//! 定义所有策略必须实现的统一接口。
//! 为高频交易预留 tick 级别处理能力。

use shared::event::market_event::MarketEvent;
use uuid::Uuid;

use crate::domain::model::market_type::MarketType;
use crate::domain::model::signal::Signal;

/// 策略元信息
#[derive(Debug, Clone)]
pub struct StrategyMeta {
    /// 策略实例 ID
    pub instance_id: Uuid,
    /// 策略类型名称
    pub strategy_type: String,
    /// 市场类型
    pub market_type: MarketType,
    /// 交易对
    pub symbol: String,
    /// 是否激活
    pub is_active: bool,
}

/// 统一策略 Trait
///
/// 所有策略（现货/合约）都必须实现此 trait。
/// 设计考虑：
/// - `on_market_event`: 标准行情事件处理
/// - `on_tick`: 高频 tick 处理（预留）
/// - `reset`: 重置策略状态
pub trait Strategy: Send + Sync {
    /// 获取策略元信息
    fn meta(&self) -> &StrategyMeta;

    /// 获取策略元信息（可变）
    fn meta_mut(&mut self) -> &mut StrategyMeta;

    /// 处理行情事件，返回交易信号
    ///
    /// # 参数
    /// - `event`: 行情事件
    ///
    /// # 返回
    /// - `Some(Signal)`: 产生交易信号
    /// - `None`: 无信号
    fn on_market_event(&mut self, event: &MarketEvent) -> Option<Signal>;

    /// 高频 tick 处理（预留接口）
    ///
    /// 默认实现：不处理，返回 None
    /// 高频策略可覆盖此方法实现微秒级响应
    #[allow(unused_variables)]
    fn on_tick(&mut self, price: rust_decimal::Decimal, timestamp_us: i64) -> Option<Signal> {
        None
    }

    /// 重置策略状态
    ///
    /// 用于策略重启或参数变更后的状态清理
    fn reset(&mut self);

    /// 激活策略
    fn activate(&mut self) {
        self.meta_mut().is_active = true;
    }

    /// 停用策略
    fn deactivate(&mut self) {
        self.meta_mut().is_active = false;
    }

    /// 是否激活
    fn is_active(&self) -> bool {
        self.meta().is_active
    }
}
