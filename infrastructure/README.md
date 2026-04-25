# 基础设施配置指南

本目录包含量化交易平台的基础设施配置脚本。

## 📋 目录

- [Kafka Topics 配置](#kafka-topics-配置)
- [数据库 Schema 配置](#数据库-schema-配置)
- [ClickHouse 配置](#clickhouse-配置)

---

## 🔧 Kafka Topics 配置

### 脚本说明

- **Linux/Mac**: `kafka-topics.sh`
- **Windows**: `kafka-topics.ps1`

### 创建的 Topics

| Topic 名称 | 描述 | 生产者 | 消费者 |
|-----------|------|--------|--------|
| `market-events` | 行情事件 | Market Data Service | Strategy Engine |
| `strategy-signals` | 策略信号 | Strategy Engine | Trading Engine |
| `execution-drafts` | 执行草稿（跟单后） | CopyTrading Service | Trading Engine |
| `execution-results` | 执行结果 | Trading Engine | Commission Service |
| `order-events` | 订单事件 | Trading Engine | Analytics Service |
| `commission-records` | 分佣记录 | Commission Service | Accounting Service |
| `risk-alerts` | 风控告警 | Risk Management | Notification Service |
| `notifications` | 通知消息 | Notification Service | Gateway |
| `user-events` | 用户事件 | User Management | Analytics Service |

### 使用方法

#### Linux/Mac

```bash
# 使用默认配置
chmod +x kafka-topics.sh
./kafka-topics.sh

# 自定义配置
export KAFKA_BROKER="localhost:9092"
export PARTITIONS=3
export REPLICATION_FACTOR=1
./kafka-topics.sh
```

#### Windows (PowerShell)

```powershell
# 使用默认配置
.\kafka-topics.ps1

# 自定义配置
.\kafka-topics.ps1 -KafkaBroker "localhost:9092" -Partitions 3 -ReplicationFactor 1
```

### 验证 Topics

```bash
# 列出所有 Topics
kafka-topics.sh --bootstrap-server localhost:9092 --list

# 查看 Topic 详情
kafka-topics.sh --bootstrap-server localhost:9092 --describe --topic market-events
```

---

## 🗄️ 数据库 Schema 配置

### 脚本说明

- **文件**: `database-schema.sql`
- **数据库**: PostgreSQL 12+

### 数据表结构

#### 1. 用户管理 (User Management)
- `users` - 用户表
- `api_keys` - API Key 表

#### 2. 策略管理 (Strategy Management)
- `strategy_configs` - 策略配置表
- `strategy_executions` - 策略执行记录表

#### 3. 订单管理 (Order Management)
- `orders` - 订单表
- `order_fills` - 订单成交记录表

#### 4. 持仓管理 (Position Management)
- `positions` - 持仓表

#### 5. 跟单系统 (CopyTrading)
- `copytrading_relations` - 跟单关系表
- `copytrading_records` - 跟单记录表

#### 6. 分佣系统 (Commission)
- `commission_configs` - 分佣配置表
- `commission_records` - 分佣记录表

#### 7. 账户管理 (Account Management)
- `accounts` - 账户表
- `bills` - 账单表

#### 8. 风控管理 (Risk Management)
- `risk_rules` - 风控规则表
- `risk_records` - 风控记录表

#### 9. 通知管理 (Notification)
- `notifications` - 通知表

#### 10. 系统配置 (System Configuration)
- `system_configs` - 系统配置表

### 使用方法

#### 方法 1: 使用 psql 命令行

```bash
# 连接到 PostgreSQL
psql -U postgres -h localhost

# 执行 SQL 脚本
\i infrastructure/database-schema.sql

# 或者一行命令
psql -U postgres -h localhost -f infrastructure/database-schema.sql
```

#### 方法 2: 使用 Docker

```bash
# 如果使用 Docker 运行 PostgreSQL
docker exec -i postgres_container psql -U postgres < infrastructure/database-schema.sql
```

### 验证数据库

```sql
-- 连接到数据库
\c trading_platform

-- 查看所有表
\dt

-- 查看表结构
\d users
\d orders
\d positions

-- 查看索引
\di

-- 查看触发器
SELECT * FROM pg_trigger;
```

---

## 📊 ClickHouse 配置

### 表结构

ClickHouse 用于存储时序数据（行情数据、订单历史等）。

#### 行情数据表

```sql
CREATE TABLE IF NOT EXISTS market_data.trades (
    exchange String,
    symbol String,
    trade_id String,
    price Float64,
    quantity Float64,
    is_buyer_maker UInt8,
    event_time DateTime64(3),
    insert_time DateTime64(3)
) ENGINE = MergeTree()
PARTITION BY toYYYYMMDD(event_time)
ORDER BY (symbol, event_time, trade_id);
```

#### 订单历史表

```sql
CREATE TABLE IF NOT EXISTS market_data.orders_history (
    order_id String,
    user_id String,
    symbol String,
    side String,
    order_type String,
    price Float64,
    quantity Float64,
    filled_quantity Float64,
    status String,
    created_at DateTime64(3),
    updated_at DateTime64(3)
) ENGINE = MergeTree()
PARTITION BY toYYYYMMDD(created_at)
ORDER BY (user_id, created_at, order_id);
```

### 使用方法

```bash
# 连接到 ClickHouse
clickhouse-client --host localhost --port 9000

# 创建数据库
CREATE DATABASE IF NOT EXISTS market_data;

# 执行建表语句
# (将上面的 SQL 复制到 clickhouse-client 中执行)
```

---

## 🚀 快速开始

### 1. 启动基础设施

```bash
# 使用 Docker Compose 启动所有基础设施
docker-compose -f docker-compose.dev.yml up -d

# 等待服务启动（约 30 秒）
sleep 30
```

### 2. 初始化 Kafka Topics

```bash
# Linux/Mac
./infrastructure/kafka-topics.sh

# Windows
.\infrastructure\kafka-topics.ps1
```

### 3. 初始化数据库

```bash
# PostgreSQL
psql -U postgres -h localhost -f infrastructure/database-schema.sql

# ClickHouse
clickhouse-client --host localhost --port 9000 < infrastructure/clickhouse-schema.sql
```

### 4. 验证配置

```bash
# 验证 Kafka
kafka-topics.sh --bootstrap-server localhost:9092 --list

# 验证 PostgreSQL
psql -U postgres -h localhost -d trading_platform -c "\dt"

# 验证 ClickHouse
clickhouse-client --host localhost --port 9000 --query "SHOW DATABASES"
```

---

## 🔍 故障排查

### Kafka 连接失败

```bash
# 检查 Kafka 是否运行
docker ps | grep kafka

# 查看 Kafka 日志
docker logs kafka_container

# 测试连接
kafka-broker-api-versions.sh --bootstrap-server localhost:9092
```

### PostgreSQL 连接失败

```bash
# 检查 PostgreSQL 是否运行
docker ps | grep postgres

# 查看日志
docker logs postgres_container

# 测试连接
psql -U postgres -h localhost -c "SELECT version();"
```

### ClickHouse 连接失败

```bash
# 检查 ClickHouse 是否运行
docker ps | grep clickhouse

# 查看日志
docker logs clickhouse_container

# 测试连接
clickhouse-client --host localhost --port 9000 --query "SELECT 1"
```

---

## 📝 环境变量

### Kafka

```bash
KAFKA_BROKERS=localhost:9092
KAFKA_MARKET_TOPIC=market-events
KAFKA_SIGNAL_TOPIC=strategy-signals
KAFKA_CONSUMER_GROUP=strategy-engine
```

### PostgreSQL

```bash
DATABASE_URL=postgresql://postgres:password@localhost:5432/trading_platform
DATABASE_HOST=localhost
DATABASE_PORT=5432
DATABASE_USER=postgres
DATABASE_PASSWORD=password
DATABASE_NAME=trading_platform
```

### ClickHouse

```bash
CLICKHOUSE_URL=http://localhost:8123
CLICKHOUSE_DATABASE=market_data
CLICKHOUSE_TABLE=trades
```

---

## 🔒 安全建议

1. **生产环境**：
   - 修改默认密码
   - 启用 SSL/TLS
   - 配置防火墙规则
   - 限制网络访问

2. **Kafka**：
   - 启用 SASL 认证
   - 配置 ACL 权限
   - 启用加密传输

3. **数据库**：
   - 使用强密码
   - 限制远程访问
   - 定期备份
   - 启用审计日志

---

## 📚 参考文档

- [Kafka 官方文档](https://kafka.apache.org/documentation/)
- [PostgreSQL 官方文档](https://www.postgresql.org/docs/)
- [ClickHouse 官方文档](https://clickhouse.com/docs/)

---

**最后更新**: 2026-01-23
