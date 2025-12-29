# DDD架构规范 (DDD Architecture Standard)

> **强制执行**: 所有服务必须遵循此架构规范

---

## 一、全局裁决（统一世界观）

### 架构范式
- **系统级**: 微服务
- **服务内部**: DDD（Engine-DDD / Business-DDD 混合）

### 通信方式
- **同步**: HTTP / gRPC
- **异步**: Kafka

### 状态存储
- **PostgreSQL**: 业务事实
- **Redis**: 短期状态 / 会话
- **ClickHouse**: 时间序列 / 统计

---

## 二、顶层代码组织（Mono Repo，Workspace）

```
quant-platform/
├── Cargo.toml                # Rust workspace
├── docker-compose.yml
├── .env.example
├── README.md
│
├── crates/                   # 或 services/
│   ├── gateway/              # API Gateway
│   ├── market-data/          # 行情服务
│   ├── trading-engine/       # 交易执行引擎
│   ├── strategy-engine/      # 策略计算引擎
│   ├── risk-engine/          # 风控引擎
│   ├── user-service/         # 用户服务
│   ├── notification/         # 通知服务
│   ├── analytics/            # 数据分析
│   ├── ai-service/           # AI 服务
│   └── shared/               # 共享内核
```

---

## 三、共享内核（Shared Kernel）

**唯一可以被所有服务共同依赖的地方**

```
crates/shared/
├── Cargo.toml
└── src/
    ├── lib.rs
    │
    ├── types/                # 纯数据结构
    │   ├── user.rs
    │   ├── order.rs
    │   ├── position.rs
    │   ├── market.rs
    │   └── strategy.rs
    │
    ├── error/                # 统一错误
    │   ├── domain_error.rs
    │   ├── infra_error.rs
    │   └── api_error.rs
    │
    ├── event/                # Kafka 事件定义
    │   ├── order_event.rs
    │   ├── trade_event.rs
    │   └── risk_event.rs
    │
    └── utils/                # 工具（无业务语义）
        ├── time.rs
        └── id.rs
```

---

## 四、服务内部统一骨架（模板）

**每一个"引擎/服务"内部的标准结构：**

```
src/
├── main.rs                    # 服务启动
├── lib.rs                     # 对外能力
├── state.rs                   # AppState（DB / Cache / Config）
│
├── interface/                 # 【接口层】
│   ├── http/                  # HTTP API
│   │   ├── handlers/          # 请求适配
│   │   └── routes.rs
│   └── grpc/ (可选)
│
├── application/               # 【应用层】（编排）
│   ├── mod.rs
│   └── service/               # 用例编排
│       └── *.rs
│
├── domain/ ⭐                  # 【核心层】
│   ├── model/                 # 状态模型
│   ├── logic/                 # 业务规则 / 算法
│   └── service/               # 跨模型规则
│
├── infrastructure/            # 【基础设施层】
│   ├── repository/
│   ├── db/
│   ├── cache/
│   └── messaging/
```

---

## 五、各服务落地骨架

### 1️⃣ Strategy Engine（策略计算）

**职责**: 接收行情/用户配置 → 计算交易信号 → 输出信号事件

```
strategy-engine/
└── src/
    ├── main.rs
    ├── lib.rs
    ├── state.rs
    │
    ├── interface/http/handlers/
    │   ├── strategies.rs       # 管理策略
    │   └── backtest.rs         # 回测接口
    │
    ├── application/service/
    │   ├── strategy_service.rs # 策略用例编排
    │   └── backtest_service.rs
    │
    ├── domain/
    │   ├── model/
    │   │   ├── strategy.rs     # 策略状态
    │   │   └── signal.rs       # Signal
    │   ├── logic/
    │   │   ├── grid.rs         # 网格算法
    │   │   └── mean.rs         # 均值回归
    │   └── service/
    │       └── signal_generator.rs
    │
    └── infrastructure/
        └── messaging/
            └── kafka_producer.rs  # 发送 SignalEvent
```

### 2️⃣ Trading Engine（订单执行）

**职责**: 接收交易信号 → 下单/撤单 → 同步交易所

```
trading-engine/
└── src/
    ├── application/service/
    │   └── execution_service.rs
    │
    ├── domain/
    │   ├── model/
    │   │   ├── order.rs
    │   │   └── trade.rs
    │   └── logic/
    │       └── execution_algo.rs
    │
    └── infrastructure/
        ├── exchange/
        │   └── binance.rs
        └── messaging/
            └── kafka_consumer.rs
```

### 3️⃣ Risk Engine（风控）

**职责**: 订单前检查 → 仓位/敞口/回撤校验

```
risk-engine/
└── src/
    ├── application/service/
    │   └── risk_check_service.rs
    │
    ├── domain/
    │   ├── model/
    │   │   └── risk_profile.rs
    │   ├── logic/
    │   │   ├── leverage.rs
    │   │   └── drawdown.rs
    │   └── service/
    │       └── risk_evaluator.rs
```

### 4️⃣ Market Data（行情）

**职责**: WebSocket拉行情 → 统一格式 → 推送事件

```
market-data/
└── src/
    ├── domain/
    │   └── model/
    │       └── tick.rs
    │
    └── infrastructure/
        ├── exchange/
        │   └── binance_ws.rs
        └── messaging/
            └── kafka_producer.rs
```

### 5️⃣ User Service（业务DDD）

**职责**: 标准业务DDD - 用户管理

```
user-service/
└── src/
    ├── interface/http/
    ├── application/service/
    ├── domain/
    │   ├── model/entity/user.rs
    │   ├── repository/user_repo.rs (trait)
    │   └── service/user_domain_service.rs
    └── infrastructure/persistence/
```

---

## 六、依赖方向规则

```
interface → application → domain ← infrastructure
                           ↑
                      核心层不依赖任何外层
```

- **Domain层**: 纯业务逻辑，不依赖任何框架
- **Repository**: Domain层定义trait，Infrastructure层实现
- **依赖倒置**: 外层依赖内层，内层不知道外层存在

---

## 七、违规检查清单

| 检查项 | 违规示例 | 正确做法 |
|--------|---------|---------|
| Domain依赖框架 | `use axum::*` in domain/ | Domain只用std和shared |
| 跨层直接调用 | handler直接调repository | handler → service → repository |
| 重复目录 | handlers/ + interface/http/ | 只保留 interface/http/handlers/ |
| 模型分散 | models/ + domain/model/ | 只保留 domain/model/ |
| 存储分散 | storage/ + infrastructure/ | 只保留 infrastructure/repository/ |
