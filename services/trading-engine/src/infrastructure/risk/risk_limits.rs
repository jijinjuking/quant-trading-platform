//! # 风险限额配置 (Risk Limits Configuration)
//!
//! 路径: services/trading-engine/src/infrastructure/risk/risk_limits.rs
//!
//! ## 职责
//! 收敛所有数值型风险参数，为 v2 参数化做准备。
//! v1 阶段使用写死默认值，v2 可从数据库/配置中心加载。

use std::collections::HashSet;
use rust_decimal::Decimal;

// ============================================================================
// RiskLimits - 风险参数结构体
// ============================================================================

/// 风险限额参数
///
/// 收敛所有数值型风险参数，为 v2 参数化做准备。
/// v1 阶段使用写死默认值，v2 可从数据库/配置中心加载。
#[derive(Debug, Clone)]
pub struct RiskLimits {
    // === 规则 A: 单笔下单限制 ===
    /// 单笔最小数量
    pub min_order_qty: Decimal,
    /// 单笔最大数量
    pub max_order_qty: Decimal,
    /// 单笔最大名义金额（USDT）
    pub max_order_notional: Decimal,
    /// 单笔最大占用可用余额比例 (0.0 - 1.0)
    pub max_balance_usage_ratio: Decimal,

    // === 规则 B: Symbol 维度限制 ===
    /// 单交易对最大持仓（含未完成订单）
    pub max_position_per_symbol: Decimal,
    /// 单交易对最大未完成订单数
    pub max_open_orders_per_symbol: usize,

    // === 规则 C: 账户总风险敞口 ===
    /// 账户最大总名义敞口（USDT）
    pub max_total_exposure: Decimal,
    /// 全局最大未完成订单数
    pub max_total_open_orders: usize,

    // === 规则 D: 市价单保护 ===
    /// 市价单最大名义金额（USDT）
    pub max_market_order_notional: Decimal,

    // === 规则 E: 强平前安全风控 (v1.1 安全修补) ===
    /// 临界保证金率阈值 (0.0 - 1.0)
    /// 当 margin_ratio < critical_margin_ratio 时，禁止新开仓，允许减仓/平仓
    /// 默认: 0.1 (10%)
    pub critical_margin_ratio: Decimal,

    // === 频率限制 ===
    /// 最小下单间隔（毫秒）
    pub min_order_interval_ms: u64,
}

impl Default for RiskLimits {
    fn default() -> Self {
        Self {
            // 规则 A
            min_order_qty: Decimal::new(1, 4),           // 0.0001
            max_order_qty: Decimal::new(10, 0),          // 10
            max_order_notional: Decimal::new(10000, 0),  // 10000 USDT
            max_balance_usage_ratio: Decimal::new(5, 1), // 0.5 (50%)
            // 规则 B
            max_position_per_symbol: Decimal::new(100, 0), // 100
            max_open_orders_per_symbol: 10,
            // 规则 C
            max_total_exposure: Decimal::new(100000, 0), // 100000 USDT
            max_total_open_orders: 50,
            // 规则 D
            max_market_order_notional: Decimal::new(5000, 0), // 5000 USDT
            // 规则 E (v1.1 安全修补)
            critical_margin_ratio: Decimal::new(1, 1), // 0.1 (10%)
            // 频率限制
            min_order_interval_ms: 100,
        }
    }
}

impl RiskLimits {
    /// 从环境变量加载风险限额
    pub fn from_env() -> Self {
        let mut limits = Self::default();

        if let Ok(v) = std::env::var("RISK_MIN_QTY") {
            if let Ok(d) = v.parse() { limits.min_order_qty = d; }
        }
        if let Ok(v) = std::env::var("RISK_MAX_QTY") {
            if let Ok(d) = v.parse() { limits.max_order_qty = d; }
        }
        if let Ok(v) = std::env::var("RISK_MAX_NOTIONAL") {
            if let Ok(d) = v.parse() { limits.max_order_notional = d; }
        }
        if let Ok(v) = std::env::var("RISK_MAX_BALANCE_RATIO") {
            if let Ok(d) = v.parse() { limits.max_balance_usage_ratio = d; }
        }
        if let Ok(v) = std::env::var("RISK_MAX_POSITION") {
            if let Ok(d) = v.parse() { limits.max_position_per_symbol = d; }
        }
        if let Ok(v) = std::env::var("RISK_MAX_OPEN_ORDERS") {
            if let Ok(d) = v.parse::<usize>() { limits.max_open_orders_per_symbol = d; }
        }
        if let Ok(v) = std::env::var("RISK_MAX_TOTAL_EXPOSURE") {
            if let Ok(d) = v.parse() { limits.max_total_exposure = d; }
        }
        if let Ok(v) = std::env::var("RISK_MAX_MARKET_NOTIONAL") {
            if let Ok(d) = v.parse() { limits.max_market_order_notional = d; }
        }
        if let Ok(v) = std::env::var("RISK_ORDER_INTERVAL_MS") {
            if let Ok(d) = v.parse() { limits.min_order_interval_ms = d; }
        }
        // v1.1 安全修补: 临界保证金率
        if let Ok(v) = std::env::var("RISK_CRITICAL_MARGIN_RATIO") {
            if let Ok(d) = v.parse() { limits.critical_margin_ratio = d; }
        }

        limits
    }
}

// ============================================================================
// OrderRiskConfig - 风控配置（基础配置 + RiskLimits）
// ============================================================================

/// 订单风控配置 v1
#[derive(Debug, Clone)]
pub struct OrderRiskConfig {
    // === 基础配置 ===
    /// 允许的交易对（空表示允许所有）
    pub allowed_symbols: HashSet<String>,
    /// 是否启用交易
    pub trading_enabled: bool,
    /// 计价资产（用于余额检查）
    pub quote_asset: String,

    // === 风险限额（收敛所有数值参数）===
    pub limits: RiskLimits,
}

impl Default for OrderRiskConfig {
    fn default() -> Self {
        Self {
            allowed_symbols: HashSet::new(),
            trading_enabled: true,
            quote_asset: "USDT".to_string(),
            limits: RiskLimits::default(),
        }
    }
}

impl OrderRiskConfig {
    /// 从环境变量创建配置
    pub fn from_env() -> Self {
        let mut config = Self::default();

        // 解析允许的交易对
        if let Ok(symbols) = std::env::var("RISK_ALLOW_SYMBOLS") {
            config.allowed_symbols = symbols
                .split(',')
                .map(|s| s.trim().to_uppercase())
                .filter(|s| !s.is_empty())
                .collect();
        }

        if let Ok(v) = std::env::var("TRADING_ENABLED") {
            config.trading_enabled = v.to_lowercase() == "true" || v == "1";
        }

        // 加载风险限额
        config.limits = RiskLimits::from_env();

        config
    }
}
