# 分析服务 (analytics) - 架构设计

## 📋 服务概述

### 服务名称
分析服务 (Analytics Service)

### 服务端口
8087

### 服务职责
- 性能分析 (策略、投资组合)
- 风险分析 (多维度风险评估)
- 统计报表 (各类分析报表)
- 数据导出 (Excel、PDF、CSV)
- 实时监控面板

## 🏗️ 服务架构

### 内部架构图
```
services/analytics/
│
├── src/
│   │
│   ├── main.rs                 # 服务入口，启动HTTP服务器
│   │
│   ├── state.rs                # 应用状态管理，持有所有组件的Arc引用
│   │
│   ├── config/                 # 配置管理
│   │   ├── mod.rs              # 配置结构体定义
│   │   └── settings.rs         # 配置加载逻辑
│   │
│   ├── handlers/               # HTTP接口层
│   │   ├── mod.rs              # 路由注册
│   │   ├── performance.rs      # 性能分析接口
│   │   ├── risk_analysis.rs    # 风险分析接口
│   │   ├── correlation.rs      # 相关性分析接口
│   │   ├── portfolio.rs        # 投资组合分析接口
│   │   ├── reports.rs          # 报告生成接口
│   │   ├── statistics.rs       # 统计接口
│   │   └── export.rs           # 数据导出接口
│   │
│   ├── analysis/               # 分析引擎
│   │   ├── mod.rs              # 分析引擎管理
│   │   ├── performance.rs      # 性能分析引擎
│   │   ├── risk.rs             # 风险分析引擎
│   │   ├── correlation.rs      # 相关性分析引擎
│   │   ├── attribution.rs      # 归因分析引擎
│   │   └── portfolio.rs        # 投资组合分析引擎
│   │
│   ├── calculators/            # 计算器模块
│   │   ├── mod.rs              # 计算器管理
│   │   ├── sharpe_calculator.rs # 夏普比率计算
│   │   ├── var_calculator.rs   # VaR计算
│   │   ├── drawdown_calculator.rs # 回撤计算
│   │   ├── volatility_calculator.rs # 波动率计算
│   │   └── alpha_beta_calculator.rs # Alpha/Beta计算
│   │
│   ├── exporters/              # 导出模块
│   │   ├── mod.rs              # 导出管理
│   │   ├── excel_exporter.rs   # Excel导出
│   │   ├── pdf_exporter.rs     # PDF导出
│   │   ├── csv_exporter.rs     # CSV导出
│   │   └── chart_exporter.rs   # 图表导出
│   │
│   ├── storage/                # 数据存储层
│   │   ├── mod.rs              # 存储接口
│   │   ├── postgres_store.rs   # PostgreSQL存储
│   │   ├── clickhouse_store.rs # ClickHouse存储
│   │   └── redis_cache.rs      # Redis缓存
│   │
│   ├── models/                 # 数据模型
│   │   ├── mod.rs              # 模型定义
│   │   ├── performance_report.rs # 性能报告模型
│   │   ├── risk_report.rs      # 风险报告模型
│   │   ├── correlation_result.rs # 相关性结果模型
│   │   ├── portfolio_report.rs # 投资组合报告模型
│   │   └── statistics.rs       # 统计数据模型
│   │
│   ├── services/               # 业务逻辑层
│   │   ├── mod.rs              # 服务管理
│   │   ├── performance_service.rs # 性能分析服务
│   │   ├── risk_analysis_service.rs # 风险分析服务
│   │   ├── report_service.rs      # 报告生成服务
│   │   ├── export_service.rs      # 导出服务
│   │   └── statistics_service.rs  # 统计服务
│   │
│   └── utils/                  # 工具函数
│       ├── mod.rs
│       ├── data_aggregator.rs  # 数据聚合工具
│       ├── chart_generator.rs  # 图表生成工具
│       └── report_formatter.rs # 报告格式化工具
│
└── Cargo.toml                  # 依赖声明
```

## 🔄 数据流向

### 性能分析流程
```
数据请求 (性能分析)
    ↓
handlers/performance.rs
    ↓
services/performance_service.rs
    ↓
analysis/performance.rs (性能计算)
    ↓
calculators/ (各种指标计算)
    ↓
storage/ (数据获取)
    ↓
返回性能分析结果
```

### 报告生成流程
```
报告生成请求
    ↓
handlers/reports.rs
    ↓
services/report_service.rs
    ↓
analysis/ (多维度分析)
    ↓
exporters/ (格式化输出)
    ↓
返回报告文件
```

## 📡 API接口设计

### 性能分析
```http
GET  /api/v1/analytics/performance # 获取性能分析
GET  /api/v1/analytics/performance/strategy/{id} # 策略性能
GET  /api/v1/analytics/performance/portfolio/{id} # 投资组合性能
GET  /api/v1/analytics/performance/user/{id} # 用户整体性能
GET  /api/v1/analytics/performance/chart # 性能图表数据
POST /api/v1/analytics/performance/calculate # 计算性能指标
```

### 风险分析
```http
GET  /api/v1/analytics/risk # 获取风险分析
GET  /api/v1/analytics/risk/var # VaR分析
GET  /api/v1/analytics/risk/correlation # 相关性分析
GET  /api/v1/analytics/risk/concentration # 集中度分析
GET  /api/v1/analytics/risk/heatmap # 风险热力图
POST /api/v1/analytics/risk/analyze # 风险分析
```

### 相关性分析
```http
GET  /api/v1/analytics/correlation # 获取相关性分析
GET  /api/v1/analytics/correlation/matrix # 相关性矩阵
GET  /api/v1/analytics/correlation/pair # 对相关性
GET  /api/v1/analytics/correlation/time-series # 时变相关性
POST /api/v1/analytics/correlation/calculate # 计算相关性
```

### 投资组合分析
```http
GET  /api/v1/analytics/portfolio/overview # 投资组合概览
GET  /api/v1/analytics/portfolio/allocation # 资产配置
GET  /api/v1/analytics/portfolio/performance # 投资组合绩效
GET  /api/v1/analytics/portfolio/risk # 投资组合风险
GET  /api/v1/analytics/portfolio/equity # 权益曲线
POST /api/v1/analytics/portfolio/analyze # 投资组合分析
```

### 统计报表
```http
GET  /api/v1/analytics/statistics/trading # 交易统计
GET  /api/v1/analytics/statistics/market # 市场统计
GET  /api/v1/analytics/statistics/user # 用户统计
GET  /api/v1/analytics/statistics/strategy # 策略统计
GET  /api/v1/analytics/statistics/time-series # 时间序列统计
```

### 数据导出
```http
POST /api/v1/export/excel # 导出Excel
POST /api/v1/export/pdf # 导出PDF
POST /api/v1/export/csv # 导出CSV
POST /api/v1/export/chart # 导出图表
GET  /api/v1/export/status/{id} # 查询导出状态
GET  /api/v1/export/download/{id} # 下载导出文件
```

### 自定义报告
```http
POST /api/v1/reports/custom # 生成自定义报告
GET  /api/v1/reports/daily # 获取日报
GET  /api/v1/reports/weekly # 获取周报
GET  /api/v1/reports/monthly # 获取月报
GET  /api/v1/reports/quarterly # 获取季报
GET  /api/v1/reports/yearly # 获取年报
GET  /api/v1/reports/history # 获取报告历史
```

## 🗄️ 数据模型

### 核心数据结构
```rust
// 性能报告模型
pub struct PerformanceReport {
    pub id: String,
    pub user_id: Option<String>,
    pub strategy_id: Option<String>,
    pub portfolio_id: Option<String>,
    pub period: TimePeriod,
    pub start_date: i64,
    pub end_date: i64,
    pub initial_capital: Decimal,
    pub final_capital: Decimal,
    pub total_return: Decimal,
    pub annual_return: Decimal,
    pub volatility: Decimal,
    pub sharpe_ratio: Decimal,
    pub sortino_ratio: Decimal,
    pub alpha: Decimal,
    pub beta: Decimal,
    pub max_drawdown: Decimal,
    pub calmar_ratio: Decimal,
    pub win_rate: Decimal,
    pub profit_factor: Decimal,
    pub total_trades: u64,
    pub avg_win: Decimal,
    pub avg_loss: Decimal,
    pub max_win: Decimal,
    pub max_loss: Decimal,
    pub r_squared: Decimal,
    pub information_ratio: Decimal,
    pub treynor_ratio: Decimal,
    pub ulcer_index: Decimal,
    pub equity_curve: Vec<EquityPoint>,
    pub trade_log: Vec<TradeRecord>,
    pub created_at: i64,
}

pub enum TimePeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom { start: i64, end: i64 },
}

pub struct EquityPoint {
    pub timestamp: i64,
    pub equity: Decimal,
    pub drawdown: Decimal,
}

pub struct TradeRecord {
    pub id: String,
    pub symbol: String,
    pub side: OrderSide,
    pub entry_time: i64,
    pub exit_time: i64,
    pub entry_price: Decimal,
    pub exit_price: Decimal,
    pub quantity: Decimal,
    pub profit: Decimal,
    pub return_rate: Decimal,
}

// 风险报告模型
pub struct RiskReport {
    pub id: String,
    pub user_id: Option<String>,
    pub strategy_id: Option<String>,
    pub var_95: Decimal,
    pub var_99: Decimal,
    pub expected_shortfall: Decimal,
    pub volatility: Decimal,
    pub max_drawdown: Decimal,
    pub value_at_risk_details: VarDetails,
    pub stress_test_results: Vec<StressTestResult>,
    pub scenario_analysis: Vec<ScenarioResult>,
    pub concentration_risk: ConcentrationRisk,
    pub liquidity_risk: LiquidityRisk,
    pub correlation_risk: CorrelationRisk,
    pub created_at: i64,
}

pub struct VarDetails {
    pub method: VarMethod,
    pub confidence_95: Decimal,
    pub confidence_99: Decimal,
    pub time_horizon: u32,
    pub historical_data_points: u32,
}

pub struct StressTestResult {
    pub scenario: String,
    pub loss_percentage: Decimal,
    pub probability: Decimal,
    pub impact: StressImpact,
}

pub enum StressImpact {
    Mild,     // 轻微
    Moderate, // 中等
    Severe,   // 严重
    Critical, // 危险
}

// 相关性分析结果模型
pub struct CorrelationResult {
    pub id: String,
    pub symbols: Vec<String>,
    pub correlation_matrix: Vec<Vec<f64>>,
    pub correlation_pairs: Vec<CorrelationPair>,
    pub cointegration_results: Option<CointegrationResult>,
    pub granger_causality: Vec<GrangerCausality>,
    pub rolling_correlation: Vec<RollingCorrelation>,
    pub created_at: i64,
}

pub struct CorrelationPair {
    pub symbol1: String,
    pub symbol2: String,
    pub pearson_correlation: f64,
    pub spearman_correlation: f64,
    pub kendall_correlation: f64,
    pub p_value: f64,
    pub confidence: f64,
}

// 投资组合报告模型
pub struct PortfolioReport {
    pub id: String,
    pub user_id: String,
    pub portfolio_id: String,
    pub allocation: Vec<AssetAllocation>,
    pub performance: PerformanceReport,
    pub risk_metrics: RiskReport,
    pub diversification_score: Decimal,
    pub efficient_frontier: Vec<PortfolioPoint>,
    pub tracking_error: Decimal,
    pub information_ratio: Decimal,
    pub benchmark_comparison: BenchmarkComparison,
    pub created_at: i64,
}

pub struct AssetAllocation {
    pub symbol: String,
    pub weight: Decimal,
    pub current_value: Decimal,
    pub purchase_value: Decimal,
    pub profit_loss: Decimal,
    pub profit_loss_percentage: Decimal,
}

pub struct PortfolioPoint {
    pub risk: Decimal,
    pub return_rate: Decimal,
    pub allocation: Vec<AssetAllocation>,
}

pub struct BenchmarkComparison {
    pub benchmark_symbol: String,
    pub portfolio_return: Decimal,
    pub benchmark_return: Decimal,
    pub excess_return: Decimal,
    pub tracking_error: Decimal,
    pub information_ratio: Decimal,
}
```

## 🔧 技术实现要点

### 分析引擎
- **高性能计算**: 优化的数学计算算法
- **多维度分析**: 支持多种分析维度
- **实时计算**: 毫秒级分析结果
- **批量处理**: 支持大量数据批量分析

### 计算器模块
- **统计指标**: 夏普比率、波动率、回撤等
- **风险指标**: VaR、期望短缺、Beta系数等
- **归因分析**: 收益归因分析
- **相关性分析**: 多种相关性算法

### 数据导出
- **Excel导出**: 复杂Excel报表生成
- **PDF报告**: 专业PDF报告生成
- **CSV导出**: 原始数据CSV格式
- **图表导出**: 高质量图表图片

### 性能优化
- **缓存策略**: Redis缓存分析结果
- **异步处理**: 异步报告生成
- **分页查询**: 大数据量分页处理
- **数据库优化**: ClickHouse时序数据优化

## 📊 监控指标

### 性能指标
- 分析计算延迟
- 报告生成时间
- 数据查询耗时
- 内存使用率

### 业务指标
- 报告生成成功率
- 导出任务完成率
- 用户分析使用率
- 报告下载量

## 🔐 安全措施

- **认证授权**: JWT认证 + RBAC权限控制
- **数据安全**: 敏感数据访问控制
- **导出限制**: 防止大量数据导出
- **审计日志**: 完整的分析操作审计日志

## 🚀 部署配置

### 环境变量
```
ANALYTICS_PORT=8087
DATABASE_URL=postgresql://user:pass@localhost/analytics
CLICKHOUSE_URL=http://clickhouse:8123
REDIS_URL=redis://localhost:6379
MARKET_DATA_URL=http://market-data:8083
TRADING_ENGINE_URL=http://trading-engine:8082
STRATEGY_ENGINE_URL=http://strategy-engine:8084
ANALYTICS_CACHE_TTL=3600
MAX_EXPORT_DATA_POINTS=100000
REPORT_GENERATION_TIMEOUT=300
CHART_WIDTH=1200
CHART_HEIGHT=800
EXCEL_MAX_ROWS=100000
PDF_DPI=300
```

### Docker配置
- 多阶段构建
- 资源限制
- 健康检查

## 🧪 测试策略

### 单元测试
- 分析算法测试
- 指标计算测试
- 数据模型测试

### 集成测试
- 端到端分析流程测试
- 报告生成测试
- 数据导出测试

### 性能测试
- 大数据量分析测试
- 高并发报告生成测试
- 系统稳定性测试