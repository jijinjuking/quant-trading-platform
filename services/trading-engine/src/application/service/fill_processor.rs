//! # 成交回报处理服务 (Fill Processor Service)
//!
//! 路径: services/trading-engine/src/application/service/fill_processor.rs
//!
//! ## 职责
//! 处理成交回报事件，驱动 RiskState 修正：
//! - 部分成交 → update_position（不移除 open_order）
//! - 全部成交 → update_position + remove_open_order
//! - 撤单 → remove_open_order
//!
//! ## 架构位置
//! - 所属层级: Application Layer
//! - 依赖端口: RiskStatePort (domain/port)
//!
//! ## 设计原则
//! - 不修改 RiskStatePort 接口
//! - 不修改 OrderRiskAdapter
//! - 只做流程编排，不包含业务逻辑

use std::sync::Arc;

use rust_decimal::Decimal;
use tracing::{debug, info, warn};

use crate::domain::model::execution_fill::{
    CancelReason, ExecutionFill, FillSide, FillType, OrderAccepted, OrderCanceled,
};
use crate::domain::port::risk_state_port::{RiskOpenOrder, RiskStatePort};

/// 成交回报处理服务
///
/// 负责将成交回报事件转换为 RiskState 修正操作。
/// 这是 ExecutionService 的辅助服务，专门处理成交回报。
pub struct FillProcessor {
    /// 风控状态端口
    risk_state: Arc<dyn RiskStatePort>,
}

impl FillProcessor {
    /// 创建成交回报处理服务
    pub fn new(risk_state: Arc<dyn RiskStatePort>) -> Self {
        Self { risk_state }
    }

    /// 处理订单接受事件
    ///
    /// 当订单被交易所接受时调用：
    /// - 添加到 open_orders
    ///
    /// 注意：订单接受 ≠ 订单成交
    pub async fn on_order_accepted(&self, event: &OrderAccepted) {
        let open_order = RiskOpenOrder {
            order_id: event.order_id.clone(),
            symbol: event.symbol.clone(),
            side: event.side.as_str().to_string(),
            quantity: event.quantity,
            price: event.price.unwrap_or(Decimal::ZERO),
            created_at: event.accepted_at.timestamp_millis(),
        };

        self.risk_state.add_open_order(open_order).await;

        info!(
            order_id = %event.order_id,
            symbol = %event.symbol,
            side = %event.side.as_str(),
            quantity = %event.quantity,
            "Order accepted, added to open_orders"
        );
    }

    /// 处理成交回报事件
    ///
    /// 根据成交类型执行不同操作：
    /// - 部分成交 → update_position（不移除 open_order）
    /// - 全部成交 → update_position + remove_open_order
    pub async fn on_execution_fill(&self, fill: &ExecutionFill) {
        // 1. 更新持仓
        let delta = fill.position_delta();
        self.risk_state
            .update_position(&fill.symbol, delta, fill.fill_price)
            .await;

        debug!(
            order_id = %fill.order_id,
            symbol = %fill.symbol,
            side = %fill.side.as_str(),
            filled_qty = %fill.filled_quantity,
            price = %fill.fill_price,
            delta = %delta,
            "Position updated from fill"
        );

        // 2. 如果全部成交，移除 open_order
        if fill.is_full() {
            self.risk_state.remove_open_order(&fill.order_id).await;

            info!(
                order_id = %fill.order_id,
                symbol = %fill.symbol,
                cumulative_qty = %fill.cumulative_quantity,
                "Order fully filled, removed from open_orders"
            );
        } else {
            debug!(
                order_id = %fill.order_id,
                remaining = %fill.remaining_quantity(),
                "Order partially filled, still in open_orders"
            );
        }
    }

    /// 处理订单取消事件
    ///
    /// 当订单被取消时调用：
    /// - 移除 open_order
    /// - 如果有部分成交，持仓已在之前的 fill 事件中更新
    pub async fn on_order_canceled(&self, event: &OrderCanceled) {
        // 移除 open_order
        self.risk_state.remove_open_order(&event.order_id).await;

        if event.has_unfilled() {
            info!(
                order_id = %event.order_id,
                symbol = %event.symbol,
                filled = %event.filled_quantity,
                unfilled = %event.unfilled_quantity(),
                reason = ?event.reason,
                "Order canceled with unfilled quantity, removed from open_orders"
            );
        } else {
            info!(
                order_id = %event.order_id,
                symbol = %event.symbol,
                reason = ?event.reason,
                "Order canceled (fully filled before cancel), removed from open_orders"
            );
        }
    }

    /// 模拟市价单立即成交
    ///
    /// 市价单通常会立即成交，此方法模拟这个过程：
    /// 1. 先 add_open_order
    /// 2. 立即 update_position + remove_open_order
    ///
    /// # 参数
    /// - `order_id`: 订单 ID
    /// - `symbol`: 交易对
    /// - `side`: 方向
    /// - `quantity`: 数量
    /// - `fill_price`: 成交价格
    pub async fn simulate_market_order_fill(
        &self,
        order_id: &str,
        symbol: &str,
        side: FillSide,
        quantity: Decimal,
        fill_price: Decimal,
    ) {
        // 1. 创建并处理 OrderAccepted
        let accepted = OrderAccepted::new(
            order_id.to_string(),
            symbol.to_string(),
            side,
            "MARKET",
            quantity,
            None,
        );
        self.on_order_accepted(&accepted).await;

        // 2. 创建并处理 ExecutionFill (全部成交)
        let fill = ExecutionFill::full(
            order_id.to_string(),
            format!("sim_{}", uuid::Uuid::new_v4()), // 生成唯一 trade_id
            symbol.to_string(),
            side,
            quantity,
            fill_price,
        );
        self.on_execution_fill(&fill).await;

        info!(
            order_id = %order_id,
            symbol = %symbol,
            side = %side.as_str(),
            quantity = %quantity,
            price = %fill_price,
            "Market order simulated: accepted → filled"
        );
    }

    /// 模拟限价单部分成交
    ///
    /// # 参数
    /// - `order_id`: 订单 ID
    /// - `symbol`: 交易对
    /// - `side`: 方向
    /// - `original_qty`: 原始数量
    /// - `filled_qty`: 本次成交数量
    /// - `cumulative_qty`: 累计成交数量
    /// - `fill_price`: 成交价格
    pub async fn simulate_partial_fill(
        &self,
        order_id: &str,
        symbol: &str,
        side: FillSide,
        original_qty: Decimal,
        filled_qty: Decimal,
        cumulative_qty: Decimal,
        fill_price: Decimal,
    ) {
        let fill = ExecutionFill::partial(
            order_id.to_string(),
            format!("sim_{}", uuid::Uuid::new_v4()), // 生成唯一 trade_id
            symbol.to_string(),
            side,
            filled_qty,
            fill_price,
            cumulative_qty,
            original_qty,
        );
        self.on_execution_fill(&fill).await;
    }

    /// 模拟订单取消（未成交）
    pub async fn simulate_cancel_unfilled(
        &self,
        order_id: &str,
        symbol: &str,
        side: FillSide,
        original_qty: Decimal,
    ) {
        let cancel = OrderCanceled::new(
            order_id.to_string(),
            symbol.to_string(),
            side,
            original_qty,
            Decimal::ZERO, // 未成交
            CancelReason::UserRequested,
        );
        self.on_order_canceled(&cancel).await;
    }

    /// 模拟订单取消（部分成交后取消）
    pub async fn simulate_cancel_partial(
        &self,
        order_id: &str,
        symbol: &str,
        side: FillSide,
        original_qty: Decimal,
        filled_qty: Decimal,
    ) {
        let cancel = OrderCanceled::new(
            order_id.to_string(),
            symbol.to_string(),
            side,
            original_qty,
            filled_qty,
            CancelReason::UserRequested,
        );
        self.on_order_canceled(&cancel).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::risk::inmemory_risk_state::InMemoryRiskStateAdapter;

    fn dec(s: &str) -> Decimal {
        s.parse().unwrap_or_default()
    }

    /// 创建测试用的 FillProcessor
    fn create_test_processor() -> (FillProcessor, Arc<InMemoryRiskStateAdapter>) {
        let risk_state = Arc::new(InMemoryRiskStateAdapter::new());
        let processor = FillProcessor::new(risk_state.clone());
        (processor, risk_state)
    }

    // ========== 测试：订单接受 ==========

    #[tokio::test]
    async fn test_order_accepted_adds_to_open_orders() {
        let (processor, risk_state) = create_test_processor();

        // 订单被接受
        let accepted = OrderAccepted::new(
            "order001".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            "LIMIT",
            dec("0.5"),
            Some(dec("50000")),
        );
        processor.on_order_accepted(&accepted).await;

        // 验证 open_orders
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.open_orders.len(), 1);
        assert_eq!(snapshot.open_orders[0].order_id, "order001");
        assert_eq!(snapshot.open_orders[0].symbol, "BTCUSDT");
        assert_eq!(snapshot.open_orders[0].side, "BUY");
        assert_eq!(snapshot.open_orders[0].quantity, dec("0.5"));
    }

    // ========== 测试：部分成交 ==========

    #[tokio::test]
    async fn test_partial_fill_updates_position_keeps_open_order() {
        let (processor, risk_state) = create_test_processor();

        // 1. 先添加订单
        let accepted = OrderAccepted::new(
            "order002".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            "LIMIT",
            dec("1.0"),
            Some(dec("50000")),
        );
        processor.on_order_accepted(&accepted).await;

        // 2. 部分成交 0.3
        let fill = ExecutionFill::partial(
            "order002".to_string(),
            "trade002".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            dec("0.3"),      // 本次成交
            dec("50000"),    // 成交价
            dec("0.3"),      // 累计成交
            dec("1.0"),      // 原始数量
        );
        processor.on_execution_fill(&fill).await;

        // 验证：持仓已更新
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.3"));

        // 验证：open_order 仍然存在
        assert_eq!(snapshot.open_orders.len(), 1);
        assert_eq!(snapshot.open_orders[0].order_id, "order002");
    }

    #[tokio::test]
    async fn test_multiple_partial_fills() {
        let (processor, risk_state) = create_test_processor();

        // 1. 添加订单
        let accepted = OrderAccepted::new(
            "order003".to_string(),
            "ETHUSDT".to_string(),
            FillSide::Buy,
            "LIMIT",
            dec("10.0"),
            Some(dec("3000")),
        );
        processor.on_order_accepted(&accepted).await;

        // 2. 第一次部分成交 3.0
        processor.simulate_partial_fill(
            "order003",
            "ETHUSDT",
            FillSide::Buy,
            dec("10.0"),     // 原始
            dec("3.0"),      // 本次
            dec("3.0"),      // 累计
            dec("3000"),
        ).await;

        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("ETHUSDT"), dec("3.0"));
        assert_eq!(snapshot.open_orders.len(), 1);

        // 3. 第二次部分成交 4.0
        processor.simulate_partial_fill(
            "order003",
            "ETHUSDT",
            FillSide::Buy,
            dec("10.0"),     // 原始
            dec("4.0"),      // 本次
            dec("7.0"),      // 累计
            dec("3010"),
        ).await;

        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("ETHUSDT"), dec("7.0"));
        assert_eq!(snapshot.open_orders.len(), 1);
    }

    // ========== 测试：全部成交 ==========

    #[tokio::test]
    async fn test_full_fill_updates_position_removes_open_order() {
        let (processor, risk_state) = create_test_processor();

        // 1. 添加订单
        let accepted = OrderAccepted::new(
            "order004".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            "LIMIT",
            dec("0.5"),
            Some(dec("50000")),
        );
        processor.on_order_accepted(&accepted).await;

        // 2. 全部成交
        let fill = ExecutionFill::full(
            "order004".to_string(),
            "trade004".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            dec("0.5"),
            dec("50000"),
        );
        processor.on_execution_fill(&fill).await;

        // 验证：持仓已更新
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.5"));

        // 验证：open_order 已移除
        assert_eq!(snapshot.open_orders.len(), 0);
    }

    #[tokio::test]
    async fn test_partial_then_full_fill() {
        let (processor, risk_state) = create_test_processor();

        // 1. 添加订单
        let accepted = OrderAccepted::new(
            "order005".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Sell,
            "LIMIT",
            dec("1.0"),
            Some(dec("55000")),
        );
        processor.on_order_accepted(&accepted).await;

        // 2. 部分成交 0.6
        let partial = ExecutionFill::partial(
            "order005".to_string(),
            "trade005".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Sell,
            dec("0.6"),
            dec("55000"),
            dec("0.6"),
            dec("1.0"),
        );
        processor.on_execution_fill(&partial).await;

        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("-0.6"));
        assert_eq!(snapshot.open_orders.len(), 1);

        // 3. 剩余部分成交（变成全部成交）
        let full = ExecutionFill {
            fill_type: FillType::Full,
            filled_quantity: dec("0.4"),
            cumulative_quantity: dec("1.0"),
            ..partial.clone()
        };
        processor.on_execution_fill(&full).await;

        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("-1.0"));
        assert_eq!(snapshot.open_orders.len(), 0);
    }

    // ========== 测试：撤单（未成交） ==========

    #[tokio::test]
    async fn test_cancel_unfilled_removes_open_order() {
        let (processor, risk_state) = create_test_processor();

        // 1. 添加订单
        let accepted = OrderAccepted::new(
            "order006".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            "LIMIT",
            dec("0.5"),
            Some(dec("45000")),
        );
        processor.on_order_accepted(&accepted).await;

        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.open_orders.len(), 1);

        // 2. 撤单（未成交）
        processor.simulate_cancel_unfilled(
            "order006",
            "BTCUSDT",
            FillSide::Buy,
            dec("0.5"),
        ).await;

        // 验证：open_order 已移除，持仓不变
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.open_orders.len(), 0);
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0"));
    }

    // ========== 测试：撤单（部分成交后） ==========

    #[tokio::test]
    async fn test_cancel_after_partial_fill() {
        let (processor, risk_state) = create_test_processor();

        // 1. 添加订单
        let accepted = OrderAccepted::new(
            "order007".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Buy,
            "LIMIT",
            dec("1.0"),
            Some(dec("50000")),
        );
        processor.on_order_accepted(&accepted).await;

        // 2. 部分成交 0.4
        processor.simulate_partial_fill(
            "order007",
            "BTCUSDT",
            FillSide::Buy,
            dec("1.0"),
            dec("0.4"),
            dec("0.4"),
            dec("50000"),
        ).await;

        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.4"));
        assert_eq!(snapshot.open_orders.len(), 1);

        // 3. 撤单（剩余 0.6 未成交）
        processor.simulate_cancel_partial(
            "order007",
            "BTCUSDT",
            FillSide::Buy,
            dec("1.0"),
            dec("0.4"),
        ).await;

        // 验证：open_order 已移除，持仓保持 0.4
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.open_orders.len(), 0);
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.4"));
    }

    // ========== 测试：市价单模拟 ==========

    #[tokio::test]
    async fn test_simulate_market_order_fill() {
        let (processor, risk_state) = create_test_processor();

        // 模拟市价单立即成交
        processor.simulate_market_order_fill(
            "market001",
            "BTCUSDT",
            FillSide::Buy,
            dec("0.1"),
            dec("52000"),
        ).await;

        // 验证：持仓已更新，open_order 已移除
        let snapshot = risk_state.get_snapshot().await.unwrap();
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.1"));
        assert_eq!(snapshot.open_orders.len(), 0);
    }

    // ========== 测试：RiskState 可被 OrderRiskAdapter 正确读取 ==========

    #[tokio::test]
    async fn test_risk_state_readable_after_fills() {
        let (processor, risk_state) = create_test_processor();

        // 设置初始余额
        risk_state.set_balance("USDT", dec("10000"), dec("0"));

        // 执行一系列操作
        processor.simulate_market_order_fill(
            "order_a",
            "BTCUSDT",
            FillSide::Buy,
            dec("0.1"),
            dec("50000"),
        ).await;

        processor.simulate_market_order_fill(
            "order_b",
            "ETHUSDT",
            FillSide::Buy,
            dec("2.0"),
            dec("3000"),
        ).await;

        // 添加一个未成交的限价单
        let accepted = OrderAccepted::new(
            "order_c".to_string(),
            "BTCUSDT".to_string(),
            FillSide::Sell,
            "LIMIT",
            dec("0.05"),
            Some(dec("55000")),
        );
        processor.on_order_accepted(&accepted).await;

        // 验证 RiskState 快照
        let snapshot = risk_state.get_snapshot().await.unwrap();

        // 余额
        assert_eq!(snapshot.get_free_balance("USDT"), dec("10000"));

        // 持仓
        assert_eq!(snapshot.get_position_qty("BTCUSDT"), dec("0.1"));
        assert_eq!(snapshot.get_position_qty("ETHUSDT"), dec("2.0"));

        // 未完成订单
        assert_eq!(snapshot.open_orders.len(), 1);
        assert_eq!(snapshot.get_open_order_count("BTCUSDT"), 1);
        assert_eq!(snapshot.get_pending_sell_qty("BTCUSDT"), dec("0.05"));
    }
}
