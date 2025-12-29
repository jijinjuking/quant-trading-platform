-- 修复通知服务数据库schema不匹配问题
-- 创建时间: 2024-12-19

-- 1. 为 delivery_attempts 表添加 response_code 字段
ALTER TABLE delivery_attempts ADD COLUMN response_code INTEGER;

-- 2. 为 notification_subscriptions 表添加 is_enabled 字段
ALTER TABLE notification_subscriptions ADD COLUMN is_enabled BOOLEAN NOT NULL DEFAULT true;

-- 3. 创建缺失的表：websocket_subscriptions
CREATE TABLE websocket_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    connection_id VARCHAR(255) NOT NULL UNIQUE,
    topics JSONB NOT NULL DEFAULT '[]',
    is_active BOOLEAN NOT NULL DEFAULT true,
    last_ping TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 4. 创建缺失的表：push_subscriptions
CREATE TABLE push_subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    endpoint VARCHAR(500) NOT NULL,
    p256dh_key TEXT NOT NULL,
    auth_key TEXT NOT NULL,
    user_agent TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, endpoint)
);

-- 5. 为新表添加索引
CREATE INDEX idx_websocket_subscriptions_user_id ON websocket_subscriptions(user_id);
CREATE INDEX idx_websocket_subscriptions_connection_id ON websocket_subscriptions(connection_id);
CREATE INDEX idx_websocket_subscriptions_active ON websocket_subscriptions(is_active);

CREATE INDEX idx_push_subscriptions_user_id ON push_subscriptions(user_id);
CREATE INDEX idx_push_subscriptions_endpoint ON push_subscriptions(endpoint);
CREATE INDEX idx_push_subscriptions_active ON push_subscriptions(is_active);

-- 6. 为 push_subscriptions 添加 updated_at 触发器
CREATE TRIGGER update_push_subscriptions_updated_at BEFORE UPDATE ON push_subscriptions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();