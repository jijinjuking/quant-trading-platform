# 量化交易平台 - 整体架构总览

## 文档状态

- 以本文件为当前架构总览（与代码对齐）
- 其他蓝图已加前缀 `废弃-`，仅保留历史参考

## 系统架构概览

```
[Frontend] -> [Gateway] -> [Business Services]
                           |-> PostgreSQL
                           |-> Redis
                           |-> ClickHouse
                           |-> Kafka

[Shared] 提供跨服务通用类型/事件/错误定义
```

### 层级说明

- 前端展示层：Vue3 + Vite + Pinia，面向用户交互与可视化
- API 网关层：统一入口、鉴权、限流、路由
- 业务服务层：领域能力拆分为独立服务
- 共享内核层：`shared/` 提供跨服务复用能力
- 数据存储层：PostgreSQL / Redis / ClickHouse / Kafka

### 服务清单（以代码为准）

- gateway：统一入口与路由
- market-data：行情采集与标准化
- trading-engine：订单与执行
- strategy-engine：策略与信号
- user-management：认证与用户
- risk-management：风控与告警
- notification：通知与推送
- analytics：统计与分析
- ai-service：AI 预测与辅助决策

> 端口与配置以各服务环境变量为准。

## 运行配置要点（market-data）

- `BINANCE_WS_URL`：币安 WS 地址（默认 `wss://stream.binance.com:9443/ws`）
- `MARKET_DATA_SYMBOLS`：订阅交易对，逗号分隔（默认 `btcusdt`）
- `KAFKA_BROKERS`：Kafka broker（默认 `localhost:9092`）
- `KAFKA_MARKET_TOPIC`：发布 topic（默认 `market-events`）
- `MARKET_DATA_PROXY`：代理地址（可选，例：`http://127.0.0.1:4780`）
  - 未设置时，自动回退读取 `HTTPS_PROXY` / `HTTP_PROXY`

## 共享内核（shared）

- `shared/types`：跨服务基础数据结构
- `shared/event`：跨服务事件定义
- `shared/error`：统一错误模型
- `shared/utils`：通用工具（无业务语义）

## 数据存储与基础设施

- PostgreSQL：业务事实数据（ACID）
- Redis：缓存/会话/限流计数
- ClickHouse：时序与统计查询
- Kafka：服务间异步事件流

## 单个服务内部架构模板（DDD + Hexagonal）

```
src/
├── main.rs               # 入口
├── state.rs              # 应用状态
├── bootstrap.rs          # 依赖注入
├── interface/            # 接口层
│   └── http/handlers/
├── application/          # 应用层（用例编排）
│   └── service/
├── domain/               # 领域层
│   ├── model/
│   ├── port/
│   └── event/            # 可选
└── infrastructure/       # 基础设施层（适配器）
    ├── persistence/
    ├── cache/
    ├── messaging/
    └── external/
```

### 依赖方向

```
interface -> application -> domain -> infrastructure
                    ^
               domain::port
```

- application 只依赖 `domain::port`
- infrastructure 只实现 `domain::port` 中的 trait
- adapter 仅在 `bootstrap.rs` / `main.rs` 组装

## 典型调用链（HTTP）

```
HTTP -> interface -> application -> domain -> port -> infrastructure -> DB
```

## 服务间通信机制

- 同步：HTTP REST（网关或服务间调用）
- 异步：Kafka 事件（解耦与最终一致）
- 实时：WebSocket（行情与通知推送）

## 技术栈概览

### 后端

- Rust + Axum + Tokio
- PostgreSQL：`deadpool-postgres` + `tokio-postgres`
- Redis：`redis`（异步连接管理）
- Kafka：`rdkafka`
- 序列化：`serde` / `serde_json`
- 配置：`config` + `dotenvy`
- 日志/监控：`tracing` + `prometheus`（按需接入）

### 前端

- Vue 3 + Pinia + Vite

### 基础设施

- Docker / Docker Compose
- Prometheus + Grafana
- Nginx

## Runtime config (strategy-engine)
- `KAFKA_BROKERS`: Kafka broker (default `localhost:9092`)
- `KAFKA_MARKET_TOPIC`: market event topic (default `market-events`)
- `KAFKA_SIGNAL_TOPIC`: signal publish topic (default `trading.signals`)
- `KAFKA_CONSUMER_GROUP`: consumer group id (default `strategy-engine`)
- `STRATEGY_ENGINE_PORT`: HTTP port (default `8083`)

## Runtime config (trading-engine)
- `KAFKA_BROKERS`: Kafka broker (default `localhost:9092`)
- `KAFKA_SIGNAL_TOPIC`: signal consume topic (default `trading.signals`)
- `KAFKA_CONSUMER_GROUP`: consumer group id (default `trading-engine`)
- `TRADING_ENGINE_PORT`: HTTP port (default `8081`)
- `TRADING_RISK_MIN_QTY`: 最小下单数量（可选）
- `TRADING_RISK_MAX_QTY`: 最大下单数量（可选）
- `TRADING_RISK_MAX_NOTIONAL`: 最大名义金额（可选）
- `TRADING_RISK_ALLOW_SYMBOLS`: 允许下单的交易对（逗号分隔，可选）
