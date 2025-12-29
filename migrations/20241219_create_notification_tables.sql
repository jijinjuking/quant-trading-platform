-- 通知服务数据库表结构
-- 创建时间: 2024-12-19

-- 通知类型枚举
CREATE TYPE notification_type AS ENUM (
    'system',
    'trading',
    'account',
    'security',
    'marketing',
    'alert',
    'reminder',
    'welcome',
    'verification',
    'password_reset',
    'order_update',
    'trade_execution',
    'risk_alert',
    'maintenance_notice',
    'promotional_offer'
);

-- 通知优先级枚举
CREATE TYPE notification_priority AS ENUM (
    'low',
    'normal',
    'high',
    'critical',
    'emergency'
);

-- 通知状态枚举
CREATE TYPE notification_status AS ENUM (
    'pending',
    'scheduled',
    'processing',
    'sent',
    'failed',
    'cancelled',
    'expired'
);

-- 通知渠道枚举
CREATE TYPE notification_channel AS ENUM (
    'email',
    'sms',
    'push',
    'websocket',
    'in_app',
    'webhook'
);

-- 模板类型枚举
CREATE TYPE template_type AS ENUM (
    'email',
    'sms',
    'push',
    'in_app',
    'webhook'
);

-- 投递状态枚举
CREATE TYPE delivery_status AS ENUM (
    'pending',
    'in_progress',
    'delivered',
    'failed',
    'bounced',
    'rejected',
    'expired'
);

-- 通知表
CREATE TABLE notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,
    notification_type notification_type NOT NULL,
    priority notification_priority NOT NULL DEFAULT 'normal',
    status notification_status NOT NULL DEFAULT 'pending',
    channels JSONB NOT NULL DEFAULT '[]',
    metadata JSONB NOT NULL DEFAULT '{}',
    scheduled_at TIMESTAMPTZ,
    sent_at TIMESTAMPTZ,
    expires_at TIMESTAMPTZ,
    retry_count INTEGER NOT NULL DEFAULT 0,
    max_retries INTEGER NOT NULL DEFAULT 3,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 通知模板表
CREATE TABLE notification_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    template_type template_type NOT NULL,
    subject_template TEXT,
    body_template TEXT NOT NULL,
    variables JSONB NOT NULL DEFAULT '[]',
    default_values JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    version INTEGER NOT NULL DEFAULT 1,
    language VARCHAR(10) NOT NULL DEFAULT 'zh-CN',
    created_by UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 投递尝试表
CREATE TABLE delivery_attempts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notification_id UUID NOT NULL REFERENCES notifications(id) ON DELETE CASCADE,
    channel notification_channel NOT NULL,
    recipient VARCHAR(255) NOT NULL,
    status delivery_status NOT NULL DEFAULT 'pending',
    attempt_number INTEGER NOT NULL DEFAULT 1,
    external_id VARCHAR(255),
    error_message TEXT,
    metadata JSONB NOT NULL DEFAULT '{}',
    attempted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_at TIMESTAMPTZ
);

-- 订阅表
CREATE TABLE notification_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    notification_type notification_type NOT NULL,
    channel notification_channel NOT NULL,
    endpoint VARCHAR(500),
    auth_token VARCHAR(255),
    public_key TEXT,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, notification_type, channel)
);

-- 渠道配置表
CREATE TABLE channel_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel notification_channel NOT NULL UNIQUE,
    name VARCHAR(255) NOT NULL,
    is_enabled BOOLEAN NOT NULL DEFAULT true,
    priority INTEGER NOT NULL DEFAULT 0,
    rate_limit_per_minute INTEGER,
    rate_limit_per_hour INTEGER,
    rate_limit_per_day INTEGER,
    max_retries INTEGER NOT NULL DEFAULT 3,
    retry_delay_seconds INTEGER NOT NULL DEFAULT 60,
    timeout_seconds INTEGER NOT NULL DEFAULT 30,
    configuration JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 渠道健康检查表
CREATE TABLE channel_health_checks (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel notification_channel NOT NULL,
    is_healthy BOOLEAN NOT NULL,
    response_time_ms BIGINT,
    error_message TEXT,
    checked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 索引
CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_type ON notifications(notification_type);
CREATE INDEX idx_notifications_scheduled_at ON notifications(scheduled_at) WHERE scheduled_at IS NOT NULL;
CREATE INDEX idx_notifications_created_at ON notifications(created_at);

CREATE INDEX idx_templates_name ON notification_templates(name);
CREATE INDEX idx_templates_type ON notification_templates(template_type);
CREATE INDEX idx_templates_active ON notification_templates(is_active);

CREATE INDEX idx_delivery_attempts_notification_id ON delivery_attempts(notification_id);
CREATE INDEX idx_delivery_attempts_status ON delivery_attempts(status);
CREATE INDEX idx_delivery_attempts_channel ON delivery_attempts(channel);
CREATE INDEX idx_delivery_attempts_attempted_at ON delivery_attempts(attempted_at);

CREATE INDEX idx_subscriptions_user_id ON notification_subscriptions(user_id);
CREATE INDEX idx_subscriptions_type ON notification_subscriptions(notification_type);
CREATE INDEX idx_subscriptions_channel ON notification_subscriptions(channel);

CREATE INDEX idx_channel_configs_channel ON channel_configs(channel);
CREATE INDEX idx_channel_configs_enabled ON channel_configs(is_enabled);

CREATE INDEX idx_health_checks_channel ON channel_health_checks(channel);
CREATE INDEX idx_health_checks_checked_at ON channel_health_checks(checked_at);

-- 触发器：自动更新 updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_notifications_updated_at BEFORE UPDATE ON notifications
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_templates_updated_at BEFORE UPDATE ON notification_templates
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_subscriptions_updated_at BEFORE UPDATE ON notification_subscriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_channel_configs_updated_at BEFORE UPDATE ON channel_configs
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();