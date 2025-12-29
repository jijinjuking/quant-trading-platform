-- 交易引擎服务数据库表结构
-- 创建时间: 2024-12-18

-- 订单类型枚举
DO $$ BEGIN
    CREATE TYPE order_type AS ENUM ('MARKET', 'LIMIT', 'STOP_LOSS', 'TAKE_PROFIT', 'STOP_LOSS_LIMIT', 'TAKE_PROFIT_LIMIT');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 订单方向枚举
DO $$ BEGIN
    CREATE TYPE order_side AS ENUM ('BUY', 'SELL');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 订单状态枚举
DO $$ BEGIN
    CREATE TYPE order_status AS ENUM ('PENDING', 'PARTIALLY_FILLED', 'FILLED', 'CANCELLED', 'REJECTED', 'EXPIRED');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 时效类型枚举
DO $$ BEGIN
    CREATE TYPE time_in_force AS ENUM ('GTC', 'IOC', 'FOK', 'GTD');
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- 订单表
DROP TABLE IF EXISTS orders CASCADE;
CREATE TABLE orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    order_type order_type NOT NULL,
    side order_side NOT NULL,
    quantity DECIMAL(20,8) NOT NULL,
    price DECIMAL(20,8),
    stop_price DECIMAL(20,8),
    status order_status NOT NULL DEFAULT 'PENDING',
    time_in_force time_in_force NOT NULL DEFAULT 'GTC',
    filled_quantity DECIMAL(20,8) NOT NULL DEFAULT 0,
    remaining_quantity DECIMAL(20,8) NOT NULL,
    average_price DECIMAL(20,8),
    fee DECIMAL(20,8) NOT NULL DEFAULT 0,
    fee_currency VARCHAR(10) NOT NULL DEFAULT 'USDT',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    client_order_id VARCHAR(100),
    metadata JSONB NOT NULL DEFAULT '{}',
    CONSTRAINT orders_quantity_positive CHECK (quantity > 0),
    CONSTRAINT orders_filled_quantity_valid CHECK (filled_quantity >= 0 AND filled_quantity <= quantity),
    CONSTRAINT orders_remaining_quantity_valid CHECK (remaining_quantity >= 0 AND remaining_quantity <= quantity)
);

-- 仓位表
CREATE TABLE IF NOT EXISTS positions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    side order_side NOT NULL,
    quantity DECIMAL(20,8) NOT NULL,
    entry_price DECIMAL(20,8) NOT NULL,
    current_price DECIMAL(20,8),
    unrealized_pnl DECIMAL(20,8) NOT NULL DEFAULT 0,
    realized_pnl DECIMAL(20,8) NOT NULL DEFAULT 0,
    margin DECIMAL(20,8) NOT NULL DEFAULT 0,
    leverage DECIMAL(10,2) NOT NULL DEFAULT 1,
    liquidation_price DECIMAL(20,8),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    closed_at TIMESTAMP WITH TIME ZONE,
    metadata JSONB NOT NULL DEFAULT '{}',
    UNIQUE(user_id, symbol, side),
    CONSTRAINT positions_quantity_positive CHECK (quantity > 0),
    CONSTRAINT positions_entry_price_positive CHECK (entry_price > 0),
    CONSTRAINT positions_leverage_valid CHECK (leverage >= 1 AND leverage <= 125)
);

-- 成交记录表
CREATE TABLE IF NOT EXISTS trades (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id),
    user_id UUID NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    side order_side NOT NULL,
    quantity DECIMAL(20,8) NOT NULL,
    price DECIMAL(20,8) NOT NULL,
    fee DECIMAL(20,8) NOT NULL DEFAULT 0,
    fee_currency VARCHAR(10) NOT NULL DEFAULT 'USDT',
    trade_time TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    exchange_trade_id VARCHAR(100),
    is_maker BOOLEAN NOT NULL DEFAULT false,
    metadata JSONB NOT NULL DEFAULT '{}',
    CONSTRAINT trades_quantity_positive CHECK (quantity > 0),
    CONSTRAINT trades_price_positive CHECK (price > 0)
);

-- 账户表
CREATE TABLE IF NOT EXISTS accounts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL UNIQUE,
    balance DECIMAL(20,8) NOT NULL DEFAULT 0,
    available_balance DECIMAL(20,8) NOT NULL DEFAULT 0,
    frozen_balance DECIMAL(20,8) NOT NULL DEFAULT 0,
    margin_balance DECIMAL(20,8) NOT NULL DEFAULT 0,
    unrealized_pnl DECIMAL(20,8) NOT NULL DEFAULT 0,
    realized_pnl DECIMAL(20,8) NOT NULL DEFAULT 0,
    total_equity DECIMAL(20,8) NOT NULL DEFAULT 0,
    margin_ratio DECIMAL(10,4) NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    CONSTRAINT accounts_balance_non_negative CHECK (balance >= 0),
    CONSTRAINT accounts_available_balance_valid CHECK (available_balance >= 0 AND available_balance <= balance),
    CONSTRAINT accounts_frozen_balance_valid CHECK (frozen_balance >= 0 AND frozen_balance <= balance)
);

-- 账户历史表
CREATE TABLE IF NOT EXISTS account_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    change_type VARCHAR(50) NOT NULL,
    amount DECIMAL(20,8) NOT NULL,
    balance_before DECIMAL(20,8) NOT NULL,
    balance_after DECIMAL(20,8) NOT NULL,
    reference_id UUID,
    reference_type VARCHAR(50),
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    metadata JSONB NOT NULL DEFAULT '{}'
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_orders_user_id ON orders(user_id);
CREATE INDEX IF NOT EXISTS idx_orders_symbol ON orders(symbol);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_created_at ON orders(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_orders_user_symbol_status ON orders(user_id, symbol, status);
CREATE INDEX IF NOT EXISTS idx_orders_client_order_id ON orders(client_order_id) WHERE client_order_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_positions_user_id ON positions(user_id);
CREATE INDEX IF NOT EXISTS idx_positions_symbol ON positions(symbol);
CREATE INDEX IF NOT EXISTS idx_positions_user_symbol ON positions(user_id, symbol);

CREATE INDEX IF NOT EXISTS idx_trades_order_id ON trades(order_id);
CREATE INDEX IF NOT EXISTS idx_trades_user_id ON trades(user_id);
CREATE INDEX IF NOT EXISTS idx_trades_symbol ON trades(symbol);
CREATE INDEX IF NOT EXISTS idx_trades_trade_time ON trades(trade_time DESC);
CREATE INDEX IF NOT EXISTS idx_trades_user_symbol ON trades(user_id, symbol);

CREATE INDEX IF NOT EXISTS idx_accounts_user_id ON accounts(user_id);

CREATE INDEX IF NOT EXISTS idx_account_history_user_id ON account_history(user_id);
CREATE INDEX IF NOT EXISTS idx_account_history_created_at ON account_history(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_account_history_reference ON account_history(reference_id, reference_type);

-- 创建更新时间触发器
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_orders_updated_at BEFORE UPDATE ON orders
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_positions_updated_at BEFORE UPDATE ON positions
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_accounts_updated_at BEFORE UPDATE ON accounts
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();