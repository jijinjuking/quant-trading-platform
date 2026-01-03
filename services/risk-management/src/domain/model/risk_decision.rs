//! # 风控决策 (Risk Decision)
//!
//! 结构化的风控返回结果，不是简单的 bool。

use std::fmt;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 风控决策结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskDecision {
    /// 通过
    Pass,
    /// 拒绝
    Reject {
        /// 拒绝代码
        code: String,
        /// 拒绝原因
        reason: String,
    },
}

impl RiskDecision {
    /// 创建通过决策
    pub fn pass() -> Self {
        RiskDecision::Pass
    }

    /// 创建拒绝决策
    pub fn reject(code: impl Into<String>, reason: impl Into<String>) -> Self {
        RiskDecision::Reject {
            code: code.into(),
            reason: reason.into(),
        }
    }

    /// 是否通过
    pub fn is_pass(&self) -> bool {
        matches!(self, RiskDecision::Pass)
    }

    /// 是否拒绝
    pub fn is_reject(&self) -> bool {
        matches!(self, RiskDecision::Reject { .. })
    }
}

/// 风控拒绝原因（可枚举、可读、可追踪）
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RiskRejectReason {
    /// 熔断中 - 连续拒绝触发
    CircuitBreakerActive {
        consecutive_rejects: u32,
        until: String,
    },
    /// 账户交易已禁用
    TradingDisabled,
    /// 单笔名义价值超限
    NotionalExceedsLimit {
        notional: Decimal,
        limit: Decimal,
    },
    /// 单 Symbol 仓位超限
    PositionExceedsLimit {
        symbol: String,
        current: Decimal,
        after: Decimal,
        limit: Decimal,
    },
    /// Symbol 不在白名单
    SymbolNotAllowed {
        symbol: String,
    },
    /// 数量低于最小值
    QuantityBelowMinimum {
        quantity: Decimal,
        minimum: Decimal,
    },
    /// 数量超过最大值
    QuantityAboveMaximum {
        quantity: Decimal,
        maximum: Decimal,
    },
    /// Side 非法
    InvalidSide {
        side: String,
    },
    /// Symbol 为空
    EmptySymbol,
}

impl fmt::Display for RiskRejectReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiskRejectReason::CircuitBreakerActive { consecutive_rejects, until } => {
                write!(f, "CIRCUIT_BREAKER: {} consecutive rejects, blocked until {}", 
                    consecutive_rejects, until)
            }
            RiskRejectReason::TradingDisabled => {
                write!(f, "TRADING_DISABLED: account trading is disabled")
            }
            RiskRejectReason::NotionalExceedsLimit { notional, limit } => {
                write!(f, "NOTIONAL_EXCEEDS_LIMIT: {} > {}", notional, limit)
            }
            RiskRejectReason::PositionExceedsLimit { symbol, current, after, limit } => {
                write!(f, "POSITION_EXCEEDS_LIMIT: {} current={} after={} limit={}", 
                    symbol, current, after, limit)
            }
            RiskRejectReason::SymbolNotAllowed { symbol } => {
                write!(f, "SYMBOL_NOT_ALLOWED: {}", symbol)
            }
            RiskRejectReason::QuantityBelowMinimum { quantity, minimum } => {
                write!(f, "QUANTITY_BELOW_MINIMUM: {} < {}", quantity, minimum)
            }
            RiskRejectReason::QuantityAboveMaximum { quantity, maximum } => {
                write!(f, "QUANTITY_ABOVE_MAXIMUM: {} > {}", quantity, maximum)
            }
            RiskRejectReason::InvalidSide { side } => {
                write!(f, "INVALID_SIDE: {}", side)
            }
            RiskRejectReason::EmptySymbol => {
                write!(f, "EMPTY_SYMBOL: symbol cannot be empty")
            }
        }
    }
}

impl std::error::Error for RiskRejectReason {}

impl RiskRejectReason {
    /// 转换为 RiskDecision
    pub fn to_decision(&self) -> RiskDecision {
        let code = match self {
            RiskRejectReason::CircuitBreakerActive { .. } => "CIRCUIT_BREAKER",
            RiskRejectReason::TradingDisabled => "TRADING_DISABLED",
            RiskRejectReason::NotionalExceedsLimit { .. } => "NOTIONAL_EXCEEDS_LIMIT",
            RiskRejectReason::PositionExceedsLimit { .. } => "POSITION_EXCEEDS_LIMIT",
            RiskRejectReason::SymbolNotAllowed { .. } => "SYMBOL_NOT_ALLOWED",
            RiskRejectReason::QuantityBelowMinimum { .. } => "QUANTITY_BELOW_MINIMUM",
            RiskRejectReason::QuantityAboveMaximum { .. } => "QUANTITY_ABOVE_MAXIMUM",
            RiskRejectReason::InvalidSide { .. } => "INVALID_SIDE",
            RiskRejectReason::EmptySymbol => "EMPTY_SYMBOL",
        };
        RiskDecision::reject(code, self.to_string())
    }
}
