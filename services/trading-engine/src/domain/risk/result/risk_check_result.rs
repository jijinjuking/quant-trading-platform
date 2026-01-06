//! # 风控检查结果 (Risk Check Result)
//!
//! 路径: services/trading-engine/src/domain/risk/result/risk_check_result.rs
//!
//! ## 职责
//! 定义风控检查的结构化返回结果，包括：
//! - 是否通过
//! - 明确的拒绝原因枚举
//! - 可扩展的拒绝详情
//!
//! ## 设计原则
//! - 不允许返回 bool，必须使用结构化结果
//! - 拒绝原因必须是枚举，便于扩展和匹配
//! - 支持携带额外的上下文信息

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 风控拒绝原因
///
/// 可扩展的枚举，每种拒绝原因对应一个变体。
/// 新增规则时，在此添加对应的拒绝原因。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RiskRejectReason {
    /// 持仓限制超限
    PositionLimitExceeded {
        /// 当前持仓
        current: Decimal,
        /// 请求数量
        requested: Decimal,
        /// 最大允许
        max_allowed: Decimal,
    },

    /// 未完成订单数量超限
    OpenOrderLimitExceeded {
        /// 当前未完成订单数
        current: usize,
        /// 最大允许
        max_allowed: usize,
    },

    /// 单笔订单数量超限
    OrderQuantityExceeded {
        /// 请求数量
        requested: Decimal,
        /// 最大允许
        max_allowed: Decimal,
    },

    /// 单笔订单数量过小
    OrderQuantityTooSmall {
        /// 请求数量
        requested: Decimal,
        /// 最小要求
        min_required: Decimal,
    },

    /// 名义价值超限
    NotionalValueExceeded {
        /// 名义价值
        notional: Decimal,
        /// 最大允许
        max_allowed: Decimal,
    },

    /// 交易对不在白名单
    SymbolNotAllowed {
        /// 请求的交易对
        symbol: String,
    },

    /// 余额不足
    InsufficientBalance {
        /// 资产
        asset: String,
        /// 可用余额
        available: Decimal,
        /// 需要数量
        required: Decimal,
    },

    /// 杠杆超限
    LeverageExceeded {
        /// 当前杠杆
        current: Decimal,
        /// 最大允许
        max_allowed: Decimal,
    },

    /// 回撤超限
    DrawdownExceeded {
        /// 当前回撤
        current: Decimal,
        /// 最大允许
        max_allowed: Decimal,
    },

    /// 冷却时间未到
    CooldownNotExpired {
        /// 剩余冷却时间（秒）
        remaining_seconds: u64,
    },

    /// 市价单名义金额超限（v1 估算）
    ///
    /// 用于区分市价单的估算名义金额超限，与限价单的精确名义金额超限
    MarketOrderNotionalExceeded {
        /// 交易对
        symbol: String,
        /// 估算名义金额（v1 使用固定价格估算）
        estimated_notional: Decimal,
        /// 最大允许
        max_allowed: Decimal,
    },

    /// 保证金率过低（v1.1 安全修补 - 强平前安全风控）
    ///
    /// 当账户保证金率低于临界阈值时，禁止新开仓，只允许减仓/平仓
    MarginRatioTooLow {
        /// 当前保证金率
        current_ratio: Decimal,
        /// 临界阈值
        critical_threshold: Decimal,
    },

    /// 自定义拒绝原因（用于扩展）
    Custom {
        /// 规则名称
        rule_name: String,
        /// 拒绝消息
        message: String,
    },
}

impl RiskRejectReason {
    /// 获取拒绝原因的代码（用于日志和监控）
    pub fn code(&self) -> &'static str {
        match self {
            RiskRejectReason::PositionLimitExceeded { .. } => "POSITION_LIMIT_EXCEEDED",
            RiskRejectReason::OpenOrderLimitExceeded { .. } => "OPEN_ORDER_LIMIT_EXCEEDED",
            RiskRejectReason::OrderQuantityExceeded { .. } => "ORDER_QTY_EXCEEDED",
            RiskRejectReason::OrderQuantityTooSmall { .. } => "ORDER_QTY_TOO_SMALL",
            RiskRejectReason::NotionalValueExceeded { .. } => "NOTIONAL_EXCEEDED",
            RiskRejectReason::SymbolNotAllowed { .. } => "SYMBOL_NOT_ALLOWED",
            RiskRejectReason::InsufficientBalance { .. } => "INSUFFICIENT_BALANCE",
            RiskRejectReason::LeverageExceeded { .. } => "LEVERAGE_EXCEEDED",
            RiskRejectReason::DrawdownExceeded { .. } => "DRAWDOWN_EXCEEDED",
            RiskRejectReason::CooldownNotExpired { .. } => "COOLDOWN_NOT_EXPIRED",
            RiskRejectReason::MarketOrderNotionalExceeded { .. } => "MARKET_ORDER_NOTIONAL_EXCEEDED",
            RiskRejectReason::MarginRatioTooLow { .. } => "MARGIN_RATIO_TOO_LOW",
            RiskRejectReason::Custom { .. } => "CUSTOM_REJECT",
        }
    }

    /// 获取人类可读的拒绝消息
    pub fn message(&self) -> String {
        match self {
            RiskRejectReason::PositionLimitExceeded { current, requested, max_allowed } => {
                format!(
                    "持仓限制超限: 当前={}, 请求={}, 最大允许={}",
                    current, requested, max_allowed
                )
            }
            RiskRejectReason::OpenOrderLimitExceeded { current, max_allowed } => {
                format!(
                    "未完成订单数量超限: 当前={}, 最大允许={}",
                    current, max_allowed
                )
            }
            RiskRejectReason::OrderQuantityExceeded { requested, max_allowed } => {
                format!(
                    "单笔订单数量超限: 请求={}, 最大允许={}",
                    requested, max_allowed
                )
            }
            RiskRejectReason::OrderQuantityTooSmall { requested, min_required } => {
                format!(
                    "单笔订单数量过小: 请求={}, 最小要求={}",
                    requested, min_required
                )
            }
            RiskRejectReason::NotionalValueExceeded { notional, max_allowed } => {
                format!(
                    "名义价值超限: 名义价值={}, 最大允许={}",
                    notional, max_allowed
                )
            }
            RiskRejectReason::SymbolNotAllowed { symbol } => {
                format!("交易对不在白名单: {}", symbol)
            }
            RiskRejectReason::InsufficientBalance { asset, available, required } => {
                format!(
                    "余额不足: 资产={}, 可用={}, 需要={}",
                    asset, available, required
                )
            }
            RiskRejectReason::LeverageExceeded { current, max_allowed } => {
                format!(
                    "杠杆超限: 当前={}, 最大允许={}",
                    current, max_allowed
                )
            }
            RiskRejectReason::DrawdownExceeded { current, max_allowed } => {
                format!(
                    "回撤超限: 当前={}, 最大允许={}",
                    current, max_allowed
                )
            }
            RiskRejectReason::CooldownNotExpired { remaining_seconds } => {
                format!("冷却时间未到: 剩余 {} 秒", remaining_seconds)
            }
            RiskRejectReason::MarketOrderNotionalExceeded { symbol, estimated_notional, max_allowed } => {
                format!(
                    "市价单估算名义金额超限: 交易对={}, 估算金额={}, 最大允许={}",
                    symbol, estimated_notional, max_allowed
                )
            }
            RiskRejectReason::MarginRatioTooLow { current_ratio, critical_threshold } => {
                format!(
                    "保证金率过低，禁止开仓: 当前={}, 临界阈值={}",
                    current_ratio, critical_threshold
                )
            }
            RiskRejectReason::Custom { rule_name, message } => {
                format!("[{}] {}", rule_name, message)
            }
        }
    }
}

/// 风控检查结果
///
/// 结构化的风控检查返回值，不允许使用 bool。
#[derive(Debug, Clone)]
pub enum RiskCheckResult {
    /// 检查通过
    Passed,
    /// 检查拒绝
    Rejected(RiskRejectReason),
}

impl RiskCheckResult {
    /// 创建通过结果
    pub fn passed() -> Self {
        RiskCheckResult::Passed
    }

    /// 创建拒绝结果
    pub fn rejected(reason: RiskRejectReason) -> Self {
        RiskCheckResult::Rejected(reason)
    }

    /// 是否通过
    pub fn is_passed(&self) -> bool {
        matches!(self, RiskCheckResult::Passed)
    }

    /// 是否被拒绝
    pub fn is_rejected(&self) -> bool {
        matches!(self, RiskCheckResult::Rejected(_))
    }

    /// 获取拒绝原因（如果被拒绝）
    pub fn reject_reason(&self) -> Option<&RiskRejectReason> {
        match self {
            RiskCheckResult::Rejected(reason) => Some(reason),
            RiskCheckResult::Passed => None,
        }
    }

    /// 转换为 anyhow::Result
    ///
    /// 通过返回 Ok(()), 拒绝返回 Err
    pub fn into_result(self) -> anyhow::Result<()> {
        match self {
            RiskCheckResult::Passed => Ok(()),
            RiskCheckResult::Rejected(reason) => {
                Err(anyhow::anyhow!("{}: {}", reason.code(), reason.message()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_passed_result() {
        let result = RiskCheckResult::passed();
        assert!(result.is_passed());
        assert!(!result.is_rejected());
        assert!(result.reject_reason().is_none());
    }

    #[test]
    fn test_rejected_result() {
        let result = RiskCheckResult::rejected(RiskRejectReason::OpenOrderLimitExceeded {
            current: 10,
            max_allowed: 5,
        });
        assert!(!result.is_passed());
        assert!(result.is_rejected());
        assert!(result.reject_reason().is_some());
    }

    #[test]
    fn test_reject_reason_code() {
        let reason = RiskRejectReason::PositionLimitExceeded {
            current: Decimal::from(100),
            requested: Decimal::from(50),
            max_allowed: Decimal::from(100),
        };
        assert_eq!(reason.code(), "POSITION_LIMIT_EXCEEDED");
    }

    #[test]
    fn test_into_result() {
        let passed = RiskCheckResult::passed();
        assert!(passed.into_result().is_ok());

        let rejected = RiskCheckResult::rejected(RiskRejectReason::SymbolNotAllowed {
            symbol: "INVALID".to_string(),
        });
        assert!(rejected.into_result().is_err());
    }
}
