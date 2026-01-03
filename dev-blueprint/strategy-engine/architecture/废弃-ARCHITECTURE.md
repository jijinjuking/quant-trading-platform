# 策略引擎服务 (strategy-engine) - 架构设计

## 📋 服务概述

### 服务名称
策略引擎服务 (Strategy Engine Service)

### 服务端口
8084

### 服务职责
- 策略管理 (创建、配置、启动/停止)
- 信号生成 (技术指标、交易信号)
- 回测系统 (历史数据回测)
- 策略执行 (自动交易)
- 性能监控 (收益、风险指标)

## 🏗️ 服务架构

### 内部架构图
```
services/strategy-engine/
│
├── src/
│   │
│   ├── main.rs                 # 服务入口，启动HTTP服务器和策略执行器
│   │
│   ├── state.rs                # 应用状态管理，持有所有组件的Arc引用
│   │
│   ├── config/                 # 配置管理
│   │   ├── mod.rs              # 配置结构体定义
│   │   └── settings.rs         # 配置加载逻辑
│   │
│   ├── handlers/               # HTTP接口层
│   │   ├── mod.rs              # 路由注册
│   │   ├── strategies.rs       # 策略管理接口
│   │   ├── signals.rs          # 信号管理接口
│   │   ├── indicators.rs       # 技术指标接口
│   │   ├── backtests.rs        # 回测管理接口
│   │   └── performance.rs      # 性能监控接口
│   │
│   ├── strategies/             # 策略实现
│   │   ├── mod.rs              # 策略管理器
│   │   ├── trend_following.rs  # 趋势跟踪策略
│   │   ├── mean_reversion.rs   # 均值回归策略
│   │   ├── arbitrage.rs        # 套利策略
│   │   ├── grid.rs             # 网格策略
│   │   └── custom.rs           # 自定义策略
│   │
│   ├── indicators/             # 技术指标
│   │   ├── mod.rs              # 指标管理器
│   │   ├── moving_average.rs   # 移动平均线
│   │   ├── rsi.rs              # RSI指标
│   │   ├── macd.rs             # MACD指标
│   │   ├── bollinger_bands.rs  # 布林带
│   │   └── custom_indicators.rs # 自定义指标
│   │
│   ├── backtest/               # 回测引擎
│   │   ├── mod.rs              # 回测管理器
│   │   ├── engine.rs           # 回测引擎
│   │   ├── simulator.rs        # 交易模拟器
│   │   └── reporter.rs         # 回测报告生成器
│   │
│   ├── storage/                # 数据存储层
│   │   ├── mod.rs              # 存储接口
│   │   ├── postgres_store.rs   # PostgreSQL存储
│   │   └── redis_cache.rs      # Redis缓存
│   │
│   ├── models/                 # 数据模型
│   │   ├── mod.rs              # 模型定义
│   │   ├── strategy.rs         # 策略模型
│   │   ├── signal.rs           # 信号模型
│   │   ├── indicator.rs        # 指标模型
│   │   ├── backtest.rs         # 回测模型
│   │   └── performance.rs      # 性能模型
│   │
│   ├── services/               # 业务逻辑层
│   │   ├── mod.rs              # 服务管理
│   │   ├── strategy_service.rs # 策略服务
│   │   ├── signal_service.rs   # 信号服务
│   │   ├── backtest_service.rs # 回测服务
│   │   └── performance_service.rs # 性能服务
│   │
│   └── utils/                  # 工具函数
│       ├── mod.rs
│       ├── data_loader.rs      # 数据加载工具
│       ├── risk_calculator.rs  # 风险计算工具
│       └── performance_calculator.rs # 性能计算工具
│
└── Cargo.toml                  # 依赖声明
```

## 🔄 数据流向

### 策略执行流程
```
市场数据 (来自market-data)
    ↓
indicators/ (计算技术指标)
    ↓
strategies/ (生成交易信号)
    ↓
services/signal_service.rs (信号处理)
    ↓
handlers/signals.rs (发送交易信号)
    ↓
(通过API发送到trading-engine)
```

### 回测流程
```
历史数据 (来自数据源)
    ↓
backtest/engine.rs (回测引擎)
    ↓
strategies/ (策略执行)
    ↓
backtest/simulator.rs (交易模拟)
    ↓
backtest/reporter.rs (生成报告)
    ↓
handlers/backtests.rs (返回结果)
```

## 📡 API接口设计

### 策略管理
```http
POST /api/v1/strategies         # 创建策略
GET  /api/v1/strategies         # 查询策略列表
GET  /api/v1/strategies/{id}    # 查询策略详情
PUT  /api/v1/strategies/{id}    # 更新策略
DELETE /api/v1/strategies/{id}  # 删除策略
POST /api/v1/strategies/{id}/start  # 启动策略
POST /api/v1/strategies/{id}/stop   # 停止策略
POST /api/v1/strategies/{id}/pause  # 暂停策略
POST /api/v1/strategies/{id}/resume # 恢复策略
```

### 信号管理
```http
GET  /api/v1/signals            # 查询交易信号
GET  /api/v1/signals/{id}       # 查询信号详情
POST /api/v1/signals/execute    # 执行信号
GET  /api/v1/signals/history    # 查询信号历史
```

### 技术指标
```http
GET  /api/v1/indicators         # 查询可用指标
GET  /api/v1/indicators/{name}  # 获取指标数据
POST /api/v1/indicators/calculate # 计算指标
```

### 回测管理
```http
POST /api/v1/backtests          # 创建回测任务
GET  /api/v1/backtests          # 查询回测列表
GET  /api/v1/backtests/{id}     # 查询回测结果
GET  /api/v1/backtests/{id}/report # 获取回测报告
```

### 性能监控
```http
GET  /api/v1/performance        # 查询性能指标
GET  /api/v1/performance/{id}   # 查询策略性能
GET  /api/v1/performance/equity # 查询权益曲线
GET  /api/v1/performance/risk   # 查询风险指标
```

## 🗄️ 数据模型

### 核心数据结构
```rust
// 策略模型
pub struct Strategy {
    pub id: String,
    pub name: String,
    pub strategy_type: StrategyType,
    pub status: StrategyStatus,     // ACTIVE/PAUSED/STOPPED
    pub symbols: Vec<String>,
    pub timeframes: Vec<TimeFrame>,
    pub parameters: HashMap<String, serde_json::Value>,
    pub risk_management: RiskManagement,
    pub performance: StrategyPerformance,
    pub created_at: i64,
    pub updated_at: i64,
}

// 策略类型枚举
pub enum StrategyType {
    MovingAverageCrossover,  // 均线交叉
    TrendFollowing,          // 趋势跟踪
    MeanReversion,          // 均值回归
    Arbitrage,              // 套利
    Grid,                   // 网格
    Custom(String),         // 自定义
}

// 信号模型
pub struct Signal {
    pub id: String,
    pub strategy_id: String,
    pub symbol: String,
    pub signal_type: SignalType,    // BUY/SELL/HOLD
    pub strength: f64,             // 信号强度
    pub confidence: f64,           // 信号置信度
    pub entry_price: Decimal,
    pub stop_loss: Option<Decimal>,
    pub take_profit: Option<Decimal>,
    pub timestamp: i64,
}

// 回测结果模型
pub struct BacktestResult {
    pub id: String,
    pub strategy_id: String,
    pub start_date: i64,
    pub end_date: i64,
    pub initial_capital: Decimal,
    pub final_capital: Decimal,
    pub total_return: Decimal,
    pub annual_return: Decimal,
    pub max_drawdown: Decimal,
    pub sharpe_ratio: Decimal,
    pub win_rate: Decimal,
    pub total_trades: u64,
    pub profit_factor: Decimal,
    pub equity_curve: Vec<EquityPoint>,
    pub trade_log: Vec<BacktestTrade>,
}

// 性能指标模型
pub struct StrategyPerformance {
    pub total_return: Decimal,
    pub annual_return: Decimal,
    pub volatility: Decimal,
    pub sharpe_ratio: Decimal,
    pub max_drawdown: Decimal,
    pub calmar_ratio: Decimal,
    pub win_rate: Decimal,
    pub profit_factor: Decimal,
    pub total_trades: u64,
    pub avg_win: Decimal,
    pub avg_loss: Decimal,
}
```

## 🔧 技术实现要点

### 策略执行
- **实时信号生成**: 基于实时数据生成交易信号
- **多时间框架**: 支持多时间框架策略
- **信号过滤**: 多重过滤机制确保信号质量
- **自动执行**: 支持信号自动转换为交易订单

### 技术指标
- **趋势指标**: MA、EMA、MACD、ADX、Parabolic SAR
- **震荡指标**: RSI、Stochastic、Williams %R、CCI
- **成交量指标**: OBV、VWAP、Volume Profile
- **波动率指标**: Bollinger Bands、ATR、Keltner Channel
- **自定义指标**: 支持用户自定义技术指标

### 回测系统
- **数据精度**: 高精度历史数据回测
- **交易成本**: 考虑手续费和滑点
- **资金管理**: 支持多种资金管理策略
- **绩效评估**: 完整的绩效指标计算

### 性能优化
- **异步处理**: Tokio异步运行时
- **缓存优化**: Redis缓存计算结果
- **批量计算**: 支持批量指标计算
- **内存管理**: 优化内存使用，避免泄漏

## 📊 监控指标

### 性能指标
- 策略执行延迟
- 指标计算耗时
- 回测完成时间
- 内存使用率

### 业务指标
- 信号生成频率
- 信号准确率
- 策略收益率
- 风险指标

## 🔐 安全措施

- **认证授权**: JWT认证 + RBAC权限控制
- **策略验证**: 严格的策略参数验证
- **风险控制**: 内置风险控制机制
- **审计日志**: 完整的策略执行审计日志

## 🚀 部署配置

### 环境变量
```
STRATEGY_ENGINE_PORT=8084
DATABASE_URL=postgresql://user:pass@localhost/strategy
REDIS_URL=redis://localhost:6379
MARKET_DATA_URL=http://market-data:8083
TRADING_ENGINE_URL=http://trading-engine:8082
BACKTEST_DATA_PATH=/data/historical
STRATEGY_TIMEOUT=300
MAX_CONCURRENT_STRATEGIES=100
```

### Docker配置
- 多阶段构建
- 资源限制
- 健康检查

## 🧪 测试策略

### 单元测试
- 策略算法测试
- 技术指标计算测试
- 数据模型测试

### 集成测试
- 端到端策略执行测试
- 回测系统测试
- 信号生成测试

### 性能测试
- 高并发策略执行测试
- 大量历史数据回测测试
- 系统稳定性测试