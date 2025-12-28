# 交易引擎服务 (trading-engine) - 架构设计

## 📋 服务概述

### 服务名称
交易引擎服务 (Trading Engine Service)

### 服务端口
8082

### 服务职责
- 订单管理 (创建、修改、取消)
- 账户管理 (余额、持仓)
- 交易执行 (订单匹配、成交)
- 风险控制 (保证金、强平)
- 交易所连接

## 🏗️ 服务架构

### 内部架构图
```
services/trading-engine/
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
│   │   ├── orders.rs           # 订单管理接口
│   │   ├── positions.rs        # 持仓管理接口
│   │   ├── balances.rs         # 余额管理接口
│   │   ├── trades.rs           # 交易记录接口
│   │   └── accounts.rs         # 账户管理接口
│   │
│   ├── exchanges/              # 交易所适配器
│   │   ├── mod.rs              # 交易所管理
│   │   ├── binance.rs          # Binance交易所适配器
│   │   ├── okx.rs              # OKX交易所适配器
│   │   ├── huobi.rs            # Huobi交易所适配器
│   │   └── exchange_trait.rs   # 交易所接口定义
│   │
│   ├── core/                   # 核心引擎
│   │   ├── mod.rs              # 核心模块管理
│   │   ├── order_matcher.rs    # 订单匹配引擎
│   │   ├── execution_engine.rs # 执行引擎
│   │   ├── risk_engine.rs      # 风控引擎
│   │   └── position_manager.rs # 持仓管理器
│   │
│   ├── storage/                # 数据存储层
│   │   ├── mod.rs              # 存储接口
│   │   ├── postgres_store.rs   # PostgreSQL存储
│   │   └── redis_cache.rs      # Redis缓存
│   │
│   ├── models/                 # 数据模型
│   │   ├── mod.rs              # 模型定义
│   │   ├── order.rs            # 订单模型
│   │   ├── position.rs         # 持仓模型
│   │   ├── balance.rs          # 余额模型
│   │   ├── trade.rs            # 成交模型
│   │   └── account.rs          # 账户模型
│   │
│   ├── services/               # 业务逻辑层
│   │   ├── mod.rs              # 服务管理
│   │   ├── order_service.rs    # 订单服务
│   │   ├── account_service.rs  # 账户服务
│   │   ├── position_service.rs # 持仓服务
│   │   └── execution_service.rs # 执行服务
│   │
│   └── utils/                  # 工具函数
│       ├── mod.rs
│       ├── validation.rs       # 参数验证
│       ├── risk_calculator.rs  # 风控计算
│       └── fee_calculator.rs   # 手续费计算
│
└── Cargo.toml                  # 依赖声明
```

## 🔄 数据流向

### 订单处理流程
```
HTTP请求 (创建订单)
    ↓
handlers/orders.rs
    ↓
services/order_service.rs
    ↓
core/order_matcher.rs (订单匹配)
    ↓
exchanges/ (发送到交易所)
    ↓
core/execution_engine.rs (执行处理)
    ↓
storage/postgres_store.rs (持久化)
    ↓
返回订单状态
```

### 账户更新流程
```
成交事件 (来自交易所)
    ↓
exchanges/
    ↓
core/execution_engine.rs
    ↓
core/position_manager.rs (更新持仓)
    ↓
storage/postgres_store.rs (更新账户)
    ↓
触发通知事件
```

## 📡 API接口设计

### 订单管理
```http
POST /api/v1/orders              # 创建订单
GET  /api/v1/orders              # 查询订单列表
GET  /api/v1/orders/{id}         # 查询单个订单
PUT  /api/v1/orders/{id}         # 修改订单
DELETE /api/v1/orders/{id}       # 取消订单
POST /api/v1/orders/batch        # 批量操作
```

### 账户管理
```http
GET  /api/v1/accounts/{id}       # 查询账户信息
GET  /api/v1/balances            # 查询余额
GET  /api/v1/positions           # 查询持仓
GET  /api/v1/trades              # 查询成交记录
POST /api/v1/accounts/transfer   # 资金划转
```

### 交易执行
```http
POST /api/v1/trade/preview       # 交易预览
POST /api/v1/trade/execute       # 执行交易
GET  /api/v1/trade/fees          # 查询手续费
```

## 🗄️ 数据模型

### 核心数据结构
```rust
// 订单模型
pub struct Order {
    pub id: String,
    pub user_id: String,
    pub symbol: String,
    pub side: OrderSide,          // BUY/SELL
    pub order_type: OrderType,    // MARKET/LIMIT/STOP
    pub quantity: Decimal,
    pub price: Option<Decimal>,
    pub status: OrderStatus,      // NEW/FILLED/CANCELED
    pub time_in_force: TimeInForce, // GTC/IOC/FOK
    pub created_at: i64,
    pub updated_at: i64,
}

// 持仓模型
pub struct Position {
    pub id: String,
    pub user_id: String,
    pub symbol: String,
    pub side: PositionSide,       // LONG/SHORT
    pub quantity: Decimal,
    pub entry_price: Decimal,
    pub unrealized_pnl: Decimal,
    pub margin_used: Decimal,
    pub leverage: Decimal,
    pub created_at: i64,
}

// 账户模型
pub struct Account {
    pub id: String,
    pub user_id: String,
    pub account_type: AccountType, // SPOT/MARGIN/FUTURES
    pub balances: HashMap<String, Balance>,
    pub total_equity: Decimal,
    pub available_margin: Decimal,
    pub used_margin: Decimal,
    pub margin_ratio: Decimal,
    pub created_at: i64,
}
```

## 🔧 技术实现要点

### 订单匹配
- **高性能匹配**: 毫秒级订单匹配
- **多种订单类型**: 支持市价单、限价单、止损单等
- **部分成交**: 支持订单部分成交处理
- **时间优先**: 按时间优先原则匹配

### 风险控制
- **保证金计算**: 实时保证金计算
- **强平机制**: 自动强平保护机制
- **风险监控**: 实时风险指标监控
- **风控规则**: 可配置风控规则

### 性能优化
- **异步处理**: Tokio异步运行时
- **批量操作**: 支持批量订单处理
- **缓存优化**: Redis缓存热点数据
- **数据库优化**: PostgreSQL索引优化

## 📊 监控指标

### 性能指标
- 订单处理延迟
- 执行成功率
- 系统吞吐量
- 内存使用率

### 业务指标
- 订单成功率
- 成交均价偏差
- 滑点控制
- 风控触发次数

## 🔐 安全措施

- **认证授权**: JWT认证 + RBAC权限控制
- **订单验证**: 严格的订单参数验证
- **资金安全**: 防止超额交易
- **审计日志**: 完整的交易审计日志

## 🚀 部署配置

### 环境变量
```
TRADING_ENGINE_PORT=8082
DATABASE_URL=postgresql://user:pass@localhost/trading
REDIS_URL=redis://localhost:6379
BINANCE_API_KEY=your_key
BINANCE_SECRET_KEY=your_secret
RISK_LIMITS_CONFIG_PATH=/config/risk_limits.json
ORDER_TIMEOUT=30
```

### Docker配置
- 多阶段构建
- 安全基镜像
- 资源限制

## 🧪 测试策略

### 单元测试
- 订单匹配算法测试
- 风控计算测试
- 数据模型测试

### 集成测试
- 端到端交易流程测试
- 多交易所适配器测试
- 风控规则测试

### 压力测试
- 高并发订单测试
- 大量持仓管理测试
- 系统稳定性测试