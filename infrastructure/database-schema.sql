-- ============================================================================
-- 量化交易平台数据库 Schema
-- ============================================================================
-- 数据库: trading_platform
-- 创建时间: 2026-01-23
-- 说明: 包含所有微服务所需的数据表
-- ============================================================================

-- 创建数据库（如果不存在）
CREATE DATABASE IF NOT EXISTS trading_platform;

\c trading_platform;

-- ============================================================================
-- 1. 用户管理 (User Management)
-- ============================================================================

-- 用户表
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    phone VARCHAR(20),
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, suspended, deleted
    kyc_status VARCHAR(20) NOT NULL DEFAULT 'unverified', -- unverified, pending, verified, rejected
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_login_at TIMESTAMP
);

CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_status ON users(status);

-- API Key 表
CREATE TABLE IF NOT EXISTS api_keys (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    key_name VARCHAR(100) NOT NULL,
    api_key VARCHAR(64) UNIQUE NOT NULL,
    api_secret VARCHAR(128) NOT NULL,
    permissions JSONB NOT NULL DEFAULT '[]', -- ["read", "trade", "withdraw"]
    ip_whitelist TEXT[], -- 允许的 IP 地址列表
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, disabled
    expires_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMP
);

CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);
CREATE INDEX idx_api_keys_api_key ON api_keys(api_key);

-- ============================================================================
-- 2. 策略管理 (Strategy Management)
-- ============================================================================

-- 策略配置表
CREATE TABLE IF NOT EXISTS strategy_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    strategy_type VARCHAR(50) NOT NULL, -- spot_grid, spot_mean_reversion, futures_grid, etc.
    strategy_name VARCHAR(100) NOT NULL,
    symbol VARCHAR(20) NOT NULL, -- BTCUSDT, ETHUSDT, etc.
    market_type VARCHAR(20) NOT NULL, -- spot, futures
    params JSONB NOT NULL, -- 策略参数（JSON格式）
    status VARCHAR(20) NOT NULL DEFAULT 'stopped', -- running, stopped, paused, error
    auto_start BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    started_at TIMESTAMP,
    stopped_at TIMESTAMP
);

CREATE INDEX idx_strategy_configs_user_id ON strategy_configs(user_id);
CREATE INDEX idx_strategy_configs_status ON strategy_configs(status);
CREATE INDEX idx_strategy_configs_symbol ON strategy_configs(symbol);

-- 策略执行记录表
CREATE TABLE IF NOT EXISTS strategy_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES strategy_configs(id) ON DELETE CASCADE,
    request_id UUID NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    price DECIMAL(20, 8) NOT NULL,
    quantity DECIMAL(20, 8) NOT NULL,
    has_intent BOOLEAN NOT NULL DEFAULT false,
    signal_side VARCHAR(10), -- buy, sell
    signal_price DECIMAL(20, 8),
    signal_quantity DECIMAL(20, 8),
    executed_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_strategy_executions_strategy_id ON strategy_executions(strategy_id);
CREATE INDEX idx_strategy_executions_executed_at ON strategy_executions(executed_at);

-- ============================================================================
-- 3. 订单管理 (Order Management)
-- ============================================================================

-- 订单表
CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    strategy_id UUID REFERENCES strategy_configs(id) ON DELETE SET NULL,
    exchange VARCHAR(20) NOT NULL, -- binance, okx, etc.
    symbol VARCHAR(20) NOT NULL,
    order_type VARCHAR(20) NOT NULL, -- market, limit, stop_loss, etc.
    side VARCHAR(10) NOT NULL, -- buy, sell
    price DECIMAL(20, 8),
    quantity DECIMAL(20, 8) NOT NULL,
    filled_quantity DECIMAL(20, 8) NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL, -- pending, filled, partially_filled, cancelled, rejected
    exchange_order_id VARCHAR(100),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    filled_at TIMESTAMP
);

CREATE INDEX idx_orders_user_id ON orders(user_id);
CREATE INDEX idx_orders_strategy_id ON orders(strategy_id);
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_symbol ON orders(symbol);
CREATE INDEX idx_orders_created_at ON orders(created_at);

-- 订单成交记录表
CREATE TABLE IF NOT EXISTS order_fills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    exchange_trade_id VARCHAR(100) NOT NULL,
    price DECIMAL(20, 8) NOT NULL,
    quantity DECIMAL(20, 8) NOT NULL,
    commission DECIMAL(20, 8) NOT NULL DEFAULT 0,
    commission_asset VARCHAR(10),
    filled_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_order_fills_order_id ON order_fills(order_id);
CREATE INDEX idx_order_fills_filled_at ON order_fills(filled_at);

-- ============================================================================
-- 4. 持仓管理 (Position Management)
-- ============================================================================

-- 持仓表
CREATE TABLE IF NOT EXISTS positions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    strategy_id UUID REFERENCES strategy_configs(id) ON DELETE SET NULL,
    exchange VARCHAR(20) NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    market_type VARCHAR(20) NOT NULL, -- spot, futures
    side VARCHAR(10), -- long, short (futures only)
    quantity DECIMAL(20, 8) NOT NULL,
    entry_price DECIMAL(20, 8) NOT NULL,
    current_price DECIMAL(20, 8),
    unrealized_pnl DECIMAL(20, 8),
    realized_pnl DECIMAL(20, 8) NOT NULL DEFAULT 0,
    leverage INTEGER DEFAULT 1,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, exchange, symbol, market_type, side)
);

CREATE INDEX idx_positions_user_id ON positions(user_id);
CREATE INDEX idx_positions_strategy_id ON positions(strategy_id);
CREATE INDEX idx_positions_symbol ON positions(symbol);

-- ============================================================================
-- 5. 跟单系统 (CopyTrading)
-- ============================================================================

-- 跟单关系表
CREATE TABLE IF NOT EXISTS copytrading_relations (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    leader_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    strategy_id UUID REFERENCES strategy_configs(id) ON DELETE CASCADE,
    copy_ratio DECIMAL(5, 2) NOT NULL DEFAULT 1.0, -- 跟单比例 (0.01 - 100.00)
    max_amount_per_trade DECIMAL(20, 8), -- 单笔最大金额
    max_trades_per_day INTEGER, -- 每日最大交易次数
    status VARCHAR(20) NOT NULL DEFAULT 'active', -- active, paused, stopped
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(follower_id, leader_id, strategy_id)
);

CREATE INDEX idx_copytrading_relations_follower_id ON copytrading_relations(follower_id);
CREATE INDEX idx_copytrading_relations_leader_id ON copytrading_relations(leader_id);
CREATE INDEX idx_copytrading_relations_status ON copytrading_relations(status);

-- 跟单记录表
CREATE TABLE IF NOT EXISTS copytrading_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    relation_id UUID NOT NULL REFERENCES copytrading_relations(id) ON DELETE CASCADE,
    leader_order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    follower_order_id UUID REFERENCES orders(id) ON DELETE SET NULL,
    leader_signal JSONB NOT NULL, -- 原始信号
    follower_signal JSONB, -- 跟单信号
    status VARCHAR(20) NOT NULL, -- pending, executed, failed, skipped
    failure_reason TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_copytrading_records_relation_id ON copytrading_records(relation_id);
CREATE INDEX idx_copytrading_records_leader_order_id ON copytrading_records(leader_order_id);
CREATE INDEX idx_copytrading_records_created_at ON copytrading_records(created_at);

-- ============================================================================
-- 6. 分佣系统 (Commission)
-- ============================================================================

-- 分佣配置表
CREATE TABLE IF NOT EXISTS commission_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES users(id) ON DELETE SET NULL, -- 上级用户
    level INTEGER NOT NULL DEFAULT 1, -- 层级 (1, 2, 3, ...)
    commission_rate DECIMAL(5, 4) NOT NULL, -- 分佣比例 (0.0001 - 1.0000)
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, parent_id)
);

CREATE INDEX idx_commission_configs_user_id ON commission_configs(user_id);
CREATE INDEX idx_commission_configs_parent_id ON commission_configs(parent_id);

-- 分佣记录表
CREATE TABLE IF NOT EXISTS commission_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    from_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- 产生交易的用户
    to_user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE, -- 获得分佣的用户
    level INTEGER NOT NULL, -- 层级
    commission_rate DECIMAL(5, 4) NOT NULL,
    trade_amount DECIMAL(20, 8) NOT NULL, -- 交易金额
    commission_amount DECIMAL(20, 8) NOT NULL, -- 分佣金额
    asset VARCHAR(10) NOT NULL, -- 分佣币种
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending, settled, failed
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    settled_at TIMESTAMP
);

CREATE INDEX idx_commission_records_order_id ON commission_records(order_id);
CREATE INDEX idx_commission_records_from_user_id ON commission_records(from_user_id);
CREATE INDEX idx_commission_records_to_user_id ON commission_records(to_user_id);
CREATE INDEX idx_commission_records_created_at ON commission_records(created_at);

-- ============================================================================
-- 7. 账户管理 (Account Management)
-- ============================================================================

-- 账户表
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    asset VARCHAR(10) NOT NULL, -- USDT, BTC, ETH, etc.
    balance DECIMAL(20, 8) NOT NULL DEFAULT 0,
    frozen DECIMAL(20, 8) NOT NULL DEFAULT 0, -- 冻结金额
    available DECIMAL(20, 8) GENERATED ALWAYS AS (balance - frozen) STORED,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, asset)
);

CREATE INDEX idx_accounts_user_id ON accounts(user_id);
CREATE INDEX idx_accounts_asset ON accounts(asset);

-- 账单表
CREATE TABLE IF NOT EXISTS bills (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    asset VARCHAR(10) NOT NULL,
    amount DECIMAL(20, 8) NOT NULL, -- 正数为入账，负数为出账
    balance_before DECIMAL(20, 8) NOT NULL,
    balance_after DECIMAL(20, 8) NOT NULL,
    bill_type VARCHAR(50) NOT NULL, -- deposit, withdraw, trade, commission, etc.
    related_id UUID, -- 关联的订单ID、分佣记录ID等
    description TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_bills_user_id ON bills(user_id);
CREATE INDEX idx_bills_asset ON bills(asset);
CREATE INDEX idx_bills_bill_type ON bills(bill_type);
CREATE INDEX idx_bills_created_at ON bills(created_at);

-- ============================================================================
-- 8. 风控管理 (Risk Management)
-- ============================================================================

-- 风控规则表
CREATE TABLE IF NOT EXISTS risk_rules (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    rule_name VARCHAR(100) NOT NULL,
    rule_type VARCHAR(50) NOT NULL, -- max_position, max_loss, max_leverage, etc.
    scope VARCHAR(20) NOT NULL, -- global, user
    target_id UUID, -- user_id (if scope = user)
    params JSONB NOT NULL, -- 规则参数
    enabled BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_risk_rules_scope ON risk_rules(scope);
CREATE INDEX idx_risk_rules_target_id ON risk_rules(target_id);
CREATE INDEX idx_risk_rules_enabled ON risk_rules(enabled);

-- 风控记录表
CREATE TABLE IF NOT EXISTS risk_records (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    rule_id UUID NOT NULL REFERENCES risk_rules(id) ON DELETE CASCADE,
    order_id UUID REFERENCES orders(id) ON DELETE SET NULL,
    risk_level VARCHAR(20) NOT NULL, -- low, medium, high, critical
    action VARCHAR(20) NOT NULL, -- alert, block, force_close
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_risk_records_user_id ON risk_records(user_id);
CREATE INDEX idx_risk_records_rule_id ON risk_records(rule_id);
CREATE INDEX idx_risk_records_risk_level ON risk_records(risk_level);
CREATE INDEX idx_risk_records_created_at ON risk_records(created_at);

-- ============================================================================
-- 9. 通知管理 (Notification)
-- ============================================================================

-- 通知表
CREATE TABLE IF NOT EXISTS notifications (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    notification_type VARCHAR(50) NOT NULL, -- order_filled, risk_alert, system, etc.
    title VARCHAR(200) NOT NULL,
    content TEXT NOT NULL,
    priority VARCHAR(20) NOT NULL DEFAULT 'normal', -- low, normal, high, urgent
    channels TEXT[] NOT NULL, -- ["email", "sms", "websocket", "in_app"]
    status VARCHAR(20) NOT NULL DEFAULT 'pending', -- pending, sent, failed
    read_at TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    sent_at TIMESTAMP
);

CREATE INDEX idx_notifications_user_id ON notifications(user_id);
CREATE INDEX idx_notifications_status ON notifications(status);
CREATE INDEX idx_notifications_created_at ON notifications(created_at);

-- ============================================================================
-- 10. 系统配置 (System Configuration)
-- ============================================================================

-- 系统配置表
CREATE TABLE IF NOT EXISTS system_configs (
    key VARCHAR(100) PRIMARY KEY,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);

-- 插入默认配置
INSERT INTO system_configs (key, value, description) VALUES
    ('maintenance_mode', 'false', '维护模式开关'),
    ('trading_enabled', 'true', '交易功能开关'),
    ('max_leverage', '10', '最大杠杆倍数'),
    ('min_order_amount', '10', '最小订单金额（USDT）')
ON CONFLICT (key) DO NOTHING;

-- ============================================================================
-- 完成
-- ============================================================================

COMMENT ON DATABASE trading_platform IS '量化交易平台数据库';

-- 创建更新时间触发器函数
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- 为所有有 updated_at 字段的表添加触发器
DO $$
DECLARE
    t text;
BEGIN
    FOR t IN
        SELECT table_name
        FROM information_schema.columns
        WHERE table_schema = 'public'
        AND column_name = 'updated_at'
    LOOP
        EXECUTE format('
            DROP TRIGGER IF EXISTS update_%I_updated_at ON %I;
            CREATE TRIGGER update_%I_updated_at
                BEFORE UPDATE ON %I
                FOR EACH ROW
                EXECUTE FUNCTION update_updated_at_column();
        ', t, t, t, t);
    END LOOP;
END;
$$;

-- 显示所有表
\dt

SELECT 'Database schema initialized successfully!' AS status;
