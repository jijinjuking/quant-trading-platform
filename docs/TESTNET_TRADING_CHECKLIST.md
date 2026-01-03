# 🚀 Binance Testnet 真实交易最小闭环检查清单

> **目标**: 在 Binance Testnet 上完成一笔真实交易
> **风险等级**: 低（测试网，无真实资金）
> **预计时间**: 30 分钟

---

## 一、前置条件检查

### 1.1 基础设施

| 组件 | 检查项 | 命令 | 预期结果 |
|------|--------|------|----------|
| Kafka | 服务运行 | `docker ps \| grep kafka` | 容器运行中 |
| Kafka | Topic 存在 | `kafka-topics --list --bootstrap-server localhost:9092` | 包含 `market-events` |
| PostgreSQL | 服务运行 | `docker ps \| grep postgres` | 容器运行中（可选） |
| 代理 | 可访问币安 | `curl -x http://127.0.0.1:4780 https://api.binance.com/api/v3/ping` | `{}` |

### 1.2 环境变量 (.env)

```bash
# 必须配置
BINANCE_API_KEY=<你的测试网 API Key>
BINANCE_SECRET_KEY=<你的测试网 Secret Key>
BINANCE_BASE_URL=https://testnet.binance.vision
TRADING_EXECUTION_MODE=binance

# Kafka
KAFKA_BROKERS=localhost:9092
KAFKA_MARKET_TOPIC=market-events

# 代理
HTTP_PROXY=http://127.0.0.1:4780
HTTPS_PROXY=http://127.0.0.1:4780
MARKET_DATA_PROXY=http://127.0.0.1:4780

# 风控（测试用宽松配置）
TRADING_RISK_ALLOW_SYMBOLS=BTCUSDT
TRADING_RISK_MIN_QTY=0.0001
TRADING_RISK_MAX_QTY=0.01
TRADING_RISK_MAX_NOTIONAL=1000

# 策略
STRATEGY_TYPE=grid
STRATEGY_GRID_UPPER=100000
STRATEGY_GRID_LOWER=80000
STRATEGY_GRID_COUNT=100
STRATEGY_GRID_QUANTITY=0.001
```

### 1.3 Binance Testnet 账户

1. 访问 https://testnet.binance.vision/
2. 登录 GitHub 账号
3. 生成 API Key 和 Secret Key
4. 确认测试账户有 BTC 和 USDT 余额

---

## 二、服务启动顺序

### 2.1 启动 Kafka（如果未运行）

```bash
docker-compose up -d kafka zookeeper
```

### 2.2 启动 strategy-engine (8083)

```bash
cargo run -p strategy-engine
```

预期日志：
```
INFO Strategy Engine listening on 0.0.0.0:8083
```

### 2.3 启动 trading-engine (8081)

```bash
cargo run -p trading-engine
```

预期日志：
```
INFO Trading Engine listening on 0.0.0.0:8081
INFO 交易审计已启用 (noop mode)
INFO Config loaded kafka_brokers="localhost:9092" kafka_market_topic="market-events"
```

### 2.4 启动 market-data (8082)

```bash
cargo run -p market-data
```

预期日志：
```
INFO Market Data Service starting...
INFO WebSocket connected to wss://stream.binance.com:9443/ws
INFO Subscribed to: btcusdt@trade
```

---

## 三、功能验证

### 3.1 健康检查

```bash
# strategy-engine
curl http://localhost:8083/health
# 预期: {"status":"ok"}

# trading-engine
curl http://localhost:8081/health
# 预期: {"status":"ok"}
```

### 3.2 策略评估 API 测试

```bash
curl -X POST http://localhost:8083/api/v1/strategy/evaluate \
  -H "Content-Type: application/json" \
  -d '{
    "strategy_id": "00000000-0000-0000-0000-000000000001",
    "symbol": "BTCUSDT",
    "price": "87000.00",
    "quantity": "0.001",
    "timestamp": 1704067200000,
    "is_buyer_maker": false
  }'
```

预期响应（有信号时）：
```json
{
  "success": true,
  "data": {
    "has_intent": true,
    "intent": {
      "strategy_id": "...",
      "symbol": "BTCUSDT",
      "side": "buy",
      "quantity": "0.001",
      ...
    }
  }
}
```

### 3.3 Kafka 消息流验证

```bash
# 监听 market-events topic
kafka-console-consumer --bootstrap-server localhost:9092 --topic market-events --from-beginning
```

预期：看到 JSON 格式的 MarketEvent 消息

### 3.4 交易日志验证

观察 trading-engine 日志，应该看到：

```
INFO Strategy generated order intent symbol="BTCUSDT" side=Buy quantity=0.001
INFO RISK_PASSED: Order intent passed risk check
INFO Order executed successfully order_id="xxx"
```

或者风控拒绝：

```
INFO RISK_REJECTED: Order intent rejected by risk check reject_reason="..."
```

---

## 四、真实下单测试

### 4.1 手动触发下单（绕过策略）

如果策略没有产生信号，可以直接调用 trading-engine 的测试接口（如果有）或修改策略配置使其更容易触发。

### 4.2 验证 Binance Testnet 订单

1. 登录 https://testnet.binance.vision/
2. 查看订单历史
3. 确认订单已创建

---

## 五、问题排查

### 5.1 常见问题

| 问题 | 可能原因 | 解决方案 |
|------|----------|----------|
| WebSocket 连接失败 | 代理配置错误 | 检查 `MARKET_DATA_PROXY` |
| Kafka 连接失败 | Kafka 未启动 | `docker-compose up -d kafka` |
| 策略无信号 | 价格不在网格范围 | 调整 `STRATEGY_GRID_*` 配置 |
| 风控拒绝 | Symbol 不在白名单 | 检查 `TRADING_RISK_ALLOW_SYMBOLS` |
| 下单失败 | API Key 错误 | 检查 `BINANCE_API_KEY` |
| 签名错误 | Secret Key 错误 | 检查 `BINANCE_SECRET_KEY` |

### 5.2 日志级别调整

```bash
# 启用 DEBUG 日志
RUST_LOG=debug cargo run -p trading-engine
```

---

## 六、成功标准

✅ **最小闭环完成标准**：

1. [ ] market-data 能连接币安 WebSocket 并收到行情
2. [ ] 行情数据能发送到 Kafka `market-events` topic
3. [ ] trading-engine 能消费 Kafka 消息
4. [ ] trading-engine 能调用 strategy-engine HTTP API
5. [ ] strategy-engine 能返回交易意图
6. [ ] 风控检查能正常执行（通过或拒绝）
7. [ ] 通过风控后能调用 Binance Testnet API 下单
8. [ ] Binance Testnet 上能看到订单记录

---

## 七、下一步

完成最小闭环后，可以继续：

1. **完善 gateway** - 统一 API 入口
2. **完善 user-management** - 用户认证
3. **添加更多策略** - MACD、RSI 等
4. **完善风控** - 回撤检查、杠杆检查
5. **添加监控** - Prometheus + Grafana
6. **准备主网** - 更严格的风控配置

---

**⚠️ 警告**: 在主网交易前，必须：
- 完成充分的测试网测试
- 配置严格的风控参数
- 设置合理的仓位限制
- 准备紧急停止机制
