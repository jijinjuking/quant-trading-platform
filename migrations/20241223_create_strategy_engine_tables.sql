-- 策略引擎服务数据库表结构
-- 创建时间: 2024-12-23
-- 服务: strategy-engine (Port 8083)

-- 策略定义表
CREATE TABLE IF NOT EXISTS strategies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    name VARCHAR(200) NOT NULL,
    description TEXT,
    strategy_type VARCHAR(50) NOT NULL, -- 'TREND_FOLLOWING', 'MEAN_REVERSION', 'ARBITRAGE', 'GRID', 'ML_PREDICTION', 'CUSTOM'
    status VARCHAR(20) NOT NULL DEFAULT 'INACTIVE', -- 'ACTIVE', 'INACTIVE', 'PAUSED', 'ERROR'
    symbols TEXT[] NOT NULL, -- 交易对列表
    timeframes TEXT[] NOT NULL, -- 时间周期 ['1m', '5m', '15m', '1h', '4h', '1d']
    parameters JSONB NOT NULL DEFAULT '{}', -- 策略参数
    risk_management JSONB NOT NULL DEFAULT '{}', -- 风险管理配置
    performance_metrics JSONB NOT NULL DEFAULT '{}', -- 性能指标
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT strategies_name_user_unique UNIQUE (user_id, name)
);

-- 策略执行记录表
CREATE TABLE IF NOT EXISTS strategy_executions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    execution_type VARCHAR(20) NOT NULL DEFAULT 'LIVE', -- 'LIVE', 'BACKTEST', 'PAPER'
    status VARCHAR(20) NOT NULL DEFAULT 'RUNNING', -- 'RUNNING', 'STOPPED', 'ERROR', 'COMPLETED'
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ,
    initial_capital NUMERIC(20,8),
    current_capital NUMERIC(20,8),
    total_pnl NUMERIC(20,8) DEFAULT 0,
    realized_pnl NUMERIC(20,8) DEFAULT 0,
    unrealized_pnl NUMERIC(20,8) DEFAULT 0,
    total_return NUMERIC(10,4), -- 总收益率
    annual_return NUMERIC(10,4), -- 年化收益率
    max_drawdown NUMERIC(10,4), -- 最大回撤
    sharpe_ratio NUMERIC(10,4), -- 夏普比率
    win_rate NUMERIC(5,2), -- 胜率
    trades_count INTEGER DEFAULT 0,
    winning_trades INTEGER DEFAULT 0,
    losing_trades INTEGER DEFAULT 0,
    error_message TEXT,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 交易信号表
CREATE TABLE IF NOT EXISTS trading_signals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    execution_id UUID REFERENCES strategy_executions(id) ON DELETE CASCADE,
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(50) NOT NULL DEFAULT 'binance',
    signal_type VARCHAR(10) NOT NULL, -- 'BUY', 'SELL', 'HOLD'
    strength NUMERIC(5,2) NOT NULL CHECK (strength >= 0 AND strength <= 100), -- 信号强度 0-100
    confidence NUMERIC(5,2) NOT NULL CHECK (confidence >= 0 AND confidence <= 100), -- 置信度 0-100
    entry_price NUMERIC(20,8),
    stop_loss NUMERIC(20,8),
    take_profit NUMERIC(20,8),
    quantity NUMERIC(20,8),
    indicators JSONB NOT NULL DEFAULT '{}', -- 技术指标值
    market_conditions JSONB NOT NULL DEFAULT '{}', -- 市场条件
    executed BOOLEAN NOT NULL DEFAULT false,
    execution_price NUMERIC(20,8),
    execution_time TIMESTAMPTZ,
    order_id UUID, -- 关联的订单ID
    pnl NUMERIC(20,8), -- 该信号的盈亏
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 回测结果表
CREATE TABLE IF NOT EXISTS backtest_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES strategies(id) ON DELETE CASCADE,
    name VARCHAR(200) NOT NULL,
    start_date TIMESTAMPTZ NOT NULL,
    end_date TIMESTAMPTZ NOT NULL,
    initial_capital NUMERIC(20,8) NOT NULL,
    final_capital NUMERIC(20,8) NOT NULL,
    total_return NUMERIC(10,4) NOT NULL,
    annual_return NUMERIC(10,4),
    volatility NUMERIC(10,4),
    sharpe_ratio NUMERIC(10,4),
    sortino_ratio NUMERIC(10,4),
    max_drawdown NUMERIC(10,4),
    max_drawdown_duration INTEGER, -- 最大回撤持续天数
    win_rate NUMERIC(5,2),
    profit_factor NUMERIC(10,4),
    total_trades INTEGER NOT NULL DEFAULT 0,
    winning_trades INTEGER NOT NULL DEFAULT 0,
    losing_trades INTEGER NOT NULL DEFAULT 0,
    avg_win NUMERIC(20,8),
    avg_loss NUMERIC(20,8),
    largest_win NUMERIC(20,8),
    largest_loss NUMERIC(20,8),
    equity_curve JSONB, -- 权益曲线数据
    trade_analysis JSONB, -- 交易分析数据
    risk_metrics JSONB, -- 风险指标
    parameters JSONB NOT NULL DEFAULT '{}', -- 回测参数
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 策略参数模板表
CREATE TABLE IF NOT EXISTS strategy_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL UNIQUE,
    strategy_type VARCHAR(50) NOT NULL,
    description TEXT,
    default_parameters JSONB NOT NULL DEFAULT '{}',
    parameter_schema JSONB NOT NULL DEFAULT '{}', -- 参数验证模式
    is_public BOOLEAN NOT NULL DEFAULT false,
    created_by UUID,
    usage_count INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 技术指标缓存表
CREATE TABLE IF NOT EXISTS indicator_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(20) NOT NULL,
    timeframe VARCHAR(10) NOT NULL,
    indicator_type VARCHAR(50) NOT NULL, -- 'MA', 'EMA', 'RSI', 'MACD', 'BOLLINGER', etc.
    parameters JSONB NOT NULL DEFAULT '{}',
    timestamp TIMESTAMPTZ NOT NULL,
    value NUMERIC(20,8),
    values JSONB, -- 多值指标（如MACD有多个输出）
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT indicator_cache_unique UNIQUE (symbol, timeframe, indicator_type, parameters, timestamp)
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_strategies_user_id ON strategies(user_id);
CREATE INDEX IF NOT EXISTS idx_strategies_status ON strategies(status);
CREATE INDEX IF NOT EXISTS idx_strategies_type ON strategies(strategy_type);
CREATE INDEX IF NOT EXISTS idx_strategies_created_at ON strategies(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_strategy_executions_strategy_id ON strategy_executions(strategy_id);
CREATE INDEX IF NOT EXISTS idx_strategy_executions_status ON strategy_executions(status);
CREATE INDEX IF NOT EXISTS idx_strategy_executions_start_time ON strategy_executions(start_time DESC);

CREATE INDEX IF NOT EXISTS idx_trading_signals_strategy_id ON trading_signals(strategy_id);
CREATE INDEX IF NOT EXISTS idx_trading_signals_symbol ON trading_signals(symbol);
CREATE INDEX IF NOT EXISTS idx_trading_signals_created_at ON trading_signals(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_trading_signals_executed ON trading_signals(executed);

CREATE INDEX IF NOT EXISTS idx_backtest_results_strategy_id ON backtest_results(strategy_id);
CREATE INDEX IF NOT EXISTS idx_backtest_results_created_at ON backtest_results(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_indicator_cache_symbol_timeframe ON indicator_cache(symbol, timeframe);
CREATE INDEX IF NOT EXISTS idx_indicator_cache_timestamp ON indicator_cache(timestamp DESC);

-- 创建更新时间触发器
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = now();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_strategies_updated_at BEFORE UPDATE ON strategies FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_strategy_executions_updated_at BEFORE UPDATE ON strategy_executions FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_strategy_templates_updated_at BEFORE UPDATE ON strategy_templates FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- 插入默认策略模板
INSERT INTO strategy_templates (name, strategy_type, description, default_parameters, parameter_schema) VALUES
('MA_CROSSOVER', 'TREND_FOLLOWING', 'Moving Average Crossover Strategy', 
 '{"fast_period": 10, "slow_period": 30, "stop_loss": 0.02, "take_profit": 0.06}',
 '{"fast_period": {"type": "integer", "min": 5, "max": 50}, "slow_period": {"type": "integer", "min": 20, "max": 200}}'),
 
('RSI_MEAN_REVERSION', 'MEAN_REVERSION', 'RSI Mean Reversion Strategy',
 '{"rsi_period": 14, "oversold": 30, "overbought": 70, "stop_loss": 0.03, "take_profit": 0.05}',
 '{"rsi_period": {"type": "integer", "min": 10, "max": 30}, "oversold": {"type": "number", "min": 20, "max": 40}}'),
 
('GRID_TRADING', 'GRID', 'Grid Trading Strategy',
 '{"grid_size": 0.01, "grid_count": 10, "base_amount": 100, "price_range": 0.1}',
 '{"grid_size": {"type": "number", "min": 0.001, "max": 0.1}, "grid_count": {"type": "integer", "min": 5, "max": 50}}')
ON CONFLICT (name) DO NOTHING;

COMMENT ON TABLE strategies IS 'Strategy definitions table';
COMMENT ON TABLE strategy_executions IS 'Strategy execution records table';
COMMENT ON TABLE trading_signals IS 'Trading signals table';
COMMENT ON TABLE backtest_results IS 'Backtest results table';
COMMENT ON TABLE strategy_templates IS 'Strategy templates table';
COMMENT ON TABLE indicator_cache IS 'Technical indicators cache table';