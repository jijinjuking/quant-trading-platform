-- 风险管理服务数据库表结构
-- 创建时间: 2024-12-19
-- 描述: 风险评估、限额管理、告警系统、规则引擎相关表

-- 风险评估表
CREATE TABLE IF NOT EXISTS risk_assessments (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    symbol VARCHAR(20),
    risk_type VARCHAR(50) NOT NULL,
    risk_level VARCHAR(20) NOT NULL,
    risk_score DECIMAL(10,4) NOT NULL,
    confidence DECIMAL(5,4) NOT NULL,
    factors JSONB DEFAULT '[]',
    recommendations JSONB DEFAULT '[]',
    time_frame VARCHAR(20) NOT NULL,
    calculated_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    metadata JSONB,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 实时风险数据表
CREATE TABLE IF NOT EXISTS realtime_risks (
    user_id UUID NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    current_risk_score DECIMAL(10,4) NOT NULL,
    risk_level VARCHAR(20) NOT NULL,
    var_1d DECIMAL(15,2) NOT NULL DEFAULT 0,
    var_1w DECIMAL(15,2) NOT NULL DEFAULT 0,
    expected_shortfall DECIMAL(15,2) NOT NULL DEFAULT 0,
    max_drawdown DECIMAL(8,4) NOT NULL DEFAULT 0,
    volatility DECIMAL(8,4) NOT NULL DEFAULT 0,
    beta DECIMAL(8,4) NOT NULL DEFAULT 0,
    correlation_risk DECIMAL(8,4) NOT NULL DEFAULT 0,
    concentration_risk DECIMAL(8,4) NOT NULL DEFAULT 0,
    liquidity_risk DECIMAL(8,4) NOT NULL DEFAULT 0,
    leverage_ratio DECIMAL(8,2) NOT NULL DEFAULT 0,
    margin_utilization DECIMAL(5,2) NOT NULL DEFAULT 0,
    position_size_risk DECIMAL(8,4) NOT NULL DEFAULT 0,
    timestamp TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, symbol)
);

-- 风险预警表
CREATE TABLE IF NOT EXISTS risk_warnings (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    symbol VARCHAR(20),
    warning_type VARCHAR(50) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    threshold_value DECIMAL(15,2) NOT NULL,
    current_value DECIMAL(15,2) NOT NULL,
    breach_percentage DECIMAL(8,4) NOT NULL DEFAULT 0,
    recommendations JSONB DEFAULT '[]',
    is_active BOOLEAN NOT NULL DEFAULT true,
    acknowledged BOOLEAN NOT NULL DEFAULT false,
    acknowledged_by UUID,
    acknowledged_at TIMESTAMPTZ,
    resolved_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 风险指标表
CREATE TABLE IF NOT EXISTS risk_metrics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    symbol VARCHAR(20),
    time_frame VARCHAR(20) NOT NULL,
    total_exposure DECIMAL(15,2) NOT NULL DEFAULT 0,
    net_exposure DECIMAL(15,2) NOT NULL DEFAULT 0,
    gross_exposure DECIMAL(15,2) NOT NULL DEFAULT 0,
    leverage DECIMAL(8,2) NOT NULL DEFAULT 0,
    margin_ratio DECIMAL(8,4) NOT NULL DEFAULT 0,
    var_95 DECIMAL(15,2) NOT NULL DEFAULT 0,
    var_99 DECIMAL(15,2) NOT NULL DEFAULT 0,
    expected_shortfall DECIMAL(15,2) NOT NULL DEFAULT 0,
    conditional_var DECIMAL(15,2) NOT NULL DEFAULT 0,
    max_drawdown DECIMAL(8,4) NOT NULL DEFAULT 0,
    current_drawdown DECIMAL(8,4) NOT NULL DEFAULT 0,
    drawdown_duration INTEGER NOT NULL DEFAULT 0,
    recovery_factor DECIMAL(8,4) NOT NULL DEFAULT 0,
    realized_volatility DECIMAL(8,4) NOT NULL DEFAULT 0,
    implied_volatility DECIMAL(8,4),
    volatility_ratio DECIMAL(8,4) NOT NULL DEFAULT 0,
    volatility_trend VARCHAR(20) NOT NULL DEFAULT 'stable',
    portfolio_correlation DECIMAL(8,4) NOT NULL DEFAULT 0,
    market_correlation DECIMAL(8,4) NOT NULL DEFAULT 0,
    sector_correlation DECIMAL(8,4) NOT NULL DEFAULT 0,
    liquidity_score DECIMAL(8,4) NOT NULL DEFAULT 0,
    bid_ask_spread DECIMAL(8,6) NOT NULL DEFAULT 0,
    market_impact DECIMAL(8,4) NOT NULL DEFAULT 0,
    time_to_liquidate INTEGER NOT NULL DEFAULT 0,
    concentration_ratio DECIMAL(8,4) NOT NULL DEFAULT 0,
    herfindahl_index DECIMAL(8,4) NOT NULL DEFAULT 0,
    largest_position_ratio DECIMAL(8,4) NOT NULL DEFAULT 0,
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 风险限额表
CREATE TABLE IF NOT EXISTS risk_limits (
    id UUID PRIMARY KEY,
    user_id UUID,
    symbol VARCHAR(20),
    strategy_id UUID,
    limit_type VARCHAR(50) NOT NULL,
    scope VARCHAR(20) NOT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    max_value DECIMAL(15,2) NOT NULL,
    warning_threshold DECIMAL(5,2) NOT NULL DEFAULT 80,
    current_value DECIMAL(15,2) NOT NULL DEFAULT 0,
    utilization_rate DECIMAL(5,2) NOT NULL DEFAULT 0,
    time_window VARCHAR(20),
    reset_frequency VARCHAR(20),
    last_reset_at TIMESTAMPTZ,
    status VARCHAR(20) NOT NULL DEFAULT 'Normal',
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_breached BOOLEAN NOT NULL DEFAULT false,
    breach_count INTEGER NOT NULL DEFAULT 0,
    last_breach_at TIMESTAMPTZ,
    auto_action VARCHAR(50),
    notification_enabled BOOLEAN NOT NULL DEFAULT true,
    escalation_enabled BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 动态限额调整表
CREATE TABLE IF NOT EXISTS dynamic_limit_adjustments (
    id UUID PRIMARY KEY,
    limit_id UUID NOT NULL REFERENCES risk_limits(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    adjustment_type VARCHAR(20) NOT NULL,
    original_value DECIMAL(15,2) NOT NULL,
    new_value DECIMAL(15,2) NOT NULL,
    adjustment_factor DECIMAL(8,4) NOT NULL,
    reason TEXT NOT NULL,
    trigger_condition VARCHAR(200) NOT NULL,
    duration INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT true,
    expires_at TIMESTAMPTZ,
    created_by UUID NOT NULL,
    approved_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 限额模板表
CREATE TABLE IF NOT EXISTS limit_templates (
    id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL,
    target_user_type VARCHAR(50) NOT NULL,
    limits_config JSONB NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 限额历史表（用于统计分析）
CREATE TABLE IF NOT EXISTS limit_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    limit_id UUID NOT NULL REFERENCES risk_limits(id) ON DELETE CASCADE,
    value DECIMAL(15,2) NOT NULL,
    status VARCHAR(20) NOT NULL,
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 风险告警表
CREATE TABLE IF NOT EXISTS risk_alerts (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    symbol VARCHAR(20),
    strategy_id UUID,
    alert_type VARCHAR(50) NOT NULL,
    priority VARCHAR(20) NOT NULL,
    severity VARCHAR(20) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'New',
    title VARCHAR(200) NOT NULL,
    message TEXT NOT NULL,
    description TEXT,
    trigger_condition VARCHAR(500) NOT NULL,
    threshold_value DECIMAL(15,2),
    current_value DECIMAL(15,2),
    breach_percentage DECIMAL(8,4),
    related_limit_id UUID,
    related_order_id UUID,
    related_position_id UUID,
    context_data JSONB,
    assigned_to UUID,
    acknowledged_by UUID,
    acknowledged_at TIMESTAMPTZ,
    resolved_by UUID,
    resolved_at TIMESTAMPTZ,
    resolution_notes TEXT,
    notification_sent BOOLEAN NOT NULL DEFAULT false,
    notification_channels JSONB,
    escalation_level INTEGER NOT NULL DEFAULT 0,
    escalated_at TIMESTAMPTZ,
    auto_action_taken VARCHAR(100),
    auto_action_result VARCHAR(500),
    expires_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 告警规则表
CREATE TABLE IF NOT EXISTS alert_rules (
    id UUID PRIMARY KEY,
    user_id UUID,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    alert_type VARCHAR(50) NOT NULL,
    priority VARCHAR(20) NOT NULL,
    condition_expression TEXT NOT NULL,
    threshold_value DECIMAL(15,2),
    comparison_operator VARCHAR(10) NOT NULL DEFAULT '>',
    time_window VARCHAR(20),
    min_occurrences INTEGER NOT NULL DEFAULT 1,
    symbols JSONB,
    strategies JSONB,
    user_groups JSONB,
    notification_enabled BOOLEAN NOT NULL DEFAULT true,
    notification_channels JSONB NOT NULL DEFAULT '["email"]',
    notification_template VARCHAR(200),
    cooldown_period INTEGER NOT NULL DEFAULT 300,
    auto_action VARCHAR(50),
    auto_action_params JSONB,
    escalation_enabled BOOLEAN NOT NULL DEFAULT false,
    escalation_delay INTEGER,
    escalation_targets JSONB,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 告警通知配置表
CREATE TABLE IF NOT EXISTS alert_notification_configs (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    alert_types JSONB NOT NULL DEFAULT '[]',
    min_priority VARCHAR(20) NOT NULL DEFAULT 'Medium',
    channels JSONB NOT NULL DEFAULT '{}',
    quiet_hours JSONB,
    frequency_limit INTEGER,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 风险规则表
CREATE TABLE IF NOT EXISTS risk_rules (
    id UUID PRIMARY KEY,
    user_id UUID,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    rule_type VARCHAR(50) NOT NULL,
    risk_type VARCHAR(50) NOT NULL,
    priority INTEGER NOT NULL DEFAULT 100,
    condition_expression TEXT NOT NULL,
    operator VARCHAR(20) NOT NULL,
    threshold_value DECIMAL(15,2),
    threshold_min DECIMAL(15,2),
    threshold_max DECIMAL(15,2),
    time_window VARCHAR(20),
    lookback_period VARCHAR(20),
    min_occurrences INTEGER NOT NULL DEFAULT 1,
    max_occurrences INTEGER,
    cooldown_period INTEGER NOT NULL DEFAULT 300,
    last_triggered_at TIMESTAMPTZ,
    trigger_count INTEGER NOT NULL DEFAULT 0,
    symbols JSONB,
    strategies JSONB,
    user_groups JSONB,
    market_conditions JSONB,
    actions JSONB NOT NULL DEFAULT '[]',
    action_params JSONB,
    auto_execute BOOLEAN NOT NULL DEFAULT false,
    require_approval BOOLEAN NOT NULL DEFAULT true,
    script_content TEXT,
    script_language VARCHAR(20),
    script_timeout INTEGER,
    ml_model_id UUID,
    ml_confidence_threshold DECIMAL(5,4),
    ml_features JSONB,
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_validated BOOLEAN NOT NULL DEFAULT false,
    validation_errors JSONB,
    last_validation_at TIMESTAMPTZ,
    execution_count INTEGER NOT NULL DEFAULT 0,
    success_count INTEGER NOT NULL DEFAULT 0,
    failure_count INTEGER NOT NULL DEFAULT 0,
    avg_execution_time DECIMAL(10,2),
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 规则执行结果表
CREATE TABLE IF NOT EXISTS rule_execution_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_id UUID NOT NULL REFERENCES risk_rules(id) ON DELETE CASCADE,
    rule_name VARCHAR(200) NOT NULL,
    triggered BOOLEAN NOT NULL,
    trigger_value DECIMAL(15,2),
    threshold_value DECIMAL(15,2),
    actions_executed JSONB DEFAULT '[]',
    execution_time_ms BIGINT NOT NULL,
    error_message TEXT,
    context_data JSONB,
    executed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 规则模板表
CREATE TABLE IF NOT EXISTS rule_templates (
    id UUID PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    category VARCHAR(50) NOT NULL,
    rule_type VARCHAR(50) NOT NULL,
    risk_type VARCHAR(50) NOT NULL,
    template_config JSONB NOT NULL,
    default_params JSONB NOT NULL DEFAULT '{}',
    is_builtin BOOLEAN NOT NULL DEFAULT false,
    is_public BOOLEAN NOT NULL DEFAULT false,
    usage_count INTEGER NOT NULL DEFAULT 0,
    rating DECIMAL(3,2),
    created_by UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_risk_assessments_user_id ON risk_assessments(user_id);
CREATE INDEX IF NOT EXISTS idx_risk_assessments_symbol ON risk_assessments(symbol);
CREATE INDEX IF NOT EXISTS idx_risk_assessments_expires_at ON risk_assessments(expires_at);
CREATE INDEX IF NOT EXISTS idx_risk_assessments_calculated_at ON risk_assessments(calculated_at);

CREATE INDEX IF NOT EXISTS idx_realtime_risks_user_symbol ON realtime_risks(user_id, symbol);
CREATE INDEX IF NOT EXISTS idx_realtime_risks_timestamp ON realtime_risks(timestamp);

CREATE INDEX IF NOT EXISTS idx_risk_warnings_user_id ON risk_warnings(user_id);
CREATE INDEX IF NOT EXISTS idx_risk_warnings_is_active ON risk_warnings(is_active);
CREATE INDEX IF NOT EXISTS idx_risk_warnings_created_at ON risk_warnings(created_at);

CREATE INDEX IF NOT EXISTS idx_risk_metrics_user_id ON risk_metrics(user_id);
CREATE INDEX IF NOT EXISTS idx_risk_metrics_symbol ON risk_metrics(symbol);
CREATE INDEX IF NOT EXISTS idx_risk_metrics_calculated_at ON risk_metrics(calculated_at);

CREATE INDEX IF NOT EXISTS idx_risk_limits_user_id ON risk_limits(user_id);
CREATE INDEX IF NOT EXISTS idx_risk_limits_symbol ON risk_limits(symbol);
CREATE INDEX IF NOT EXISTS idx_risk_limits_is_active ON risk_limits(is_active);
CREATE INDEX IF NOT EXISTS idx_risk_limits_limit_type ON risk_limits(limit_type);

CREATE INDEX IF NOT EXISTS idx_dynamic_adjustments_limit_id ON dynamic_limit_adjustments(limit_id);
CREATE INDEX IF NOT EXISTS idx_dynamic_adjustments_user_id ON dynamic_limit_adjustments(user_id);
CREATE INDEX IF NOT EXISTS idx_dynamic_adjustments_is_active ON dynamic_limit_adjustments(is_active);

CREATE INDEX IF NOT EXISTS idx_limit_templates_category ON limit_templates(category);
CREATE INDEX IF NOT EXISTS idx_limit_templates_is_active ON limit_templates(is_active);

CREATE INDEX IF NOT EXISTS idx_limit_history_limit_id ON limit_history(limit_id);
CREATE INDEX IF NOT EXISTS idx_limit_history_recorded_at ON limit_history(recorded_at);

CREATE INDEX IF NOT EXISTS idx_risk_alerts_user_id ON risk_alerts(user_id);
CREATE INDEX IF NOT EXISTS idx_risk_alerts_status ON risk_alerts(status);
CREATE INDEX IF NOT EXISTS idx_risk_alerts_priority ON risk_alerts(priority);
CREATE INDEX IF NOT EXISTS idx_risk_alerts_created_at ON risk_alerts(created_at);
CREATE INDEX IF NOT EXISTS idx_risk_alerts_expires_at ON risk_alerts(expires_at);

CREATE INDEX IF NOT EXISTS idx_alert_rules_user_id ON alert_rules(user_id);
CREATE INDEX IF NOT EXISTS idx_alert_rules_is_active ON alert_rules(is_active);
CREATE INDEX IF NOT EXISTS idx_alert_rules_alert_type ON alert_rules(alert_type);

CREATE INDEX IF NOT EXISTS idx_alert_notification_configs_user_id ON alert_notification_configs(user_id);

CREATE INDEX IF NOT EXISTS idx_risk_rules_user_id ON risk_rules(user_id);
CREATE INDEX IF NOT EXISTS idx_risk_rules_is_active ON risk_rules(is_active);
CREATE INDEX IF NOT EXISTS idx_risk_rules_priority ON risk_rules(priority);
CREATE INDEX IF NOT EXISTS idx_risk_rules_rule_type ON risk_rules(rule_type);

CREATE INDEX IF NOT EXISTS idx_rule_execution_results_rule_id ON rule_execution_results(rule_id);
CREATE INDEX IF NOT EXISTS idx_rule_execution_results_executed_at ON rule_execution_results(executed_at);

CREATE INDEX IF NOT EXISTS idx_rule_templates_category ON rule_templates(category);
CREATE INDEX IF NOT EXISTS idx_rule_templates_is_public ON rule_templates(is_public);
CREATE INDEX IF NOT EXISTS idx_rule_templates_usage_count ON rule_templates(usage_count);

-- 创建触发器函数用于自动更新 updated_at 字段
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 为需要的表创建触发器
CREATE TRIGGER update_risk_assessments_updated_at BEFORE UPDATE ON risk_assessments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_risk_warnings_updated_at BEFORE UPDATE ON risk_warnings FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_risk_limits_updated_at BEFORE UPDATE ON risk_limits FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_dynamic_limit_adjustments_updated_at BEFORE UPDATE ON dynamic_limit_adjustments FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_limit_templates_updated_at BEFORE UPDATE ON limit_templates FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_risk_alerts_updated_at BEFORE UPDATE ON risk_alerts FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_alert_rules_updated_at BEFORE UPDATE ON alert_rules FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_alert_notification_configs_updated_at BEFORE UPDATE ON alert_notification_configs FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_risk_rules_updated_at BEFORE UPDATE ON risk_rules FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_rule_templates_updated_at BEFORE UPDATE ON rule_templates FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- 插入一些默认的限额模板
INSERT INTO limit_templates (id, name, description, category, target_user_type, limits_config, is_default, is_active, created_by) VALUES
(
    gen_random_uuid(),
    '保守型限额模板',
    '适用于风险偏好较低的用户',
    'conservative',
    'retail',
    '[
        {"limit_type": "Position", "name": "最大仓位限额", "max_value": "50000", "warning_threshold": "80", "time_window": "1d", "reset_frequency": "daily"},
        {"limit_type": "DailyLoss", "name": "日内最大损失", "max_value": "5000", "warning_threshold": "75", "time_window": "1d", "reset_frequency": "daily"},
        {"limit_type": "Leverage", "name": "最大杠杆", "max_value": "3", "warning_threshold": "80"},
        {"limit_type": "MaxDrawdown", "name": "最大回撤", "max_value": "0.1", "warning_threshold": "80"}
    ]'::jsonb,
    true,
    true,
    gen_random_uuid()
),
(
    gen_random_uuid(),
    '平衡型限额模板',
    '适用于风险偏好中等的用户',
    'moderate',
    'retail',
    '[
        {"limit_type": "Position", "name": "最大仓位限额", "max_value": "100000", "warning_threshold": "80", "time_window": "1d", "reset_frequency": "daily"},
        {"limit_type": "DailyLoss", "name": "日内最大损失", "max_value": "10000", "warning_threshold": "75", "time_window": "1d", "reset_frequency": "daily"},
        {"limit_type": "Leverage", "name": "最大杠杆", "max_value": "5", "warning_threshold": "80"},
        {"limit_type": "MaxDrawdown", "name": "最大回撤", "max_value": "0.15", "warning_threshold": "80"}
    ]'::jsonb,
    true,
    true,
    gen_random_uuid()
),
(
    gen_random_uuid(),
    '激进型限额模板',
    '适用于风险偏好较高的专业用户',
    'aggressive',
    'professional',
    '[
        {"limit_type": "Position", "name": "最大仓位限额", "max_value": "500000", "warning_threshold": "85", "time_window": "1d", "reset_frequency": "daily"},
        {"limit_type": "DailyLoss", "name": "日内最大损失", "max_value": "50000", "warning_threshold": "80", "time_window": "1d", "reset_frequency": "daily"},
        {"limit_type": "Leverage", "name": "最大杠杆", "max_value": "10", "warning_threshold": "85"},
        {"limit_type": "MaxDrawdown", "name": "最大回撤", "max_value": "0.25", "warning_threshold": "85"}
    ]'::jsonb,
    true,
    true,
    gen_random_uuid()
);

-- 插入一些默认的规则模板
INSERT INTO rule_templates (id, name, description, category, rule_type, risk_type, template_config, default_params, is_builtin, is_public, created_by) VALUES
(
    gen_random_uuid(),
    'VaR超限告警规则',
    '当VaR超过设定阈值时触发告警',
    'risk_control',
    'Threshold',
    'Market',
    '{"condition_expression": "var_1d > threshold_value", "actions": ["Alert", "Notify"]}'::jsonb,
    '{"threshold_value": "10000", "time_window": "1h", "cooldown_period": "300"}'::jsonb,
    true,
    true,
    gen_random_uuid()
),
(
    gen_random_uuid(),
    '杠杆过高预警规则',
    '当杠杆比率超过安全水平时发出预警',
    'position_management',
    'Threshold',
    'Leverage',
    '{"condition_expression": "leverage_ratio > threshold_value", "actions": ["Alert", "Log"]}'::jsonb,
    '{"threshold_value": "8", "time_window": "5m", "cooldown_period": "600"}'::jsonb,
    true,
    true,
    gen_random_uuid()
),
(
    gen_random_uuid(),
    '保证金不足告警规则',
    '当保证金使用率过高时触发紧急告警',
    'risk_control',
    'Threshold',
    'Operational',
    '{"condition_expression": "margin_utilization > threshold_value", "actions": ["Alert", "Block", "Notify"]}'::jsonb,
    '{"threshold_value": "85", "time_window": "1m", "cooldown_period": "60"}'::jsonb,
    true,
    true,
    gen_random_uuid()
);

COMMENT ON TABLE risk_assessments IS '风险评估结果表';
COMMENT ON TABLE realtime_risks IS '实时风险数据表';
COMMENT ON TABLE risk_warnings IS '风险预警表';
COMMENT ON TABLE risk_metrics IS '风险指标表';
COMMENT ON TABLE risk_limits IS '风险限额表';
COMMENT ON TABLE dynamic_limit_adjustments IS '动态限额调整表';
COMMENT ON TABLE limit_templates IS '限额模板表';
COMMENT ON TABLE limit_history IS '限额历史记录表';
COMMENT ON TABLE risk_alerts IS '风险告警表';
COMMENT ON TABLE alert_rules IS '告警规则表';
COMMENT ON TABLE alert_notification_configs IS '告警通知配置表';
COMMENT ON TABLE risk_rules IS '风险规则表';
COMMENT ON TABLE rule_execution_results IS '规则执行结果表';
COMMENT ON TABLE rule_templates IS '规则模板表';