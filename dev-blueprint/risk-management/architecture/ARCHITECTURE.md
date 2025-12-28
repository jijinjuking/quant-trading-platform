# 风险管理服务 (risk-management) - 架构设计

## 📋 服务概述

### 服务名称
风险管理服务 (Risk Management Service)

### 服务端口
8085

### 服务职责
- 实时风险监控
- 风险评估与计算
- 风险预警系统
- 风险限额管理
- VaR计算

## 🏗️ 服务架构

### 内部架构图
```
services/risk-management/
│
├── src/
│   │
│   ├── main.rs                 # 服务入口，启动HTTP服务器和监控器
│   │
│   ├── state.rs                # 应用状态管理，持有所有组件的Arc引用
│   │
│   ├── config/                 # 配置管理
│   │   ├── mod.rs              # 配置结构体定义
│   │   └── settings.rs         # 配置加载逻辑
│   │
│   ├── handlers/               # HTTP接口层
│   │   ├── mod.rs              # 路由注册
│   │   ├── risk_assessment.rs  # 风险评估接口
│   │   ├── risk_limits.rs      # 风险限额接口
│   │   ├── risk_warnings.rs    # 风险预警接口
│   │   ├── var_calculator.rs   # VaR计算接口
│   │   └── risk_monitoring.rs  # 风险监控接口
│   │
│   ├── risk_engine/            # 风险引擎
│   │   ├── mod.rs              # 风险引擎管理
│   │   ├── market_risk.rs      # 市场风险计算
│   │   ├── credit_risk.rs      # 信用风险计算
│   │   ├── liquidity_risk.rs   # 流动性风险计算
│   │   └── operational_risk.rs # 操作风险计算
│   │
│   ├── calculators/            # 计算器模块
│   │   ├── mod.rs              # 计算器管理
│   │   ├── var_calculator.rs   # VaR计算器
│   │   ├── stress_test.rs      # 压力测试
│   │   ├── scenario_analyzer.rs # 情景分析
│   │   └── correlation_analyzer.rs # 相关性分析
│   │
│   ├── monitors/               # 监控模块
│   │   ├── mod.rs              # 监控管理
│   │   ├── position_monitor.rs # 持仓监控
│   │   ├── margin_monitor.rs   # 保证金监控
│   │   ├── exposure_monitor.rs # 暴露度监控
│   │   └── threshold_monitor.rs # 阈值监控
│   │
│   ├── storage/                # 数据存储层
│   │   ├── mod.rs              # 存储接口
│   │   ├── postgres_store.rs   # PostgreSQL存储
│   │   └── redis_cache.rs      # Redis缓存
│   │
│   ├── models/                 # 数据模型
│   │   ├── mod.rs              # 模型定义
│   │   ├── risk_assessment.rs  # 风险评估模型
│   │   ├── risk_limit.rs       # 风险限额模型
│   │   ├── risk_warning.rs     # 风险预警模型
│   │   ├── var_result.rs       # VaR结果模型
│   │   └── risk_metrics.rs     # 风险指标模型
│   │
│   ├── services/               # 业务逻辑层
│   │   ├── mod.rs              # 服务管理
│   │   ├── risk_assessment_service.rs # 风险评估服务
│   │   ├── risk_limit_service.rs      # 风险限额服务
│   │   ├── risk_monitor_service.rs    # 风险监控服务
│   │   └── risk_calculation_service.rs # 风险计算服务
│   │
│   └── utils/                  # 工具函数
│       ├── mod.rs
│       ├── risk_calculator.rs  # 风险计算工具
│       ├── statistics.rs       # 统计工具
│       └── alert_notifier.rs   # 预警通知工具
│
└── Cargo.toml                  # 依赖声明
```

## 🔄 数据流向

### 风险评估流程
```
市场数据 (来自market-data)
    ↓
risk_engine/market_risk.rs (市场风险计算)
    ↓
calculators/var_calculator.rs (VaR计算)
    ↓
monitors/position_monitor.rs (持仓监控)
    ↓
services/risk_assessment_service.rs (风险评估)
    ↓
storage/postgres_store.rs (存储评估结果)
    ↓
触发预警 (如需要)
```

### 风险预警流程
```
实时监控数据
    ↓
monitors/ (各种监控器)
    ↓
threshold_monitor.rs (阈值检查)
    ↓
services/risk_monitor_service.rs (风险判断)
    ↓
utils/alert_notifier.rs (预警通知)
    ↓
通知服务 (notification service)
```

## 📡 API接口设计

### 风险评估
```http
POST /api/v1/risk/assess        # 执行风险评估
GET  /api/v1/risk/assessments   # 查询风险评估历史
GET  /api/v1/risk/assessments/{id} # 查询评估详情
POST /api/v1/risk/assessments/batch # 批量风险评估
```

### 风险限额
```http
GET  /api/v1/risk/limits        # 查询风险限额
PUT  /api/v1/risk/limits        # 更新风险限额
POST /api/v1/risk/limits        # 创建风险限额
GET  /api/v1/risk/limits/{id}   # 查询限额详情
DELETE /api/v1/risk/limits/{id} # 删除风险限额
```

### 风险预警
```http
GET  /api/v1/risk/warnings      # 查询风险预警
POST /api/v1/risk/warnings/{id}/ack # 确认预警
GET  /api/v1/risk/warnings/types # 查询预警类型
GET  /api/v1/risk/warnings/history # 查询预警历史
```

### VaR计算
```http
POST /api/v1/risk/var           # 计算VaR
GET  /api/v1/risk/var/history   # 查询VaR历史
POST /api/v1/risk/var/stress    # 压力测试
POST /api/v1/risk/var/scenario  # 情景分析
```

### 风险监控
```http
GET  /api/v1/risk/realtime      # 获取实时风险数据
GET  /api/v1/risk/positions     # 查询持仓风险
GET  /api/v1/risk/exposure      # 查询风险暴露
GET  /api/v1/risk/metrics       # 获取风险指标
GET  /api/v1/risk/concentration # 查询集中度风险
```

## 🗄️ 数据模型

### 核心数据结构
```rust
// 风险评估模型
pub struct RiskAssessment {
    pub id: String,
    pub user_id: String,
    pub symbol: Option<String>,
    pub risk_types: Vec<RiskType>,
    pub risk_score: Decimal,        // 0-1风险评分
    pub confidence: Decimal,        // 评估置信度
    pub recommendations: Vec<String>,
    pub timestamp: i64,
    pub assessment_details: RiskAssessmentDetails,
}

pub enum RiskType {
    MarketRisk,         // 市场风险
    LiquidityRisk,      // 流动性风险
    CreditRisk,         // 信用风险
    OperationalRisk,    // 操作风险
    ConcentrationRisk,  // 集中度风险
    VolatilityRisk,     // 波动率风险
}

pub struct RiskAssessmentDetails {
    pub market_risk: MarketRiskDetails,
    pub liquidity_risk: LiquidityRiskDetails,
    pub concentration_risk: ConcentrationRiskDetails,
    pub volatility_risk: VolatilityRiskDetails,
}

// 风险限额模型
pub struct RiskLimit {
    pub id: String,
    pub user_id: Option<String>,
    pub group_id: Option<String>,  // 用户组限额
    pub limit_type: LimitType,
    pub symbol: Option<String>,
    pub exchange: Option<String>,
    pub max_value: Decimal,
    pub current_value: Decimal,
    pub utilization_rate: Decimal,
    pub status: LimitStatus,
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum LimitType {
    MaxPosition,        // 最大持仓
    MaxDrawdown,        // 最大回撤
    MaxDailyLoss,       // 最大日亏损
    MaxLeverage,        // 最大杠杆
    MaxOrderSize,       // 最大单笔订单
    MaxExposure,        // 最大风险暴露
    MaxConcentration,   // 最大集中度
    MaxVaR,             // 最大VaR
}

pub enum LimitStatus {
    Active,
    Breached,           // 已突破
    Warning,            // 警告
    Suspended,          // 已暂停
}

// 风险预警模型
pub struct RiskWarning {
    pub id: String,
    pub user_id: Option<String>,
    pub risk_type: RiskType,
    pub level: WarningLevel,      // LOW/MEDIUM/HIGH/CRITICAL
    pub message: String,
    pub details: serde_json::Value,
    pub status: WarningStatus,    // ACTIVE/ACKNOWLEDGED/RESOLVED
    pub created_at: i64,
    pub acknowledged_at: Option<i64>,
    pub resolved_at: Option<i64>,
    pub acknowledged_by: Option<String>,
}

pub enum WarningLevel {
    Low,      // 低风险
    Medium,   // 中风险
    High,     // 高风险
    Critical, // 严重风险
}

pub enum WarningStatus {
    Active,        // 活跃
    Acknowledged,  // 已确认
    Resolved,      // 已解决
}

// VaR计算结果模型
pub struct VarResult {
    pub id: String,
    pub user_id: Option<String>,
    pub symbol: Option<String>,
    pub calculation_method: VarMethod,
    pub confidence_level: f64,    // 置信水平 (0.95, 0.99)
    pub time_horizon: u32,        // 持有期 (天)
    pub var_value: Decimal,       // VaR值
    pub expected_shortfall: Decimal, // 期望短缺
    pub parameters: VarParameters,
    pub calculation_date: i64,
    pub backtesting_results: Option<BacktestingResults>,
}

pub enum VarMethod {
    Historical,      // 历史模拟法
    Parametric,      // 参数法
    MonteCarlo,      // 蒙特卡洛法
}

pub struct VarParameters {
    pub window_size: u32,         // 历史窗口大小
    pub bootstrap_samples: u32,   // 自举样本数
    pub volatility_adjustment: bool, // 波动率调整
}

// 风险指标模型
pub struct RiskMetrics {
    pub user_id: String,
    pub symbol: Option<String>,
    pub var_95: Decimal,          // 95% VaR
    pub var_99: Decimal,          // 99% VaR
    pub expected_shortfall: Decimal, // 期望短缺
    pub volatility: Decimal,      // 波动率
    pub beta: Option<Decimal>,    // Beta系数
    pub sharpe_ratio: Option<Decimal>, // 夏普比率
    pub sortino_ratio: Option<Decimal>, // 索提诺比率
    pub maximum_drawdown: Decimal, // 最大回撤
    pub alpha: Option<Decimal>,   // Alpha系数
    pub r_squared: Option<Decimal>, // R平方
    pub updated_at: i64,
}
```

## 🔧 技术实现要点

### 风险计算
- **实时计算**: 毫秒级风险指标计算
- **多种方法**: 支持多种VaR计算方法
- **压力测试**: 支持压力测试和情景分析
- **统计模型**: 先进的统计风险模型

### 监控系统
- **多维度监控**: 持仓、保证金、暴露度等
- **阈值预警**: 可配置的预警阈值
- **实时推送**: 实时风险数据推送
- **批量处理**: 支持批量风险计算

### 性能优化
- **缓存策略**: Redis缓存计算结果
- **异步计算**: 异步风险计算任务
- **批量处理**: 批量数据处理
- **数据库优化**: 索引和查询优化

## 📊 监控指标

### 性能指标
- 风险计算延迟
- 预警响应时间
- 数据处理吞吐量
- 内存使用率

### 业务指标
- 预警准确率
- 风险覆盖率
- 限额使用率
- 风险事件统计

## 🔐 安全措施

- **认证授权**: JWT认证 + RBAC权限控制
- **数据安全**: 敏感风险数据加密
- **访问控制**: 严格的权限控制
- **审计日志**: 完整的风险操作审计日志

## 🚀 部署配置

### 环境变量
```
RISK_MANAGEMENT_PORT=8085
DATABASE_URL=postgresql://user:pass@localhost/risk
REDIS_URL=redis://localhost:6379
MARKET_DATA_URL=http://market-data:8083
NOTIFICATION_URL=http://notification:8086
VAR_CALCULATION_INTERVAL=60
RISK_MONITORING_INTERVAL=10
HIGH_RISK_THRESHOLD=0.8
CRITICAL_RISK_THRESHOLD=0.95
MAX_CALCULATION_THREADS=10
```

### Docker配置
- 多阶段构建
- 资源限制
- 健康检查

## 🧪 测试策略

### 单元测试
- 风险计算算法测试
- 预警逻辑测试
- 数据模型测试

### 集成测试
- 端到端风险评估测试
- 预警通知测试
- 限额控制测试

### 压力测试
- 大量并发风险计算测试
- 高频数据更新测试
- 系统稳定性测试