# API网关 (gateway) - 架构设计

## 📋 服务概述

### 服务名称
API网关 (API Gateway Service)

### 服务端口
8080

### 服务职责
- 统一入口管理
- 请求路由和负载均衡
- 认证和授权
- 限流和熔断
- 日志和监控
- 协议转换

## 🏗️ 服务架构

### 内部架构图
```
services/gateway/
│
├── src/
│   │
│   ├── main.rs                 # 服务入口，启动HTTP/HTTPS服务器
│   │
│   ├── state.rs                # 应用状态管理，持有所有组件的Arc引用
│   │
│   ├── config/                 # 配置管理
│   │   ├── mod.rs              # 配置结构体定义
│   │   └── settings.rs         # 配置加载逻辑
│   │
│   ├── routes/                 # 路由管理
│   │   ├── mod.rs              # 路由注册
│   │   ├── user_management.rs  # 用户管理路由
│   │   ├── trading_engine.rs   # 交易引擎路由
│   │   ├── market_data.rs      # 市场数据路由
│   │   ├── strategy_engine.rs  # 策略引擎路由
│   │   ├── risk_management.rs  # 风险管理路由
│   │   ├── notification.rs     # 通知路由
│   │   ├── analytics.rs        # 分析路由
│   │   ├── ai_service.rs       # AI服务路由
│   │   └── admin_backend.rs    # 管理后台路由
│   │
│   ├── middleware/             # 中间件
│   │   ├── mod.rs              # 中间件管理
│   │   ├── auth.rs             # 认证中间件
│   │   ├── cors.rs             # CORS中间件
│   │   ├── rate_limiter.rs     # 限流中间件
│   │   ├── logger.rs           # 日志中间件
│   │   ├── cors.rs             # 跨域中间件
│   │   ├── request_id.rs       # 请求ID中间件
│   │   ├── error_handler.rs    # 错误处理中间件
│   │   └── circuit_breaker.rs  # 熔断器中间件
│   │
│   ├── proxy/                  # 代理模块
│   │   ├── mod.rs              # 代理管理
│   │   ├── http_proxy.rs       # HTTP代理
│   │   ├── load_balancer.rs    # 负载均衡器
│   │   ├── health_checker.rs   # 健康检查器
│   │   └── service_discovery.rs # 服务发现
│   │
│   ├── auth/                   # 认证模块
│   │   ├── mod.rs              # 认证管理
│   │   ├── jwt_validator.rs    # JWT验证器
│   │   ├── api_key_validator.rs # API密钥验证器
│   │   └── rbac_checker.rs     # RBAC权限检查器
│   │
│   ├── storage/                # 数据存储层
│   │   ├── mod.rs              # 存储接口
│   │   └── redis_store.rs      # Redis存储 (限流、缓存)
│   │
│   ├── models/                 # 数据模型
│   │   ├── mod.rs              # 模型定义
│   │   ├── route.rs            # 路由模型
│   │   ├── service.rs          # 服务模型
│   │   ├── rate_limit.rs       # 限流模型
│   │   └── circuit_breaker.rs  # 熔断器模型
│   │
│   ├── services/               # 业务逻辑层
│   │   ├── mod.rs              # 服务管理
│   │   ├── route_service.rs    # 路由服务
│   │   ├── rate_limit_service.rs # 限流服务
│   │   ├── health_service.rs   # 健康检查服务
│   │   └── monitoring_service.rs # 监控服务
│   │
│   └── utils/                  # 工具函数
│       ├── mod.rs
│       ├── config_loader.rs    # 配置加载工具
│       ├── metrics_collector.rs # 指标收集工具
│       └── response_builder.rs # 响应构建工具
│
└── Cargo.toml                  # 依赖声明
```

## 🔄 数据流向

### 请求处理流程
```
HTTP请求
    ↓
middleware/request_id.rs (生成请求ID)
    ↓
middleware/logger.rs (记录请求日志)
    ↓
middleware/auth.rs (认证授权)
    ↓
routes/ (路由匹配)
    ↓
middleware/rate_limiter.rs (限流检查)
    ↓
proxy/http_proxy.rs (代理到后端服务)
    ↓
middleware/circuit_breaker.rs (熔断器)
    ↓
返回响应
    ↓
middleware/logger.rs (记录响应日志)
```

### 服务发现流程
```
健康检查请求
    ↓
proxy/health_checker.rs
    ↓
services/health_service.rs
    ↓
检查后端服务健康状态
    ↓
更新路由表
    ↓
负载均衡选择
```

## 📡 API接口设计

### 网关管理接口
```http
GET  /health                   # 网关健康检查
GET  /status                   # 网关状态
GET  /metrics                  # 监控指标
GET  /routes                   # 路由列表
GET  /services                 # 后端服务列表
GET  /rate-limits              # 限流配置
POST /rate-limits              # 更新限流配置
GET  /circuit-breakers         # 熔断器状态
POST /circuit-breakers/reset   # 重置熔断器
GET  /config                   # 当前配置
POST /config/reload            # 重载配置
```

### 代理接口 (透明代理到后端服务)
```
# 用户管理服务代理
/api/v1/auth/*                 → user-management:8081
/api/v1/users/*                → user-management:8081
/api/v1/permissions/*          → user-management:8081

# 交易引擎服务代理
/api/v1/orders/*               → trading-engine:8082
/api/v1/positions/*            → trading-engine:8082
/api/v1/balances/*             → trading-engine:8082
/api/v1/trades/*               → trading-engine:8082

# 市场数据服务代理
/api/v1/market/*               → market-data:8083
/api/v1/klines/*               → market-data:8083
/api/v1/tickers/*              → market-data:8083

# 策略引擎服务代理
/api/v1/strategies/*           → strategy-engine:8084
/api/v1/signals/*              → strategy-engine:8084
/api/v1/backtests/*            → strategy-engine:8084

# 风险管理服务代理
/api/v1/risk/*                 → risk-management:8085
/api/v1/risk/limits/*          → risk-management:8085

# 通知服务代理
/api/v1/notifications/*        → notification:8086
/api/v1/templates/*            → notification:8086

# 分析服务代理
/api/v1/analytics/*            → analytics:8087
/api/v1/reports/*              → analytics:8087

# AI服务代理
/api/v1/predict/*              → ai-service:8088
/api/v1/arbitrage/*            → ai-service:8088
/api/v1/signals/*              → ai-service:8088
```

## 🗄️ 数据模型

### 核心数据结构
```rust
// 路由模型
pub struct Route {
    pub id: String,
    pub path: String,                    // 路径模式
    pub method: HttpMethod,              // HTTP方法
    pub upstream_service: String,        // 后端服务名
    pub upstream_url: String,            // 后端服务URL
    pub priority: u32,                   // 优先级
    pub enabled: bool,                   // 是否启用
    pub rate_limit: Option<RateLimit>,   // 限流配置
    pub auth_required: bool,             // 是否需要认证
    pub roles_required: Vec<String>,     // 需要的角色
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    HEAD,
    OPTIONS,
}

// 服务模型
pub struct BackendService {
    pub id: String,
    pub name: String,
    pub url: String,                     // 服务URL
    pub health_check_url: String,        // 健康检查URL
    pub status: ServiceStatus,           // 服务状态
    pub weight: u32,                     // 负载均衡权重
    pub max_connections: u32,           // 最大连接数
    pub timeout: u64,                   // 超时时间(毫秒)
    pub retries: u32,                   // 重试次数
    pub circuit_breaker: CircuitBreaker, // 熔断器配置
    pub rate_limit: Option<RateLimit>,   // 服务级限流
    pub last_heartbeat: Option<i64>,    // 最后心跳时间
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum ServiceStatus {
    Healthy,        // 健康
    Unhealthy,      // 不健康
    Unknown,        // 未知
    Maintenance,    // 维护中
}

// 限流模型
pub struct RateLimit {
    pub id: String,
    pub limit_type: RateLimitType,       // 限流类型
    pub limit: u32,                      // 限制数量
    pub window_size: u64,               // 时间窗口(秒)
    pub key: RateLimitKey,              // 限流键
    pub enabled: bool,                   // 是否启用
    pub strategy: RateLimitStrategy,     // 限流策略
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum RateLimitType {
    RequestPerSecond,     // 每秒请求数
    RequestPerMinute,     // 每分钟请求数
    RequestPerHour,       // 每小时请求数
    Concurrency,          // 并发数
    DataPerMinute,        // 每分钟数据量
}

pub enum RateLimitKey {
    Global,             // 全局限流
    IP,                 // IP限流
    User,               // 用户限流
    APIKey,             // API密钥限流
    Endpoint,           // 端点限流
    Custom(String),     // 自定义限流
}

pub enum RateLimitStrategy {
    TokenBucket,        // 令牌桶
    LeakyBucket,        // 漏桶
    FixedWindow,        // 固定窗口
    SlidingWindow,      // 滑动窗口
}

// 熔断器模型
pub struct CircuitBreaker {
    pub id: String,
    pub service_id: String,
    pub state: CircuitState,             // 熔断器状态
    pub failure_threshold: u32,         // 失败阈值
    pub success_threshold: u32,         // 成功阈值
    pub timeout: u64,                   // 熔断超时(毫秒)
    pub failure_rate_threshold: f64,    // 失败率阈值
    pub last_failure_time: Option<i64>, // 最后失败时间
    pub failure_count: u32,             // 失败计数
    pub success_count: u32,             // 成功计数
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum CircuitState {
    Closed,     // 关闭(正常)
    Open,       // 开启(熔断)
    HalfOpen,   // 半开(尝试恢复)
}

// 网关配置模型
pub struct GatewayConfig {
    pub server: ServerConfig,
    pub routes: Vec<Route>,
    pub services: Vec<BackendService>,
    pub authentication: AuthConfig,
    pub rate_limiting: RateLimitConfig,
    pub circuit_breakers: CircuitBreakerConfig,
    pub logging: LoggingConfig,
    pub cors: CorsConfig,
    pub monitoring: MonitoringConfig,
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub ssl_enabled: bool,
    pub ssl_cert_path: Option<String>,
    pub ssl_key_path: Option<String>,
    pub max_connections: u32,
    pub request_timeout: u64,           // 请求超时(毫秒)
    pub keep_alive_timeout: u64,        // 长连接超时(毫秒)
}
```

## 🔧 技术实现要点

### 路由管理
- **动态路由**: 支持运行时路由更新
- **路径匹配**: 支持通配符和正则匹配
- **负载均衡**: 支持多种负载均衡算法
- **健康检查**: 自动健康检查和故障转移

### 认证授权
- **JWT验证**: 支持JWT令牌验证
- **API密钥**: 支持API密钥认证
- **RBAC控制**: 基于角色的访问控制
- **多租户**: 支持多租户认证

### 限流熔断
- **多种算法**: 令牌桶、漏桶等算法
- **多维度限流**: IP、用户、API等维度
- **智能熔断**: 自适应熔断策略
- **实时调整**: 支持运行时限流配置调整

### 性能优化
- **连接池**: HTTP连接池管理
- **请求缓存**: 智能响应缓存
- **异步处理**: Tokio异步处理
- **批量处理**: 支持批量请求处理

## 📊 监控指标

### 性能指标
- 请求响应时间
- QPS (每秒查询率)
- 连接池使用率
- 内存使用率

### 业务指标
- 服务可用性
- 错误率
- 限流触发次数
- 熔断器状态

## 🔐 安全措施

- **传输加密**: HTTPS/TLS加密
- **认证授权**: JWT + RBAC权限控制
- **API限流**: 防止API滥用
- **熔断保护**: 防止级联故障
- **访问控制**: 细粒度访问控制
- **日志审计**: 完整的访问日志

## 🚀 部署配置

### 环境变量
```
GATEWAY_PORT=8080
GATEWAY_HOST=0.0.0.0
REDIS_URL=redis://localhost:6379
USER_MANAGEMENT_URL=http://user-management:8081
TRADING_ENGINE_URL=http://trading-engine:8082
MARKET_DATA_URL=http://market-data:8083
STRATEGY_ENGINE_URL=http://strategy-engine:8084
RISK_MANAGEMENT_URL=http://risk-management:8085
NOTIFICATION_URL=http://notification:8086
ANALYTICS_URL=http://analytics:8087
AI_SERVICE_URL=http://ai-service:8088
ADMIN_BACKEND_URL=http://admin-backend:8089
GATEWAY_REQUEST_TIMEOUT=30000
GATEWAY_MAX_CONNECTIONS=10000
RATE_LIMIT_ENABLED=true
CIRCUIT_BREAKER_ENABLED=true
JWT_SECRET_KEY=your_secret_key
LOG_LEVEL=info
HEALTH_CHECK_INTERVAL=30
```

### Docker配置
- 多阶段构建
- 资源限制
- 健康检查

## 🧪 测试策略

### 单元测试
- 路由匹配测试
- 认证逻辑测试
- 限流算法测试

### 集成测试
- 端到端请求代理测试
- 熔断器功能测试
- 负载均衡测试

### 压力测试
- 高并发请求测试
- 故障恢复测试
- 系统稳定性测试