//! # 策略执行器适配器 (Strategy Executor Adapter)
//!
//! 将旧的 Strategy trait 适配到新的 StrategyExecutorPort trait。
//!
//! ## 设计目的
//! - 保留现有策略代码
//! - 适配到新的架构
//! - 提供统一的执行接口

use std::sync::Arc;

use anyhow::Result;
use parking_lot::RwLock;
use shared::event::market_event::{MarketEvent, MarketEventData, TradeData};

use crate::domain::logic::strategy_trait::Strategy;
use crate::domain::model::signal::SignalType;
use crate::domain::model::strategy_runtime::{ExecutionRequest, ExecutionResult, TradeIntent};
use crate::domain::port::strategy_executor_port::StrategyExecutorPort;

/// 策略执行器适配器
///
/// 将实现了 Strategy trait 的策略适配到 StrategyExecutorPort。
pub struct StrategyExecutorAdapter<S: Strategy> {
    /// 策略实例（使用 RwLock 实现内部可变性）
    strategy: Arc<RwLock<S>>,
}

impl<S: Strategy> StrategyExecutorAdapter<S> {
    /// 创建适配器
    pub fn new(strategy: S) -> Self {
        Self {
            strategy: Arc::new(RwLock::new(strategy)),
        }
    }

    /// 将 ExecutionRequest 转换为 MarketEvent
    fn request_to_market_event(&self, request: &ExecutionRequest) -> MarketEvent {
        MarketEvent {
            event_type: shared::event::market_event::MarketEventType::Trade,
            exchange: "binance".to_string(),
            symbol: request.symbol.clone(),
            timestamp: request.timestamp,
            data: MarketEventData::Trade(TradeData {
                trade_id: request.request_id.to_string(),
                price: request.price,
                quantity: request.quantity,
                is_buyer_maker: request.is_buyer_maker,
            }),
        }
    }
}

impl<S: Strategy + Send + Sync + 'static> StrategyExecutorPort for StrategyExecutorAdapter<S> {
    fn execute(&self, request: &ExecutionRequest) -> Result<ExecutionResult> {
        let start = std::time::Instant::now();

        // 转换请求为行情事件
        let market_event = self.request_to_market_event(request);

        // 执行策略
        let signal = {
            let mut strategy = self.strategy.write();
            strategy.on_market_event(&market_event)
        };

        // 转换信号为交易意图
        let intent = signal.and_then(|sig| {
            let side = match sig.signal_type {
                SignalType::Buy => shared::types::order::OrderSide::Buy,
                SignalType::Sell => shared::types::order::OrderSide::Sell,
                SignalType::Hold => return None, // Hold信号不生成交易意图
            };

            Some(TradeIntent {
                id: sig.id,
                strategy_id: sig.strategy_id,
                symbol: sig.symbol,
                side,
                quantity: sig.quantity,
                price: Some(sig.price),
                order_type: shared::types::order::OrderType::Limit,
                confidence: sig.confidence,
                created_at: sig.created_at,
            })
        });

        let execution_time_us = start.elapsed().as_micros() as u64;

        Ok(ExecutionResult {
            request_id: request.request_id,
            has_intent: intent.is_some(),
            intent,
            execution_time_us,
            error: None,
        })
    }

    fn reset(&self) -> Result<()> {
        let mut strategy = self.strategy.write();
        strategy.reset();
        Ok(())
    }

    fn state_snapshot(&self) -> Result<serde_json::Value> {
        let strategy = self.strategy.read();
        let meta = strategy.meta();

        Ok(serde_json::json!({
            "instance_id": meta.instance_id,
            "strategy_type": meta.strategy_type,
            "market_type": format!("{:?}", meta.market_type),
            "symbol": meta.symbol,
            "is_active": meta.is_active,
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use rust_decimal::Decimal;
    use uuid::Uuid;

    // 测试用的简单策略
    struct TestStrategy {
        instance_id: Uuid,
        call_count: u32,
    }

    impl Strategy for TestStrategy {
        fn meta(&self) -> &crate::domain::logic::strategy_trait::StrategyMeta {
            unimplemented!()
        }

        fn meta_mut(&mut self) -> &mut crate::domain::logic::strategy_trait::StrategyMeta {
            unimplemented!()
        }

        fn on_market_event(&mut self, _event: &MarketEvent) -> Option<crate::domain::model::signal::Signal> {
            self.call_count += 1;
            None
        }

        fn reset(&mut self) {
            self.call_count = 0;
        }
    }

    #[test]
    fn test_adapter_execute() {
        let strategy = TestStrategy {
            instance_id: Uuid::new_v4(),
            call_count: 0,
        };
        let adapter = StrategyExecutorAdapter::new(strategy);

        let request = ExecutionRequest {
            request_id: Uuid::new_v4(),
            symbol: "BTCUSDT".to_string(),
            price: Decimal::new(50000, 0),
            quantity: Decimal::new(1, 3),
            timestamp: Utc::now(),
            is_buyer_maker: false,
        };

        let result = adapter.execute(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_adapter_reset() {
        let strategy = TestStrategy {
            instance_id: Uuid::new_v4(),
            call_count: 0,
        };
        let adapter = StrategyExecutorAdapter::new(strategy);

        let result = adapter.reset();
        assert!(result.is_ok());
    }
}
