use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

use crate::common::Exchange;
use crate::trading::{OrderSide, OrderType};

/// 风险规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub rule_type: RiskRuleType,
    pub priority: u8,
    pub is_active: bool,
    pub conditions: Vec<RiskCondition>,
    pub actions: Vec<RiskAction>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 风险规则类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskRuleType {
    PreTrade,    // 交易前检�?
    PostTrade,   // 交易后检�?
    Portfolio,   // 组合风险
    Position,    // 仓位风险
    Market,      // 市场风险
    Liquidity,   // 流动性风�?
    Credit,      // 信用风险
    Operational, // 操作风险
}

/// 风险条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskCondition {
    pub field: String,
    pub operator: RiskOperator,
    pub value: serde_json::Value,
    pub logical_operator: Option<LogicalOperator>,
}

/// 风险操作�?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskOperator {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    NotEqual,
    In,
    NotIn,
    Contains,
    NotContains,
}

/// 逻辑操作�?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

/// 风险动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAction {
    pub action_type: RiskActionType,
    pub parameters: HashMap<String, serde_json::Value>,
}

/// 风险动作类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskActionType {
    Block,          // 阻止交易
    Warn,           // 发出警告
    Modify,         // 修改订单
    Cancel,         // 取消订单
    ClosePosition,  // 平仓
    ReducePosition, // 减仓
    Notify,         // 通知
    Log,            // 记录日志
}

/// 风险限额
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskLimit {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub exchange: Option<Exchange>,
    pub symbol: Option<String>,
    pub limit_type: RiskLimitType,
    pub value: Decimal,
    pub period: Option<RiskPeriod>,
    pub is_active: bool,
    pub current_usage: Decimal,
    pub last_reset: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 风险限额类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLimitType {
    MaxPositionSize,  // 最大仓位大�?
    MaxOrderValue,    // 最大订单价�?
    MaxDailyLoss,     // 最大日损失
    MaxDrawdown,      // 最大回�?
    MaxLeverage,      // 最大杠�?
    MaxConcentration, // 最大集中度
    MaxCorrelation,   // 最大相关�?
    MaxVaR,           // 最大风险价�?
    MaxTradingVolume, // 最大交易量
    MaxOrderCount,    // 最大订单数�?
}

/// 风险周期
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskPeriod {
    Intraday,  // 日内
    Daily,     // 每日
    Weekly,    // 每周
    Monthly,   // 每月
    Quarterly, // 每季�?
    Yearly,    // 每年
}

/// 风险检查请�?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskCheckRequest {
    pub user_id: Uuid,
    pub exchange: Exchange,
    pub symbol: String,
    pub side: OrderSide,
    pub order_type: OrderType,
    pub quantity: Decimal,
    pub price: Option<Decimal>,
    pub current_positions: HashMap<String, Decimal>,
    pub account_balance: Decimal,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 风险检查结�?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskCheckResult {
    pub decision: RiskDecision,
    pub violations: Vec<RiskViolation>,
    pub warnings: Vec<RiskWarning>,
    pub modifications: Option<OrderModification>,
    pub check_time: DateTime<Utc>,
}

/// 风险决策
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskDecision {
    Allow,
    Block,
    Modify,
    Warning,
}

/// 风险违规
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskViolation {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub severity: RiskSeverity,
    pub message: String,
    pub current_value: serde_json::Value,
    pub limit_value: serde_json::Value,
}

/// 风险严重程度
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// 风险警告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskWarning {
    pub rule_id: Uuid,
    pub rule_name: String,
    pub message: String,
    pub recommendation: Option<String>,
}

/// 订单修改建议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderModification {
    pub new_quantity: Option<Decimal>,
    pub new_price: Option<Decimal>,
    pub reason: String,
}

/// 风险指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMetric {
    pub id: Uuid,
    pub user_id: Uuid,
    pub metric_type: RiskMetricType,
    pub value: Decimal,
    pub timestamp: DateTime<Utc>,
    pub period: RiskPeriod,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 风险指标类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RiskMetricType {
    VaR,           // Value at Risk
    CVaR,          // Conditional Value at Risk
    Volatility,    // 波动�?
    Beta,          // 贝塔系数
    Correlation,   // 相关�?
    Concentration, // 集中�?
    Leverage,      // 杠杆�?
    Drawdown,      // 回撤
    SharpeRatio,   // 夏普比率
    SortinoRatio,  // 索提诺比�?
    MaxDrawdown,   // 最大回�?
    TrackingError, // 跟踪误差
}

/// 风险报告
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskReport {
    pub id: Uuid,
    pub user_id: Uuid,
    pub report_type: RiskReportType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub summary: RiskSummary,
    pub metrics: Vec<RiskMetric>,
    pub violations: Vec<RiskViolation>,
    pub recommendations: Vec<String>,
    pub generated_at: DateTime<Utc>,
}

/// 风险报告类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskReportType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    OnDemand,
}

/// 风险摘要
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskSummary {
    pub overall_risk_score: Decimal,
    pub risk_level: RiskLevel,
    pub total_exposure: Decimal,
    pub max_drawdown: Decimal,
    pub var_95: Decimal,
    pub var_99: Decimal,
    pub volatility: Decimal,
    pub sharpe_ratio: Decimal,
    pub violation_count: u32,
    pub warning_count: u32,
}

/// 风险等级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Extreme,
}

/// 风险事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: RiskEventType,
    pub severity: RiskSeverity,
    pub title: String,
    pub description: String,
    pub affected_positions: Vec<String>,
    pub impact_amount: Option<Decimal>,
    pub status: RiskEventStatus,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// 风险事件类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskEventType {
    LimitBreach,      // 限额突破
    MarketCrash,      // 市场崩盘
    LiquidityDry,     // 流动性枯�?
    SystemFailure,    // 系统故障
    FraudDetection,   // 欺诈检�?
    ComplianceIssue,  // 合规问题
    OperationalRisk,  // 操作风险
    CounterpartyRisk, // 对手方风�?
}

/// 风险事件状�?
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskEventStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
    Escalated,
}

/// 风险配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskConfig {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub rules: Vec<Uuid>,
    pub limits: Vec<Uuid>,
    pub is_active: bool,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 风险监控设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskMonitoringSettings {
    pub real_time_monitoring: bool,
    pub alert_thresholds: HashMap<RiskMetricType, Decimal>,
    pub notification_channels: Vec<NotificationChannel>,
    pub monitoring_frequency: u32, // �?
    pub auto_actions: HashMap<RiskSeverity, Vec<RiskActionType>>,
}

/// 通知渠道
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationChannel {
    Email,
    SMS,
    Push,
    Webhook,
    Slack,
    Discord,
}



