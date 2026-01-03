//! # 风控检查器 (Risk Checker)
//!
//! 核心风控规则检查服务，实现所有风控规则。

use rust_decimal::Decimal;

use crate::domain::model::risk_decision::{RiskDecision, RiskRejectReason};

/// 风控检查器配置
#[derive(Debug, Clone)]
pub struct RiskCheckerConfig {
    /// 允许的交易对白名单（空 = 允许所有）
    pub allowed_symbols: Vec<String>,
    /// 最小数量
    pub min_quantity: Decimal,
    /// 最大数量
    pub max_quantity: Decimal,
    /// 最大名义价值
    pub max_notional: Decimal,
}

/// 风控检查器
///
/// 实现所有风控规则，返回结构化的 RiskDecision。
pub struct RiskChecker {
    config: RiskCheckerConfig,
}

impl RiskChecker {
    /// 创建风控检查器
    pub fn new(config: RiskCheckerConfig) -> Self {
        Self { config }
    }

    /// 执行完整风控检查
    ///
    /// # 规则执行顺序（至少 3 条）
    /// 1. Symbol 非空检查
    /// 2. Side 合法性检查
    /// 3. Symbol 白名单检查
    /// 4. 数量范围检查
    /// 5. 名义价值检查
    pub fn check(
        &self,
        symbol: &str,
        side: &str,
        quantity: Decimal,
        price: Decimal,
    ) -> RiskDecision {
        // 规则 1: Symbol 非空检查
        if let Err(reason) = self.check_symbol_not_empty(symbol) {
            return reason.to_decision();
        }

        // 规则 2: Side 合法性检查
        if let Err(reason) = self.check_side_valid(side) {
            return reason.to_decision();
        }

        // 规则 3: Symbol 白名单检查
        if let Err(reason) = self.check_symbol_allowed(symbol) {
            return reason.to_decision();
        }

        // 规则 4: 数量范围检查
        if let Err(reason) = self.check_quantity_range(quantity) {
            return reason.to_decision();
        }

        // 规则 5: 名义价值检查
        if let Err(reason) = self.check_notional_limit(quantity, price) {
            return reason.to_decision();
        }

        // 所有规则通过
        RiskDecision::pass()
    }

    /// 规则 1: Symbol 非空检查
    fn check_symbol_not_empty(&self, symbol: &str) -> Result<(), RiskRejectReason> {
        if symbol.trim().is_empty() {
            return Err(RiskRejectReason::EmptySymbol);
        }
        Ok(())
    }

    /// 规则 2: Side 合法性检查
    fn check_side_valid(&self, side: &str) -> Result<(), RiskRejectReason> {
        let side_lower = side.to_lowercase();
        if side_lower != "buy" && side_lower != "sell" {
            return Err(RiskRejectReason::InvalidSide {
                side: side.to_string(),
            });
        }
        Ok(())
    }

    /// 规则 3: Symbol 白名单检查
    fn check_symbol_allowed(&self, symbol: &str) -> Result<(), RiskRejectReason> {
        if self.config.allowed_symbols.is_empty() {
            return Ok(()); // 空列表表示允许所有
        }
        let symbol_upper = symbol.to_uppercase();
        if !self.config.allowed_symbols.iter().any(|s| s.eq_ignore_ascii_case(&symbol_upper)) {
            return Err(RiskRejectReason::SymbolNotAllowed {
                symbol: symbol.to_string(),
            });
        }
        Ok(())
    }

    /// 规则 4: 数量范围检查
    fn check_quantity_range(&self, quantity: Decimal) -> Result<(), RiskRejectReason> {
        if quantity < self.config.min_quantity {
            return Err(RiskRejectReason::QuantityBelowMinimum {
                quantity,
                minimum: self.config.min_quantity,
            });
        }
        if quantity > self.config.max_quantity {
            return Err(RiskRejectReason::QuantityAboveMaximum {
                quantity,
                maximum: self.config.max_quantity,
            });
        }
        Ok(())
    }

    /// 规则 5: 名义价值检查
    fn check_notional_limit(&self, quantity: Decimal, price: Decimal) -> Result<(), RiskRejectReason> {
        let notional = quantity * price;
        if notional > self.config.max_notional {
            return Err(RiskRejectReason::NotionalExceedsLimit {
                notional,
                limit: self.config.max_notional,
            });
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_checker() -> RiskChecker {
        RiskChecker::new(RiskCheckerConfig {
            allowed_symbols: vec!["BTCUSDT".to_string(), "ETHUSDT".to_string()],
            min_quantity: Decimal::new(1, 4), // 0.0001
            max_quantity: Decimal::new(10, 0), // 10
            max_notional: Decimal::new(100000, 0), // 100000
        })
    }

    #[test]
    fn test_empty_symbol_rejected() {
        let checker = create_test_checker();
        let result = checker.check("", "buy", Decimal::new(1, 0), Decimal::new(50000, 0));
        assert!(result.is_reject());
    }

    #[test]
    fn test_invalid_side_rejected() {
        let checker = create_test_checker();
        let result = checker.check("BTCUSDT", "invalid", Decimal::new(1, 0), Decimal::new(50000, 0));
        assert!(result.is_reject());
    }

    #[test]
    fn test_symbol_not_allowed_rejected() {
        let checker = create_test_checker();
        let result = checker.check("DOGEUSDT", "buy", Decimal::new(1, 0), Decimal::new(1, 0));
        assert!(result.is_reject());
    }

    #[test]
    fn test_quantity_below_minimum_rejected() {
        let checker = create_test_checker();
        let result = checker.check("BTCUSDT", "buy", Decimal::new(1, 6), Decimal::new(50000, 0));
        assert!(result.is_reject());
    }

    #[test]
    fn test_valid_order_passes() {
        let checker = create_test_checker();
        let result = checker.check("BTCUSDT", "buy", Decimal::new(1, 2), Decimal::new(50000, 0));
        assert!(result.is_pass());
    }
}
