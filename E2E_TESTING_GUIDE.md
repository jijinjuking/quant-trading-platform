# 端到端集成测试指南

本文档描述如何进行完整的端到端集成测试，验证从行情采集到交易执行的完整链路。

## 📋 测试目标

验证以下完整数据流：

```
Market Data Service (行情采集)
    ↓ Kafka: market-events
Strategy Engine (策略计算)
    ↓ Kafka: strategy-signals
Trading Engine (交易执行)
    ↓ Kafka: execution-results
```

---

## 🚀 快速开始

### 前置条件

1. ✅ 基础设施已启动（Kafka、PostgreSQL、Redis）
2. ✅ Kafka Topics 已创建
3. ✅ 数据库 Schema 已初始化

### 启动服务

```bash
# 1. 启动 Market Data Service
cd services/market-data
RUST_LOG=info cargo run

# 2. 启动 Strategy Engine
cd services/strategy-engine
RUST_LOG=info cargo run

# 3. 启动 Trading Engine
cd services/trading-engine
RUST_LOG=info cargo run
```

---

## 🧪 测试场景

### 场景 1: 行情采集测试

**目标**: 验证 Market Data Service 能正确采集币安行情并发布到 Kafka

**步骤**:

1. 启动 Market Data Service
2. 观察日志，确认 WebSocket 连接成功
3. 使用 Kafka 消费者验证行情数据

```bash
# 消费 market-events 主题
kafka-console-consumer.sh \
    --bootstrap-server localhost:9092 \
    --topic market-events \
    --from-beginning \
    --max-messages 10
```

**预期结果**:

```json
{
  "event_type": "Trade",
  "exchange": "binance",
  "symbol": "BTCUSDT",
  "timestamp": "2026-01-23T10:30:00.123Z",
  "data": {
    "Trade": {
      "trade_id": "12345678",
      "price": "45000.50",
      "quantity": "0.001",
      "is_buyer_maker": false
    }
  }
}
```

**验证点**:
- ✅ WebSocket 连接成功
- ✅ 收到实时行情数据
- ✅ 数据格式正确
- ✅ 数据发布到 Kafka 成功

---

### 场景 2: 策略计算测试

**目标**: 验证 Strategy Engine 能消费行情数据并生成交易信号

**步骤**:

1. 确保 Market Data Service 正在运行
2. 启动 Strategy Engine
3. 观察日志，确认策略加载成功
4. 使用 Kafka 消费者验证策略信号

```bash
# 消费 strategy-signals 主题
kafka-console-consumer.sh \
    --bootstrap-server localhost:9092 \
    --topic strategy-signals \
    --from-beginning
```

**预期结果**:

```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "strategy_id": "660e8400-e29b-41d4-a716-446655440001",
  "symbol": "BTCUSDT",
  "side": "Buy",
  "price": "45000.00",
  "quantity": "0.001",
  "signal_type": "Entry",
  "timestamp": "2026-01-23T10:30:01.456Z"
}
```

**验证点**:
- ✅ 策略加载成功（日志显示 "Loaded 2 strategies"）
- ✅ 消费行情事件成功
- ✅ 策略计算执行
- ✅ 生成交易信号
- ✅ 信号发布到 Kafka 成功

---

### 场景 3: 交易执行测试

**目标**: 验证 Trading Engine 能消费策略信号并执行交易（模拟模式）

**步骤**:

1. 确保 Strategy Engine 正在运行
2. 启动 Trading Engine（模拟模式）
3. 观察日志，确认订单创建
4. 使用 Kafka 消费者验证执行结果

```bash
# 消费 execution-results 主题
kafka-console-consumer.sh \
    --bootstrap-server localhost:9092 \
    --topic execution-results \
    --from-beginning
```

**预期结果**:

```json
{
  "order_id": "770e8400-e29b-41d4-a716-446655440002",
  "signal_id": "550e8400-e29b-41d4-a716-446655440000",
  "symbol": "BTCUSDT",
  "side": "Buy",
  "status": "Filled",
  "filled_price": "45000.50",
  "filled_quantity": "0.001",
  "timestamp": "2026-01-23T10:30:02.789Z"
}
```

**验证点**:
- ✅ 消费策略信号成功
- ✅ 风控检查通过
- ✅ 订单创建成功
- ✅ 订单执行成功（模拟）
- ✅ 执行结果发布到 Kafka 成功

---

### 场景 4: 完整链路测试

**目标**: 验证从行情到交易的完整链路

**步骤**:

1. 同时启动所有三个服务
2. 等待 30 秒，让系统稳定
3. 观察日志，确认数据流转
4. 检查数据库，验证数据持久化

```bash
# 检查策略执行记录
psql -U postgres -d trading_platform -c "SELECT * FROM strategy_executions ORDER BY executed_at DESC LIMIT 10;"

# 检查订单记录
psql -U postgres -d trading_platform -c "SELECT * FROM orders ORDER BY created_at DESC LIMIT 10;"
```

**验证点**:
- ✅ 行情数据流转正常
- ✅ 策略计算正常
- ✅ 交易执行正常
- ✅ 数据持久化正常
- ✅ 无错误日志

---

## 📊 性能测试

### 延迟测试

**目标**: 测量从行情到信号的端到端延迟

```bash
# 在日志中查找时间戳
# Market Event: 2026-01-23T10:30:00.123Z
# Strategy Signal: 2026-01-23T10:30:00.456Z
# 延迟 = 456ms - 123ms = 333ms
```

**预期延迟**:
- 行情 → 策略: < 100ms
- 策略 → 交易: < 50ms
- 端到端: < 200ms

### 吞吐量测试

**目标**: 测试系统能处理的最大行情数据量

```bash
# 监控 Kafka 消费延迟
kafka-consumer-groups.sh \
    --bootstrap-server localhost:9092 \
    --describe \
    --group strategy-engine
```

**预期吞吐量**:
- Market Data: > 1000 events/s
- Strategy Engine: > 500 signals/s
- Trading Engine: > 200 orders/s

---

## 🔍 故障排查

### 问题 1: 行情数据未收到

**症状**: Kafka `market-events` 主题无数据

**排查步骤**:
1. 检查 Market Data Service 日志
2. 验证 WebSocket 连接状态
3. 检查网络连接（代理配置）
4. 验证 Kafka 连接

```bash
# 查看 Market Data Service 日志
docker logs market-data-service

# 测试 Kafka 连接
kafka-topics.sh --bootstrap-server localhost:9092 --list
```

### 问题 2: 策略未生成信号

**症状**: Kafka `strategy-signals` 主题无数据

**排查步骤**:
1. 检查 Strategy Engine 日志
2. 验证策略加载状态
3. 检查策略配置参数
4. 验证行情数据格式

```bash
# 查看 Strategy Engine 日志
docker logs strategy-engine

# 检查策略配置
psql -U postgres -d trading_platform -c "SELECT * FROM strategy_configs;"
```

### 问题 3: 订单未执行

**症状**: Kafka `execution-results` 主题无数据

**排查步骤**:
1. 检查 Trading Engine 日志
2. 验证风控规则
3. 检查账户余额
4. 验证交易所 API 配置

```bash
# 查看 Trading Engine 日志
docker logs trading-engine

# 检查风控记录
psql -U postgres -d trading_platform -c "SELECT * FROM risk_records ORDER BY created_at DESC LIMIT 10;"
```

---

## 📝 测试清单

### 功能测试

- [ ] Market Data Service 启动成功
- [ ] WebSocket 连接成功
- [ ] 行情数据采集正常
- [ ] 行情数据发布到 Kafka
- [ ] Strategy Engine 启动成功
- [ ] 策略加载成功
- [ ] 策略消费行情数据
- [ ] 策略生成交易信号
- [ ] 信号发布到 Kafka
- [ ] Trading Engine 启动成功
- [ ] 消费策略信号
- [ ] 风控检查执行
- [ ] 订单创建成功
- [ ] 订单执行成功（模拟）
- [ ] 执行结果发布到 Kafka

### 数据一致性测试

- [ ] 行情数据格式正确
- [ ] 策略信号格式正确
- [ ] 订单数据格式正确
- [ ] 数据库记录完整
- [ ] Kafka 消息无丢失

### 性能测试

- [ ] 端到端延迟 < 200ms
- [ ] 行情吞吐量 > 1000 events/s
- [ ] 策略吞吐量 > 500 signals/s
- [ ] 交易吞吐量 > 200 orders/s
- [ ] CPU 使用率 < 80%
- [ ] 内存使用率 < 80%

### 容错测试

- [ ] Kafka 断开重连
- [ ] 数据库断开重连
- [ ] WebSocket 断开重连
- [ ] 服务重启恢复
- [ ] 异常数据处理

---

## 🎯 成功标准

系统通过端到端测试的标准：

1. ✅ **功能完整性**: 所有功能测试项通过
2. ✅ **数据一致性**: 数据格式正确，无丢失
3. ✅ **性能达标**: 延迟和吞吐量满足要求
4. ✅ **稳定性**: 连续运行 1 小时无错误
5. ✅ **容错性**: 能从故障中自动恢复

---

## 📚 相关文档

- [Market Data Service 文档](../services/market-data/README.md)
- [Strategy Engine 文档](../services/strategy-engine/README.md)
- [Trading Engine 文档](../services/trading-engine/README.md)
- [基础设施配置](../infrastructure/README.md)

---

**最后更新**: 2026-01-23
