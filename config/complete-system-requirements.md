# 完整量化交易系统资源需求

## 系统组件清单

### 基础设施层
| 组件 | 内存需求 | CPU需求 | 说明 |
|------|----------|---------|------|
| PostgreSQL | 2GB | 2.0核 | 主数据库 |
| ClickHouse | 4GB | 2.0核 | 历史数据存储 |
| Redis | 512MB | 0.5核 | 缓存层 |
| Kafka | 1GB | 1.0核 | 消息队列 |
| Zookeeper | 512MB | 0.5核 | Kafka协调 |

### 业务服务层 (10个服务)
| 服务 | 内存需求 | CPU需求 | 副本数 | 说明 |
|------|----------|---------|--------|------|
| market-data | 512MB | 1.0核 | 3副本 | 市场数据服务 |
| trading-engine | 1GB | 1.5核 | 2副本 | 交易引擎 |
| strategy-engine | 512MB | 1.0核 | 1副本 | 策略引擎 |
| user-management | 512MB | 0.5核 | 1副本 | 用户管理 |
| risk-management | 512MB | 0.5核 | 1副本 | 风险管理 |
| notification | 256MB | 0.3核 | 1副本 | 通知服务 |
| ai-service | 1GB | 1.0核 | 1副本 | AI分析服务 |
| analytics | 512MB | 0.5核 | 1副本 | 数据分析 |
| gateway | 256MB | 0.3核 | 1副本 | API网关 |
| admin-backend | 256MB | 0.3核 | 1副本 | 管理后台 |

### 监控和管理层
| 组件 | 内存需求 | CPU需求 | 说明 |
|------|----------|---------|------|
| Prometheus | 1GB | 0.5核 | 监控数据收集 |
| Grafana | 512MB | 0.3核 | 监控面板 |
| Alertmanager | 256MB | 0.2核 | 告警管理 |
| Nginx | 128MB | 0.2核 | 负载均衡 |

### 开发和调试工具
| 组件 | 内存需求 | CPU需求 | 说明 |
|------|----------|---------|------|
| Kafka UI | 256MB | 0.2核 | Kafka管理界面 |
| Redis Commander | 128MB | 0.1核 | Redis管理界面 |

## 资源需求汇总

### 内存需求计算
```
基础设施层:
- PostgreSQL:        2GB
- ClickHouse:        4GB  
- Redis:             512MB
- Kafka:             1GB
- Zookeeper:         512MB
小计:                8GB

业务服务层:
- market-data:       512MB × 3 = 1.5GB
- trading-engine:    1GB × 2 = 2GB
- strategy-engine:   512MB × 1 = 512MB
- user-management:   512MB × 1 = 512MB
- risk-management:   512MB × 1 = 512MB
- notification:      256MB × 1 = 256MB
- ai-service:        1GB × 1 = 1GB
- analytics:         512MB × 1 = 512MB
- gateway:           256MB × 1 = 256MB
- admin-backend:     256MB × 1 = 256MB
小计:                7.3GB

监控管理层:
- Prometheus:        1GB
- Grafana:           512MB
- Alertmanager:      256MB
- Nginx:             128MB
小计:                1.9GB

开发工具:
- Kafka UI:          256MB
- Redis Commander:   128MB
小计:                384MB

系统预留:            1GB

总内存需求:          18.6GB
```

### CPU需求计算
```
基础设施层:
- PostgreSQL:        2.0核
- ClickHouse:        2.0核
- Redis:             0.5核
- Kafka:             1.0核
- Zookeeper:         0.5核
小计:                6.0核

业务服务层:
- market-data:       1.0核 × 3 = 3.0核
- trading-engine:    1.5核 × 2 = 3.0核
- strategy-engine:   1.0核 × 1 = 1.0核
- user-management:   0.5核 × 1 = 0.5核
- risk-management:   0.5核 × 1 = 0.5核
- notification:      0.3核 × 1 = 0.3核
- ai-service:        1.0核 × 1 = 1.0核
- analytics:         0.5核 × 1 = 0.5核
- gateway:           0.3核 × 1 = 0.3核
- admin-backend:     0.3核 × 1 = 0.3核
小计:                10.4核

监控管理层:
- Prometheus:        0.5核
- Grafana:           0.3核
- Alertmanager:      0.2核
- Nginx:             0.2核
小计:                1.2核

开发工具:
- Kafka UI:          0.2核
- Redis Commander:   0.1核
小计:                0.3核

系统预留:            1.1核

总CPU需求:           19.0核
```

### 磁盘需求计算
```
数据存储:
- PostgreSQL数据:    20GB
- ClickHouse数据:    100GB
- Redis持久化:       5GB
- Kafka日志:         20GB
- 应用日志:          10GB
小计:                155GB

系统和应用:
- 操作系统:          20GB
- Docker镜像:        15GB
- 编译产物:          10GB
小计:                45GB

预留空间:            50GB

总磁盘需求:          250GB
```

## 推荐服务器配置

### 最小生产配置
- **CPU**: 20核 (Intel Xeon或AMD EPYC)
- **内存**: 32GB DDR4
- **磁盘**: 500GB SSD
- **网络**: 1Gbps

### 推荐生产配置  
- **CPU**: 24-32核
- **内存**: 64GB DDR4
- **磁盘**: 1TB NVMe SSD
- **网络**: 10Gbps

### 高可用配置
- **CPU**: 32-48核
- **内存**: 128GB DDR4
- **磁盘**: 2TB NVMe SSD (RAID1)
- **网络**: 10Gbps (双网卡)

## 8GB+4核适配性结论

**❌ 8GB内存+4核CPU无法运行完整系统**

原因：
- 内存需求18.6GB，实际只有8GB (缺口10.6GB)
- CPU需求19核，实际只有4核 (缺口15核)
- 即使单副本部署也需要至少12GB内存+12核CPU

**建议最小配置**: 16GB内存 + 12核CPU + 250GB SSD