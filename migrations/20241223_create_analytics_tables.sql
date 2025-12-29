-- 分析服务数据库表结构
-- 创建时间: 2024-12-23
-- 服务: analytics (Port 8087)

-- 性能报告表
CREATE TABLE IF NOT EXISTS performance_reports (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    report_type VARCHAR(50) NOT NULL, -- 'DAILY', 'WEEKLY', 'MONTHLY', 'QUARTERLY', 'YEARLY', 'CUSTOM'
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    
    -- 基础指标
    initial_capital NUMERIC(20,8) NOT NULL,
    final_capital NUMERIC(20,8) NOT NULL,
    total_return NUMERIC(10,4) NOT NULL, -- 总收益率
    annual_return NUMERIC(10,4), -- 年化收益率
    volatility NUMERIC(10,4), -- 波动率
    
    -- 风险指标
    sharpe_ratio NUMERIC(10,4), -- 夏普比率
    sortino_ratio NUMERIC(10,4), -- 索提诺比率
    calmar_ratio NUMERIC(10,4), -- 卡玛比率
    max_drawdown NUMERIC(10,4), -- 最大回撤
    max_drawdown_duration INTEGER, -- 最大回撤持续天数
    var_95 NUMERIC(20,8), -- 95% VaR
    cvar_95 NUMERIC(20,8), -- 95% CVaR
    
    -- 交易统计
    total_trades INTEGER NOT NULL DEFAULT 0,
    winning_trades INTEGER NOT NULL DEFAULT 0,
    losing_trades INTEGER NOT NULL DEFAULT 0,
    win_rate NUMERIC(5,2), -- 胜率
    profit_factor NUMERIC(10,4), -- 盈利因子
    avg_win NUMERIC(20,8), -- 平均盈利
    avg_loss NUMERIC(20,8), -- 平均亏损
    largest_win NUMERIC(20,8), -- 最大盈利
    largest_loss NUMERIC(20,8), -- 最大亏损
    
    -- 详细数据
    equity_curve JSONB, -- 权益曲线
    monthly_returns JSONB, -- 月度收益
    drawdown_periods JSONB, -- 回撤期间
    trade_distribution JSONB, -- 交易分布
    sector_allocation JSONB, -- 行业配置
    
    -- 元数据
    report_data JSONB NOT NULL DEFAULT '{}', -- 完整报告数据
    generated_by VARCHAR(50) DEFAULT 'system',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 投资组合分析表
CREATE TABLE IF NOT EXISTS portfolio_analysis (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    analysis_date TIMESTAMPTZ NOT NULL,
    
    -- 组合概况
    total_value NUMERIC(20,8) NOT NULL,
    cash_balance NUMERIC(20,8) NOT NULL DEFAULT 0,
    invested_amount NUMERIC(20,8) NOT NULL DEFAULT 0,
    unrealized_pnl NUMERIC(20,8) NOT NULL DEFAULT 0,
    realized_pnl NUMERIC(20,8) NOT NULL DEFAULT 0,
    
    -- 持仓分析
    positions_count INTEGER NOT NULL DEFAULT 0,
    long_positions INTEGER NOT NULL DEFAULT 0,
    short_positions INTEGER NOT NULL DEFAULT 0,
    largest_position_pct NUMERIC(5,2), -- 最大持仓占比
    
    -- 风险分析
    portfolio_beta NUMERIC(10,4), -- 组合贝塔
    portfolio_alpha NUMERIC(10,4), -- 组合阿尔法
    tracking_error NUMERIC(10,4), -- 跟踪误差
    information_ratio NUMERIC(10,4), -- 信息比率
    concentration_risk NUMERIC(5,2), -- 集中度风险
    
    -- 配置分析
    asset_allocation JSONB NOT NULL DEFAULT '{}', -- 资产配置
    sector_weights JSONB NOT NULL DEFAULT '{}', -- 行业权重
    geographic_allocation JSONB NOT NULL DEFAULT '{}', -- 地理配置
    currency_exposure JSONB NOT NULL DEFAULT '{}', -- 货币敞口
    
    -- 相关性分析
    correlation_matrix JSONB, -- 相关性矩阵
    diversification_ratio NUMERIC(10,4), -- 分散化比率
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 相关性数据表
CREATE TABLE IF NOT EXISTS correlation_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol_1 VARCHAR(20) NOT NULL,
    symbol_2 VARCHAR(20) NOT NULL,
    timeframe VARCHAR(10) NOT NULL, -- '1d', '1w', '1m', '3m', '6m', '1y'
    correlation_coefficient NUMERIC(10,6) NOT NULL, -- 相关系数
    p_value NUMERIC(10,6), -- p值
    sample_size INTEGER NOT NULL, -- 样本数量
    calculation_date TIMESTAMPTZ NOT NULL,
    
    -- 滚动相关性
    rolling_correlation JSONB, -- 滚动相关性数据
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT correlation_data_unique UNIQUE (symbol_1, symbol_2, timeframe, calculation_date)
);

-- 统计缓存表
CREATE TABLE IF NOT EXISTS statistics_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cache_key VARCHAR(255) NOT NULL UNIQUE,
    cache_type VARCHAR(50) NOT NULL, -- 'MARKET_STATS', 'USER_STATS', 'TRADING_STATS', 'PERFORMANCE_STATS'
    user_id UUID, -- 用户相关的缓存
    symbol VARCHAR(20), -- 交易对相关的缓存
    timeframe VARCHAR(10), -- 时间周期
    
    -- 缓存数据
    data JSONB NOT NULL,
    
    -- 缓存管理
    expires_at TIMESTAMPTZ NOT NULL,
    hit_count INTEGER NOT NULL DEFAULT 0,
    last_accessed TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 市场统计表
CREATE TABLE IF NOT EXISTS market_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    date DATE NOT NULL,
    
    -- 市场概况
    total_volume NUMERIC(20,8) NOT NULL DEFAULT 0,
    total_trades INTEGER NOT NULL DEFAULT 0,
    active_symbols INTEGER NOT NULL DEFAULT 0,
    
    -- 价格统计
    avg_price_change NUMERIC(10,4), -- 平均价格变化
    volatility_index NUMERIC(10,4), -- 波动率指数
    
    -- 交易统计
    top_gainers JSONB, -- 涨幅榜
    top_losers JSONB, -- 跌幅榜
    most_active JSONB, -- 最活跃
    
    -- 市场情绪
    fear_greed_index NUMERIC(5,2), -- 恐惧贪婪指数
    market_sentiment VARCHAR(20), -- 'BULLISH', 'BEARISH', 'NEUTRAL'
    
    -- 详细数据
    sector_performance JSONB, -- 行业表现
    market_cap_distribution JSONB, -- 市值分布
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT market_statistics_date_unique UNIQUE (date)
);

-- 用户统计表
CREATE TABLE IF NOT EXISTS user_statistics (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    date DATE NOT NULL,
    
    -- 交易统计
    trades_count INTEGER NOT NULL DEFAULT 0,
    volume_traded NUMERIC(20,8) NOT NULL DEFAULT 0,
    fees_paid NUMERIC(20,8) NOT NULL DEFAULT 0,
    
    -- 盈亏统计
    realized_pnl NUMERIC(20,8) NOT NULL DEFAULT 0,
    unrealized_pnl NUMERIC(20,8) NOT NULL DEFAULT 0,
    total_pnl NUMERIC(20,8) NOT NULL DEFAULT 0,
    
    -- 持仓统计
    positions_count INTEGER NOT NULL DEFAULT 0,
    portfolio_value NUMERIC(20,8) NOT NULL DEFAULT 0,
    cash_balance NUMERIC(20,8) NOT NULL DEFAULT 0,
    
    -- 风险统计
    max_drawdown NUMERIC(10,4),
    var_1d NUMERIC(20,8), -- 1日VaR
    leverage_ratio NUMERIC(10,2),
    
    -- 活动统计
    login_count INTEGER NOT NULL DEFAULT 0,
    api_calls INTEGER NOT NULL DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT user_statistics_user_date_unique UNIQUE (user_id, date)
);

-- 报告模板表
CREATE TABLE IF NOT EXISTS report_templates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(200) NOT NULL UNIQUE,
    description TEXT,
    report_type VARCHAR(50) NOT NULL,
    
    -- 模板配置
    template_config JSONB NOT NULL DEFAULT '{}',
    chart_configs JSONB NOT NULL DEFAULT '[]',
    table_configs JSONB NOT NULL DEFAULT '[]',
    
    -- 权限和使用
    is_public BOOLEAN NOT NULL DEFAULT false,
    created_by UUID,
    usage_count INTEGER NOT NULL DEFAULT 0,
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_performance_reports_user_id ON performance_reports(user_id);
CREATE INDEX IF NOT EXISTS idx_performance_reports_type ON performance_reports(report_type);
CREATE INDEX IF NOT EXISTS idx_performance_reports_period ON performance_reports(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_performance_reports_created_at ON performance_reports(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_portfolio_analysis_user_id ON portfolio_analysis(user_id);
CREATE INDEX IF NOT EXISTS idx_portfolio_analysis_date ON portfolio_analysis(analysis_date DESC);

CREATE INDEX IF NOT EXISTS idx_correlation_data_symbols ON correlation_data(symbol_1, symbol_2);
CREATE INDEX IF NOT EXISTS idx_correlation_data_timeframe ON correlation_data(timeframe);
CREATE INDEX IF NOT EXISTS idx_correlation_data_date ON correlation_data(calculation_date DESC);

CREATE INDEX IF NOT EXISTS idx_statistics_cache_key ON statistics_cache(cache_key);
CREATE INDEX IF NOT EXISTS idx_statistics_cache_type ON statistics_cache(cache_type);
CREATE INDEX IF NOT EXISTS idx_statistics_cache_expires ON statistics_cache(expires_at);
CREATE INDEX IF NOT EXISTS idx_statistics_cache_user_id ON statistics_cache(user_id) WHERE user_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_market_statistics_date ON market_statistics(date DESC);

CREATE INDEX IF NOT EXISTS idx_user_statistics_user_id ON user_statistics(user_id);
CREATE INDEX IF NOT EXISTS idx_user_statistics_date ON user_statistics(date DESC);

-- 创建更新时间触发器
CREATE TRIGGER update_performance_reports_updated_at BEFORE UPDATE ON performance_reports FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_statistics_cache_updated_at BEFORE UPDATE ON statistics_cache FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_report_templates_updated_at BEFORE UPDATE ON report_templates FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- 插入默认报告模板
INSERT INTO report_templates (name, report_type, description, template_config) VALUES
('DAILY_TRADING_REPORT', 'DAILY', 'Daily trading activity and PnL report', 
 '{"sections": ["summary", "trades", "pnl", "positions"], "charts": ["equity_curve", "daily_pnl"]}'),
 
('MONTHLY_PORTFOLIO_REPORT', 'MONTHLY', 'Monthly portfolio analysis and risk assessment report',
 '{"sections": ["portfolio_overview", "performance", "risk_analysis", "allocation"], "charts": ["portfolio_pie", "performance_chart", "drawdown"]}'),
 
('ANNUAL_PERFORMANCE_REPORT', 'YEARLY', 'Annual investment performance and risk metrics report',
 '{"sections": ["annual_summary", "performance_metrics", "risk_metrics", "benchmark_comparison"], "charts": ["annual_returns", "rolling_sharpe", "correlation_heatmap"]}')
ON CONFLICT (name) DO NOTHING;

COMMENT ON TABLE performance_reports IS 'Performance reports table';
COMMENT ON TABLE portfolio_analysis IS 'Portfolio analysis table';
COMMENT ON TABLE correlation_data IS 'Correlation data table';
COMMENT ON TABLE statistics_cache IS 'Statistics cache table';
COMMENT ON TABLE market_statistics IS 'Market statistics table';
COMMENT ON TABLE user_statistics IS 'User statistics table';
COMMENT ON TABLE report_templates IS 'Report templates table';