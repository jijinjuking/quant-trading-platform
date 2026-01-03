# AI智能服务 (ai-service) - 架构设计

## 📋 服务概述

### 服务名称
AI智能服务 (AI Service)

### 服务端口
8088

### 服务职责
- 价格预测 (机器学习模型)
- 套利机会发现
- 智能信号生成
- 模型管理 (加载、更新、版本控制)
- 预测结果缓存

## 🏗️ 服务架构

### 内部架构图
```
services/ai-service/
│
├── src/
│   │
│   ├── main.rs                 # 服务入口，启动HTTP服务器和模型管理器
│   │
│   ├── state.rs                # 应用状态管理，持有所有组件的Arc引用
│   │
│   ├── config/                 # 配置管理
│   │   ├── mod.rs              # 配置结构体定义
│   │   └── settings.rs         # 配置加载逻辑
│   │
│   ├── handlers/               # HTTP接口层
│   │   ├── mod.rs              # 路由注册
│   │   ├── predict.rs          # 预测接口
│   │   ├── arbitrage.rs        # 套利接口
│   │   ├── signals.rs          # 信号接口
│   │   ├── models.rs           # 模型管理接口
│   │   └── analysis.rs         # 分析接口
│   │
│   ├── models/                 # AI模型
│   │   ├── mod.rs              # 模型管理器
│   │   ├── price_predictor.rs  # 价格预测模型
│   │   ├── arbitrage_detector.rs # 套利检测模型
│   │   ├── signal_generator.rs # 信号生成模型
│   │   ├── trend_analyzer.rs   # 趋势分析模型
│   │   └── pattern_recognizer.rs # 形态识别模型
│   │
│   ├── ml/                     # 机器学习组件
│   │   ├── mod.rs              # ML管理
│   │   ├── data_preprocessor.rs # 数据预处理
│   │   ├── feature_engineer.rs  # 特征工程
│   │   ├── trainer.rs          # 模型训练器
│   │   ├── evaluator.rs        # 模型评估器
│   │   └── predictor.rs        # 预测器
│   │
│   ├── algorithms/             # 算法实现
│   │   ├── mod.rs              # 算法管理
│   │   ├── lstm.rs             # LSTM算法
│   │   ├── transformer.rs      # Transformer算法
│   │   ├── random_forest.rs    # 随机森林算法
│   │   ├── svm.rs              # SVM算法
│   │   └── ensemble.rs         # 集成算法
│   │
│   ├── storage/                # 数据存储层
│   │   ├── mod.rs              # 存储接口
│   │   ├── postgres_store.rs   # PostgreSQL存储
│   │   └── redis_cache.rs      # Redis缓存
│   │
│   ├── services/               # 业务逻辑层
│   │   ├── mod.rs              # 服务管理
│   │   ├── prediction_service.rs # 预测服务
│   │   ├── arbitrage_service.rs  # 套利服务
│   │   ├── signal_service.rs     # 信号服务
│   │   ├── model_service.rs      # 模型服务
│   │   └── analysis_service.rs   # 分析服务
│   │
│   └── utils/                  # 工具函数
│       ├── mod.rs
│       ├── model_loader.rs     # 模型加载工具
│       ├── data_validator.rs   # 数据验证工具
│       └── confidence_calculator.rs # 置信度计算工具
│
└── Cargo.toml                  # 依赖声明
```

## 🔄 数据流向

### 价格预测流程
```
市场数据 (来自market-data)
    ↓
ml/data_preprocessor.rs (数据预处理)
    ↓
algorithms/lstm.rs (LSTM预测)
    ↓
models/price_predictor.rs (模型预测)
    ↓
services/prediction_service.rs (预测服务)
    ↓
storage/redis_cache.rs (结果缓存)
    ↓
handlers/predict.rs (返回预测结果)
```

### 套利检测流程
```
多交易所市场数据
    ↓
models/arbitrage_detector.rs (套利检测)
    ↓
algorithms/ (套利算法)
    ↓
services/arbitrage_service.rs (套利服务)
    ↓
返回套利机会
```

## 📡 API接口设计

### 价格预测
```http
POST /api/v1/predict/price      # 价格预测
POST /api/v1/predict/trend      # 趋势预测
POST /api/v1/predict/momentum   # 动量预测
POST /api/v1/predict/volatility # 波动率预测
GET  /api/v1/predict/history    # 预测历史
POST /api/v1/predict/batch      # 批量预测
```

### 套利机会
```http
POST /api/v1/arbitrage/opportunities # 套利机会发现
POST /api/v1/arbitrage/analyze      # 套利分析
GET  /api/v1/arbitrage/opportunities # 查询套利机会
POST /api/v1/arbitrage/simulate     # 套利模拟
GET  /api/v1/arbitrage/performance  # 套利表现
```

### 智能信号
```http
POST /api/v1/signals/generate   # 生成交易信号
POST /api/v1/signals/evaluate   # 评估信号质量
GET  /api/v1/signals/history    # 信号历史
POST /api/v1/signals/validate   # 信号验证
GET  /api/v1/signals/statistics # 信号统计
```

### 模型管理
```http
GET  /api/v1/models/list        # 获取模型列表
GET  /api/v1/models/status      # 获取模型状态
POST /api/v1/models/reload      # 重新加载模型
POST /api/v1/models/train       # 训练模型
POST /api/v1/models/evaluate    # 评估模型
GET  /api/v1/models/metrics     # 获取模型指标
POST /api/v1/models/deploy      # 部署模型
```

### AI分析
```http
POST /api/v1/ai/analyze         # AI分析
GET  /api/v1/ai/patterns       # 形态识别
POST /api/v1/ai/forecast        # 市场预测
GET  /api/v1/ai/confidence      # 置信度查询
POST /api/v1/ai/ensemble        # 集成预测
```

## 🗄️ 数据模型

### 核心数据结构
```rust
// 价格预测请求模型
pub struct PricePredictionRequest {
    pub symbol: String,
    pub exchange: String,
    pub timeframe: String,        // 时间框架 (1m, 5m, 1h, etc.)
    pub horizon: u32,            // 预测时间范围 (分钟)
    pub features: Vec<String>,    // 使用的特征
    pub model_type: ModelType,    // 模型类型
    pub lookback_period: u32,     // 回看周期 (分钟)
}

pub enum ModelType {
    LSTM,
    Transformer,
    RandomForest,
    SVM,
    Ensemble,
    Custom(String),
}

// 价格预测响应模型
pub struct PricePredictionResponse {
    pub symbol: String,
    pub exchange: String,
    pub current_price: f64,
    pub predicted_price: f64,
    pub predicted_prices: Vec<f64>, // 多时间点预测
    pub confidence: f64,           // 预测置信度 (0-1)
    pub direction: PriceDirection, // 价格方向
    pub prediction_horizon: u32,   // 预测时间范围
    pub model_used: ModelType,     // 使用的模型
    pub features_used: Vec<String>, // 使用的特征
    pub prediction_timestamp: i64, // 预测时间戳
    pub confidence_interval: ConfidenceInterval, // 置信区间
    pub model_confidence: f64,     // 模型置信度
}

pub enum PriceDirection {
    Up,
    Down,
    Sideways,
    Unknown,
}

pub struct ConfidenceInterval {
    pub lower_bound: f64,         // 下界
    pub upper_bound: f64,         // 上界
    pub confidence_level: f64,    // 置信水平
}

// 套利机会模型
pub struct ArbitrageOpportunity {
    pub id: String,
    pub symbol: String,
    pub exchanges: Vec<ExchangePrice>, // 多交易所价格
    pub profit_amount: f64,
    pub profit_percentage: f64,
    pub confidence: f64,           // 机会置信度
    pub risk_score: f64,          // 风险评分
    pub estimated_execution_time: u32, // 预估执行时间 (毫秒)
    pub transaction_costs: f64,   // 交易成本
    pub net_profit: f64,          // 净利润
    pub opportunity_type: ArbitrageType, // 套利类型
    pub status: ArbitrageStatus,  // 机会状态
    pub created_at: i64,
    pub expires_at: i64,
}

pub struct ExchangePrice {
    pub exchange: String,
    pub price: f64,
    pub quantity: f64,
    pub timestamp: i64,
}

pub enum ArbitrageType {
    Spatial,        // 空间套利 (不同交易所)
    Triangular,     // 三角套利 (三个货币对)
    Convergence,    // 收敛套利
    Statistical,    // 统计套利
}

pub enum ArbitrageStatus {
    Available,      // 可用
    Executing,      // 执行中
    Executed,       // 已执行
    Expired,        // 已过期
    Risky,          // 风险过高
}

// 交易信号模型
pub struct TradingSignal {
    pub id: String,
    pub symbol: String,
    pub signal_type: SignalType,
    pub strength: f64,            // 信号强度
    pub confidence: f64,          // 信号置信度
    pub entry_price: f64,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub time_frame: String,
    pub indicators: Vec<IndicatorValue>,
    pub generated_at: i64,
    pub expires_at: Option<i64>,
    pub ai_model: String,         // 生成信号的AI模型
    pub ai_confidence: f64,      // AI置信度
    pub risk_level: RiskLevel,    // 风险等级
}

pub enum SignalType {
    Buy,
    Sell,
    StrongBuy,
    StrongSell,
    Hold,
    CloseLong,
    CloseShort,
}

pub struct IndicatorValue {
    pub name: String,
    pub value: f64,
    pub signal: IndicatorSignal,
}

pub enum IndicatorSignal {
    Bullish,
    Bearish,
    Neutral,
    Overbought,
    Oversold,
}

pub enum RiskLevel {
    Low,
    Medium,
    High,
    VeryHigh,
}

// 模型信息模型
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub model_type: ModelType,
    pub version: String,
    pub status: ModelState,
    pub accuracy: f64,            // 模型准确率
    pub precision: f64,           // 精确率
    pub recall: f64,              // 召回率
    pub f1_score: f64,            // F1分数
    pub training_data_size: u64,  // 训练数据量
    pub features: Vec<String>,    // 使用的特征
    pub input_shape: Vec<u32>,    // 输入形状
    pub output_shape: Vec<u32>,   // 输出形状
    pub training_date: i64,
    pub last_updated: i64,
    pub performance_metrics: PerformanceMetrics,
}

pub enum ModelState {
    Loading,          // 加载中
    Ready,            // 就绪
    Error,            // 错误
    Updating,         // 更新中
    Retraining,       // 重新训练中
    Uninitialized,    // 未初始化
}

pub struct PerformanceMetrics {
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub mean_absolute_error: f64,
    pub mean_squared_error: f64,
    pub root_mean_squared_error: f64,
    pub r_squared: f64,
    pub sharpe_ratio: f64,        // 夏普比率 (用于预测准确性)
    pub max_drawdown: f64,        // 最大回撤 (用于预测风险)
}
```

## 🔧 技术实现要点

### AI模型
- **深度学习**: LSTM、Transformer等神经网络
- **传统算法**: 随机森林、SVM等
- **集成学习**: 多模型集成预测
- **模型优化**: 模型压缩和加速

### 数据处理
- **特征工程**: 自动特征提取和选择
- **数据预处理**: 数据清洗和标准化
- **时间序列**: 时间序列数据处理
- **多维度分析**: 多因子模型

### 预测算法
- **价格预测**: 基于历史价格和指标预测
- **趋势预测**: 趋势识别和预测
- **波动率预测**: 波动率预测模型
- **套利检测**: 多市场套利机会识别

### 性能优化
- **模型缓存**: 模型和预测结果缓存
- **异步推理**: 异步AI推理处理
- **批量处理**: 批量预测处理
- **GPU加速**: 支持GPU加速计算

## 📊 监控指标

### 性能指标
- 预测响应时间
- 模型推理延迟
- 数据处理吞吐量
- 内存使用率

### 业务指标
- 预测准确率
- 信号成功率
- 套利机会发现率
- 模型性能指标

## 🔐 安全措施

- **认证授权**: JWT认证 + RBAC权限控制
- **模型安全**: 模型文件访问控制
- **数据安全**: 训练数据加密
- **API限流**: 防止滥用AI服务

## 🚀 部署配置

### 环境变量
```
AI_SERVICE_PORT=8088
DATABASE_URL=postgresql://user:pass@localhost/ai
REDIS_URL=redis://localhost:6379
MARKET_DATA_URL=http://market-data:8083
MODEL_STORAGE_PATH=/models
PREDICTION_CACHE_TTL=300
MAX_PREDICTION_LOOKBACK=1000
MODEL_RETRAIN_INTERVAL=86400
AI_PREDICTION_TIMEOUT=30
GPU_ENABLED=false
MAX_CONCURRENT_PREDICTIONS=10
MODEL_WARMUP_ENABLED=true
PREDICTION_CONFIDENCE_THRESHOLD=0.7
ARBITRAGE_MIN_PROFIT=0.01
ARBITRAGE_MAX_RISK=0.1
SIGNAL_MIN_CONFIDENCE=0.6
```

### Docker配置
- 多阶段构建
- GPU支持 (可选)
- 资源限制
- 健康检查

## 🧪 测试策略

### 单元测试
- AI算法测试
- 模型预测测试
- 数据处理测试

### 集成测试
- 端到端预测流程测试
- 套利检测测试
- 模型管理测试

### 性能测试
- AI推理性能测试
- 大数据量预测测试
- 模型加载性能测试