-- AI智能服务数据库表结构
-- 创建时间: 2024-12-23
-- 服务: ai-service (Port 8088)

-- 价格预测表
CREATE TABLE IF NOT EXISTS price_predictions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(50) NOT NULL DEFAULT 'binance',
    
    -- 预测输入
    current_price NUMERIC(20,8) NOT NULL,
    timeframe VARCHAR(10) NOT NULL, -- '1m', '5m', '15m', '1h', '4h', '1d'
    horizon INTEGER NOT NULL, -- 预测时间范围(分钟)
    
    -- 预测输出
    predicted_price NUMERIC(20,8) NOT NULL,
    direction VARCHAR(10) NOT NULL, -- 'UP', 'DOWN', 'SIDEWAYS'
    confidence NUMERIC(5,2) NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    
    -- 预测区间
    price_lower NUMERIC(20,8), -- 预测价格下限
    price_upper NUMERIC(20,8), -- 预测价格上限
    
    -- 模型信息
    model_name VARCHAR(100) NOT NULL,
    model_version VARCHAR(50) NOT NULL,
    features_used JSONB NOT NULL DEFAULT '[]', -- 使用的特征
    
    -- 验证结果
    actual_price NUMERIC(20,8), -- 实际价格
    accuracy_score NUMERIC(5,2), -- 准确度评分
    is_verified BOOLEAN NOT NULL DEFAULT false,
    verified_at TIMESTAMPTZ,
    
    -- 元数据
    prediction_metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 套利机会表
CREATE TABLE IF NOT EXISTS arbitrage_opportunities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(20) NOT NULL,
    
    -- 交易所信息
    buy_exchange VARCHAR(50) NOT NULL,
    sell_exchange VARCHAR(50) NOT NULL,
    
    -- 价格信息
    buy_price NUMERIC(20,8) NOT NULL,
    sell_price NUMERIC(20,8) NOT NULL,
    price_diff NUMERIC(20,8) NOT NULL,
    
    -- 盈利分析
    profit_amount NUMERIC(20,8) NOT NULL,
    profit_percentage NUMERIC(10,4) NOT NULL,
    
    -- 风险评估
    confidence NUMERIC(5,2) NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    risk_score NUMERIC(5,2) NOT NULL CHECK (risk_score >= 0 AND risk_score <= 100),
    
    -- 执行分析
    estimated_execution_time INTEGER, -- 预计执行时间(秒)
    min_volume NUMERIC(20,8), -- 最小交易量
    max_volume NUMERIC(20,8), -- 最大交易量
    
    -- 市场条件
    buy_orderbook_depth NUMERIC(20,8), -- 买方订单簿深度
    sell_orderbook_depth NUMERIC(20,8), -- 卖方订单簿深度
    volume_24h NUMERIC(20,8), -- 24小时交易量
    
    -- 状态跟踪
    status VARCHAR(20) NOT NULL DEFAULT 'DETECTED', -- 'DETECTED', 'EXECUTING', 'EXECUTED', 'EXPIRED', 'FAILED'
    executed_at TIMESTAMPTZ,
    actual_profit NUMERIC(20,8),
    
    -- 元数据
    detection_metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (now() + INTERVAL '5 minutes')
);

-- 模型性能表
CREATE TABLE IF NOT EXISTS model_performance (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    model_name VARCHAR(100) NOT NULL,
    model_version VARCHAR(50) NOT NULL,
    model_type VARCHAR(50) NOT NULL, -- 'PRICE_PREDICTION', 'TREND_PREDICTION', 'ARBITRAGE_DETECTION', 'SIGNAL_GENERATION'
    
    -- 性能指标
    accuracy NUMERIC(5,2), -- 准确率
    precision_score NUMERIC(5,2), -- 精确率
    recall NUMERIC(5,2), -- 召回率
    f1_score NUMERIC(5,2), -- F1分数
    mse NUMERIC(20,8), -- 均方误差
    mae NUMERIC(20,8), -- 平均绝对误差
    
    -- 时间性能
    avg_prediction_time NUMERIC(10,3), -- 平均预测时间(毫秒)
    max_prediction_time NUMERIC(10,3), -- 最大预测时间(毫秒)
    
    -- 统计信息
    total_predictions INTEGER NOT NULL DEFAULT 0,
    correct_predictions INTEGER NOT NULL DEFAULT 0,
    evaluation_period_start TIMESTAMPTZ NOT NULL,
    evaluation_period_end TIMESTAMPTZ NOT NULL,
    
    -- 详细指标
    performance_details JSONB NOT NULL DEFAULT '{}',
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    CONSTRAINT model_performance_unique UNIQUE (model_name, model_version, evaluation_period_start)
);

-- 训练数据表
CREATE TABLE IF NOT EXISTS training_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dataset_name VARCHAR(200) NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    timeframe VARCHAR(10) NOT NULL,
    
    -- 数据范围
    start_time TIMESTAMPTZ NOT NULL,
    end_time TIMESTAMPTZ NOT NULL,
    record_count INTEGER NOT NULL,
    
    -- 特征信息
    features JSONB NOT NULL DEFAULT '[]', -- 特征列表
    target_variable VARCHAR(100) NOT NULL, -- 目标变量
    
    -- 数据质量
    missing_values_pct NUMERIC(5,2), -- 缺失值比例
    outliers_pct NUMERIC(5,2), -- 异常值比例
    data_quality_score NUMERIC(5,2), -- 数据质量评分
    
    -- 预处理信息
    preprocessing_steps JSONB NOT NULL DEFAULT '[]',
    normalization_params JSONB NOT NULL DEFAULT '{}',
    
    -- 数据分割
    train_split NUMERIC(5,2) NOT NULL DEFAULT 70, -- 训练集比例
    validation_split NUMERIC(5,2) NOT NULL DEFAULT 15, -- 验证集比例
    test_split NUMERIC(5,2) NOT NULL DEFAULT 15, -- 测试集比例
    
    -- 存储信息
    storage_path TEXT, -- 数据存储路径
    file_size BIGINT, -- 文件大小(字节)
    
    -- 元数据
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- AI信号表
CREATE TABLE IF NOT EXISTS ai_signals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol VARCHAR(20) NOT NULL,
    exchange VARCHAR(50) NOT NULL DEFAULT 'binance',
    
    -- 信号信息
    signal_type VARCHAR(10) NOT NULL, -- 'BUY', 'SELL', 'HOLD'
    strength NUMERIC(5,2) NOT NULL CHECK (strength >= 0 AND strength <= 100),
    confidence NUMERIC(5,2) NOT NULL CHECK (confidence >= 0 AND confidence <= 100),
    
    -- 价格信息
    current_price NUMERIC(20,8) NOT NULL,
    entry_price NUMERIC(20,8),
    stop_loss NUMERIC(20,8),
    take_profit NUMERIC(20,8),
    
    -- AI分析
    model_predictions JSONB NOT NULL DEFAULT '{}', -- 各模型预测结果
    feature_importance JSONB NOT NULL DEFAULT '{}', -- 特征重要性
    market_regime VARCHAR(50), -- 市场状态
    
    -- 风险评估
    risk_score NUMERIC(5,2) NOT NULL CHECK (risk_score >= 0 AND risk_score <= 100),
    volatility_forecast NUMERIC(10,4), -- 波动率预测
    
    -- 执行跟踪
    is_executed BOOLEAN NOT NULL DEFAULT false,
    execution_price NUMERIC(20,8),
    execution_time TIMESTAMPTZ,
    pnl NUMERIC(20,8), -- 盈亏
    
    -- 元数据
    signal_metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 模型配置表
CREATE TABLE IF NOT EXISTS model_configs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    model_name VARCHAR(100) NOT NULL UNIQUE,
    model_type VARCHAR(50) NOT NULL,
    
    -- 模型文件
    model_path TEXT NOT NULL, -- 模型文件路径
    model_size BIGINT, -- 模型文件大小
    
    -- 配置参数
    hyperparameters JSONB NOT NULL DEFAULT '{}',
    feature_config JSONB NOT NULL DEFAULT '{}',
    preprocessing_config JSONB NOT NULL DEFAULT '{}',
    
    -- 训练信息
    training_dataset VARCHAR(200),
    training_start TIMESTAMPTZ,
    training_end TIMESTAMPTZ,
    training_duration INTEGER, -- 训练时长(秒)
    
    -- 版本信息
    version VARCHAR(50) NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    
    -- 性能要求
    max_prediction_time INTEGER, -- 最大预测时间(毫秒)
    min_accuracy NUMERIC(5,2), -- 最小准确率要求
    
    -- 元数据
    description TEXT,
    created_by VARCHAR(100),
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 预测缓存表 (Redis的数据库备份)
CREATE TABLE IF NOT EXISTS prediction_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cache_key VARCHAR(255) NOT NULL UNIQUE,
    symbol VARCHAR(20) NOT NULL,
    model_name VARCHAR(100) NOT NULL,
    
    -- 缓存数据
    prediction_data JSONB NOT NULL,
    
    -- 缓存管理
    expires_at TIMESTAMPTZ NOT NULL,
    hit_count INTEGER NOT NULL DEFAULT 0,
    last_accessed TIMESTAMPTZ NOT NULL DEFAULT now(),
    
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- 创建索引
CREATE INDEX IF NOT EXISTS idx_price_predictions_symbol ON price_predictions(symbol);
CREATE INDEX IF NOT EXISTS idx_price_predictions_created_at ON price_predictions(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_price_predictions_model ON price_predictions(model_name, model_version);
CREATE INDEX IF NOT EXISTS idx_price_predictions_verified ON price_predictions(is_verified);

CREATE INDEX IF NOT EXISTS idx_arbitrage_opportunities_symbol ON arbitrage_opportunities(symbol);
CREATE INDEX IF NOT EXISTS idx_arbitrage_opportunities_created_at ON arbitrage_opportunities(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_arbitrage_opportunities_status ON arbitrage_opportunities(status);
CREATE INDEX IF NOT EXISTS idx_arbitrage_opportunities_profit ON arbitrage_opportunities(profit_percentage DESC);
CREATE INDEX IF NOT EXISTS idx_arbitrage_opportunities_expires ON arbitrage_opportunities(expires_at);

CREATE INDEX IF NOT EXISTS idx_model_performance_model ON model_performance(model_name, model_version);
CREATE INDEX IF NOT EXISTS idx_model_performance_type ON model_performance(model_type);
CREATE INDEX IF NOT EXISTS idx_model_performance_created_at ON model_performance(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_training_data_symbol ON training_data(symbol);
CREATE INDEX IF NOT EXISTS idx_training_data_timeframe ON training_data(timeframe);
CREATE INDEX IF NOT EXISTS idx_training_data_created_at ON training_data(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_ai_signals_symbol ON ai_signals(symbol);
CREATE INDEX IF NOT EXISTS idx_ai_signals_created_at ON ai_signals(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_signals_executed ON ai_signals(is_executed);
CREATE INDEX IF NOT EXISTS idx_ai_signals_strength ON ai_signals(strength DESC);

CREATE INDEX IF NOT EXISTS idx_model_configs_name ON model_configs(model_name);
CREATE INDEX IF NOT EXISTS idx_model_configs_type ON model_configs(model_type);
CREATE INDEX IF NOT EXISTS idx_model_configs_active ON model_configs(is_active);

CREATE INDEX IF NOT EXISTS idx_prediction_cache_key ON prediction_cache(cache_key);
CREATE INDEX IF NOT EXISTS idx_prediction_cache_symbol ON prediction_cache(symbol);
CREATE INDEX IF NOT EXISTS idx_prediction_cache_expires ON prediction_cache(expires_at);

-- 创建更新时间触发器
CREATE TRIGGER update_training_data_updated_at BEFORE UPDATE ON training_data FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
CREATE TRIGGER update_model_configs_updated_at BEFORE UPDATE ON model_configs FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- 插入默认模型配置
INSERT INTO model_configs (model_name, model_type, model_path, version, hyperparameters, description) VALUES
('LSTM_PRICE_PREDICTION', 'PRICE_PREDICTION', '/models/lstm_price_v1.pkl', 'v1.0', 
 '{"sequence_length": 60, "hidden_units": 50, "dropout": 0.2, "learning_rate": 0.001}',
 'LSTM-based short-term price prediction model'),
 
('RF_TREND_PREDICTION', 'TREND_PREDICTION', '/models/rf_trend_v1.pkl', 'v1.0',
 '{"n_estimators": 100, "max_depth": 10, "min_samples_split": 5}',
 'Random Forest trend direction prediction model'),
 
('ARBITRAGE_DETECTOR', 'ARBITRAGE_DETECTION', '/models/arbitrage_detector_v1.pkl', 'v1.0',
 '{"threshold": 0.001, "min_volume": 1000, "max_execution_time": 300}',
 'Real-time arbitrage opportunity detection model')
ON CONFLICT (model_name) DO NOTHING;

-- 创建清理过期数据的函数
CREATE OR REPLACE FUNCTION cleanup_expired_ai_data()
RETURNS INTEGER AS $$
DECLARE
    deleted_count INTEGER := 0;
BEGIN
    -- 清理过期的套利机会
    DELETE FROM arbitrage_opportunities WHERE expires_at < now();
    GET DIAGNOSTICS deleted_count = ROW_COUNT;
    
    -- 清理过期的预测缓存
    DELETE FROM prediction_cache WHERE expires_at < now();
    
    -- 清理30天前的价格预测记录
    DELETE FROM price_predictions WHERE created_at < now() - INTERVAL '30 days';
    
    -- 清理7天前的AI信号记录
    DELETE FROM ai_signals WHERE created_at < now() - INTERVAL '7 days';
    
    RETURN deleted_count;
END;
$$ LANGUAGE plpgsql;

COMMENT ON TABLE price_predictions IS 'AI price prediction records table';
COMMENT ON TABLE arbitrage_opportunities IS 'Arbitrage opportunity detection table';
COMMENT ON TABLE model_performance IS 'AI model performance evaluation table';
COMMENT ON TABLE training_data IS 'Training dataset management table';
COMMENT ON TABLE ai_signals IS 'AI-generated trading signals table';
COMMENT ON TABLE model_configs IS 'AI model configuration table';
COMMENT ON TABLE prediction_cache IS 'Prediction results cache table';