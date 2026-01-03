# 基础设施 (infrastructure) - 架构设计

## 📋 服务概述

### 服务名称
基础设施 (Infrastructure)

### 服务职责
- 监控系统 (Prometheus + Grafana)
- 日志管理 (ELK Stack)
- CI/CD流水线
- 容器编排 (Kubernetes/Docker Compose)
- 安全管理
- 备份与恢复
- 性能调优

## 🏗️ 服务架构

### 内部架构图
```
infrastructure/
│
├── monitoring/                 # 监控系统
│   ├── prometheus/            # Prometheus配置
│   │   ├── prometheus.yml     # Prometheus主配置
│   │   ├── rules/             # 告警规则
│   │   │   ├── general_rules.yml
│   │   │   ├── performance_rules.yml
│   │   │   └── business_rules.yml
│   │   └── targets/           # 目标配置
│   │       ├── services.yml   # 服务发现配置
│   │       └── databases.yml  # 数据库监控配置
│   │
│   ├── grafana/               # Grafana配置
│   │   ├── dashboards/        # 仪表板配置
│   │   │   ├── system.json    # 系统资源仪表板
│   │   │   ├── services.json  # 服务性能仪表板
│   │   │   ├── business.json  # 业务指标仪表板
│   │   │   └── custom/        # 自定义仪表板
│   │   ├── datasources/       # 数据源配置
│   │   │   └── prometheus.yml
│   │   └── provisioning/      # 预配置
│   │       ├── dashboards.yml
│   │       └── datasources.yml
│   │
│   └── exporters/             # 数据导出器
│       ├── node_exporter/     # 节点导出器
│       ├── postgres_exporter/ # PostgreSQL导出器
│       ├── redis_exporter/    # Redis导出器
│       └── custom_exporters/  # 自定义导出器
│
├── logging/                   # 日志系统
│   ├── elasticsearch/         # Elasticsearch配置
│   │   ├── elasticsearch.yml
│   │   └── jvm.options
│   ├── logstash/              # Logstash配置
│   │   ├── pipelines.yml
│   │   ├── config/            # 配置文件
│   │   │   ├── input.conf     # 输入配置
│   │   │   ├── filter.conf    # 过滤配置
│   │   │   └── output.conf    # 输出配置
│   │   └── patterns/          # 自定义模式
│   └── kibana/                # Kibana配置
│       └── kibana.yml
│
├── ci-cd/                     # CI/CD配置
│   ├── github-actions/        # GitHub Actions
│   │   ├── build.yml          # 构建流水线
│   │   ├── test.yml           # 测试流水线
│   │   ├── deploy.yml         # 部署流水线
│   │   └── security.yml       # 安全扫描流水线
│   ├── docker/                # Docker配置
│   │   ├── Dockerfile         # 基础镜像
│   │   ├── Dockerfile.prod    # 生产镜像
│   │   ├── docker-compose.yml
│   │   ├── docker-compose.prod.yml
│   │   └── docker-compose.dev.yml
│   └── kubernetes/            # Kubernetes配置
│       ├── deployments/       # 部署配置
│       ├── services/          # 服务配置
│       ├── ingress/           # 入口配置
│       ├── configmaps/        # 配置映射
│       ├── secrets/           # 密钥配置
│       └── helm/              # Helm图表
│
├── security/                  # 安全配置
│   ├── nginx/                 # Nginx配置
│   │   ├── nginx.conf         # 主配置
│   │   ├── ssl.conf           # SSL配置
│   │   ├── security.conf      # 安全配置
│   │   └── rate-limit.conf    # 限流配置
│   ├── certificates/          # 证书管理
│   │   ├── ca.crt             # CA证书
│   │   ├── server.crt         # 服务器证书
│   │   └── server.key         # 服务器私钥
│   └── firewall/              # 防火墙配置
│       └── rules.conf
│
├── backup/                    # 备份配置
│   ├── postgres/              # PostgreSQL备份
│   │   ├── backup.sh          # 备份脚本
│   │   ├── restore.sh         # 恢复脚本
│   │   └── cron.conf          # 定时任务配置
│   ├── redis/                 # Redis备份
│   ├── clickhouse/            # ClickHouse备份
│   └── application/           # 应用数据备份
│
└── performance/               # 性能优化
    ├── tuning/                # 系统调优
    │   ├── sysctl.conf        # 系统参数调优
    │   ├── limits.conf        # 限制配置
    │   └── network.conf       # 网络调优
    ├── resource/              # 资源配置
    │   ├── limits.yml         # 资源限制
    │   └── requests.yml       # 资源请求
    └── optimization/          # 优化脚本
        ├── db_optimize.sql    # 数据库优化
        └── cache_warmup.sh    # 缓存预热
```

## 🔄 数据流向

### 监控数据流向
```
应用服务 (OpenTelemetry metrics)
    ↓
Prometheus Exporters
    ↓
Prometheus Server (数据收集)
    ↓
Grafana (数据展示)
    ↓
告警系统 (Prometheus Alertmanager)
    ↓
通知服务 (Email/Slack/Webhook)
```

### 日志数据流向
```
应用服务 (Structured logs)
    ↓
Logstash (收集和处理)
    ↓
Elasticsearch (存储和索引)
    ↓
Kibana (查询和可视化)
    ↓
告警系统 (异常检测)
```

### CI/CD流程
```
代码提交 (Git)
    ↓
GitHub Actions (构建和测试)
    ↓
Docker (容器化)
    ↓
Kubernetes (部署)
    ↓
健康检查 (验证部署)
    ↓
监控系统 (持续监控)
```

## 📡 配置管理

### 监控配置

#### Prometheus配置 (prometheus.yml)
```yaml
global:
  scrape_interval: 15s
  evaluation_interval: 15s

rule_files:
  - "rules/general_rules.yml"
  - "rules/performance_rules.yml"
  - "rules/business_rules.yml"

scrape_configs:
  - job_name: 'gateway'
    static_configs:
      - targets: ['gateway:8080']
    metrics_path: /metrics
    scrape_interval: 5s

  - job_name: 'user-management'
    static_configs:
      - targets: ['user-management:8081']
    metrics_path: /metrics
    scrape_interval: 10s

  - job_name: 'trading-engine'
    static_configs:
      - targets: ['trading-engine:8082']
    metrics_path: /metrics
    scrape_interval: 10s

  - job_name: 'market-data'
    static_configs:
      - targets: ['market-data:8083']
    metrics_path: /metrics
    scrape_interval: 5s

  - job_name: 'strategy-engine'
    static_configs:
      - targets: ['strategy-engine:8084']
    metrics_path: /metrics
    scrape_interval: 10s

  - job_name: 'risk-management'
    static_configs:
      - targets: ['risk-management:8085']
    metrics_path: /metrics
    scrape_interval: 15s

  - job_name: 'notification'
    static_configs:
      - targets: ['notification:8086']
    metrics_path: /metrics
    scrape_interval: 15s

  - job_name: 'analytics'
    static_configs:
      - targets: ['analytics:8087']
    metrics_path: /metrics
    scrape_interval: 30s

  - job_name: 'ai-service'
    static_configs:
      - targets: ['ai-service:8088']
    metrics_path: /metrics
    scrape_interval: 30s

  - job_name: 'databases'
    static_configs:
      - targets: ['postgres-exporter:9187', 'redis-exporter:9121', 'clickhouse:8123']
    scrape_interval: 30s
```

#### 告警规则 (rules/business_rules.yml)
```yaml
groups:
  - name: business_rules
    rules:
      - alert: HighErrorRate
        expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.1
        for: 2m
        labels:
          severity: critical
        annotations:
          summary: "High error rate detected"
          description: "Error rate is above 10% for 5 minutes: {{ $value }}"

      - alert: HighLatency
        expr: histogram_quantile(0.95, http_request_duration_seconds_bucket) > 1
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High request latency"
          description: "95th percentile latency is above 1s: {{ $value }}s"

      - alert: LowSuccessRate
        expr: (1 - rate(http_requests_total{status=~"5.."}[5m]) / rate(http_requests_total[5m])) < 0.95
        for: 5m
        labels:
          severity: critical
        annotations:
          summary: "Low success rate"
          description: "Success rate is below 95%: {{ $value }}"

      - alert: HighCPUUsage
        expr: 100 - (avg by(instance) (rate(node_cpu_seconds_total{mode="idle"}[5m])) * 100) > 80
        for: 5m
        labels:
          severity: warning
        annotations:
          summary: "High CPU usage"
          description: "CPU usage is above 80%: {{ $value }}%"
```

### 日志配置

#### Logstash配置 (config/filter.conf)
```
filter {
  if [type] == "application" {
    grok {
      match => { "message" => "%{TIMESTAMP_ISO8601:timestamp} \[%{LOGLEVEL:level}\] %{GREEDYDATA:logger} - %{GREEDYDATA:content}" }
    }
    
    date {
      match => [ "timestamp", "ISO8601" ]
    }
    
    if [level] == "ERROR" or [level] == "FATAL" {
      mutate {
        add_tag => [ "error" ]
      }
    }
  }
  
  if [type] == "access" {
    grok {
      match => { "message" => "%{IPORHOST:clientip} %{USER:ident} %{USER:auth} \[%{HTTPDATE:timestamp}\] \"%{WORD:verb} %{DATA:request} HTTP/%{NUMBER:httpversion}\" %{NUMBER:response} (?:%{NUMBER:bytes}|-) %{QS:referrer} %{QS:agent}" }
    }
  }
}
```

### CI/CD配置

#### GitHub Actions构建流水线 (.github/workflows/build.yml)
```yaml
name: Build and Test

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        rust-version: [1.70.0]
        
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust-version }}
        override: true
        
    - name: Cache dependencies
      uses: actions/cache@v3
      with:
        path: |
          ~/.cargo/registry
          ~/.cargo/git
          target
        key: ${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}
        
    - name: Install dependencies
      run: |
        rustup component add clippy rustfmt
        cargo install cargo-audit
        
    - name: Check formatting
      run: cargo fmt --all -- --check
      
    - name: Run clippy
      run: cargo clippy -- -D warnings
      
    - name: Run tests
      run: cargo test --verbose
      
    - name: Security audit
      run: cargo audit
```

## 🗄️ 基础设施组件

### 监控系统组件
```yaml
# docker-compose.monitoring.yml
version: '3.8'

services:
  prometheus:
    image: prom/prometheus:latest
    container_name: prometheus
    ports:
      - "9090:9090"
    volumes:
      - ./monitoring/prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus
    command:
      - '--config.file=/etc/prometheus/prometheus.yml'
      - '--storage.tsdb.path=/prometheus'
      - '--web.console.libraries=/etc/prometheus/console_libraries'
      - '--web.console.templates=/etc/prometheus/consoles'
      - '--storage.tsdb.retention.time=200h'
      - '--web.enable-lifecycle'

  grafana:
    image: grafana/grafana:latest
    container_name: grafana
    ports:
      - "3000:3000"
    volumes:
      - ./monitoring/grafana/provisioning:/etc/grafana/provisioning
      - grafana_data:/var/lib/grafana
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=admin
      - GF_USERS_ALLOW_SIGN_UP=false
    depends_on:
      - prometheus

  alertmanager:
    image: prom/alertmanager:latest
    container_name: alertmanager
    ports:
      - "9093:9093"
    volumes:
      - ./monitoring/alertmanager/config.yml:/etc/alertmanager/config.yml
    command:
      - '--config.file=/etc/alertmanager/config.yml'
      - '--storage.path=/alertmanager'

volumes:
  prometheus_data:
  grafana_data:
```

### 日志系统组件
```yaml
# docker-compose.logging.yml
version: '3.8'

services:
  elasticsearch:
    image: docker.elastic.co/elasticsearch/elasticsearch:7.17.0
    container_name: elasticsearch
    environment:
      - discovery.type=single-node
      - "ES_JAVA_OPTS=-Xms512m -Xmx512m"
    ports:
      - "9200:9200"
    volumes:
      - es_data:/usr/share/elasticsearch/data

  logstash:
    image: docker.elastic.co/logstash/logstash:7.17.0
    container_name: logstash
    ports:
      - "5044:5044"
      - "5000:5000/tcp"
      - "5000:5000/udp"
      - "9600:9600"
    volumes:
      - ./logging/logstash/config:/usr/share/logstash/pipeline
    environment:
      - "LS_JAVA_OPTS=-Xms256m -Xmx256m"
    depends_on:
      - elasticsearch

  kibana:
    image: docker.elastic.co/kibana/kibana:7.17.0
    container_name: kibana
    ports:
      - "5601:5601"
    environment:
      - ELASTICSEARCH_HOSTS=http://elasticsearch:9200
    depends_on:
      - elasticsearch

volumes:
  es_data:
```

### 安全组件
```nginx
# nginx.conf
upstream backend {
    server gateway:8080;
}

server {
    listen 80;
    server_name localhost;
    
    # SSL配置
    listen 443 ssl http2;
    ssl_certificate /etc/ssl/certs/server.crt;
    ssl_certificate_key /etc/ssl/private/server.key;
    
    # 安全头
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-XSS-Protection "1; mode=block" always;
    add-header X-Content-Type-Options "nosniff" always;
    add_header Referrer-Policy "no-referrer-when-downgrade" always;
    add_header Content-Security-Policy "default-src 'self' http: https: data: blob: 'unsafe-inline'" always;
    
    # 限流配置
    limit_req_zone $binary_remote_addr zone=api:10m rate=10r/s;
    limit_req zone=api burst=20 nodelay;
    
    location / {
        proxy_pass http://backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        
        # 超时配置
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }
    
    # 静态文件缓存
    location ~* \.(jpg|jpeg|png|gif|ico|css|js)$ {
        expires 1y;
        add_header Cache-Control "public, immutable";
    }
}
```

## 🔧 性能优化策略

### 数据库优化
```sql
-- PostgreSQL性能优化脚本
-- 增加连接数
ALTER SYSTEM SET max_connections = 200;

-- 调整共享缓冲区
ALTER SYSTEM SET shared_buffers = '256MB';

-- 调整工作内存
ALTER SYSTEM SET work_mem = '16MB';

-- 启用查询计划缓存
ALTER SYSTEM SET plan_cache_mode = force_generic_plan;

-- 创建索引优化
CREATE INDEX CONCURRENTLY idx_orders_user_id_status_created 
ON orders(user_id, status, created_at);

CREATE INDEX CONCURRENTLY idx_klines_symbol_timeframe_start_time 
ON klines(symbol, timeframe, start_time);

-- 分区表优化
CREATE TABLE klines_2025 PARTITION OF klines
FOR VALUES FROM ('2025-01-01') TO ('2026-01-01');
```

### 系统参数优化
```bash
# sysctl.conf - 系统参数调优
net.core.somaxconn = 65535
net.core.netdev_max_backlog = 5000
net.ipv4.tcp_max_syn_backlog = 65535
net.ipv4.tcp_fin_timeout = 10
net.ipv4.tcp_keepalive_time = 1200
net.ipv4.tcp_window_scaling = 1
net.ipv4.tcp_sack = 1
net.ipv4.tcp_timestamps = 1
vm.swappiness = 1
vm.overcommit_memory = 1
fs.file-max = 1000000
```

## 📊 监控指标体系

### 系统级指标
- CPU使用率
- 内存使用率
- 磁盘I/O
- 网络带宽
- 文件描述符使用

### 应用级指标
- 请求延迟 (P50, P95, P99)
- 请求速率 (RPS)
- 错误率
- 活跃连接数
- 线程池状态

### 业务级指标
- 订单处理延迟
- 策略执行成功率
- 风险指标监控
- 用户活跃度
- 交易量统计

## 🔐 安全措施

### 网络安全
- **防火墙**: 限制不必要的端口访问
- **SSL/TLS**: 全站HTTPS加密
- **DDoS防护**: 限流和熔断机制
- **WAF**: Web应用防火墙

### 数据安全
- **数据加密**: 敏感数据传输和存储加密
- **访问控制**: RBAC权限管理
- **审计日志**: 完整的操作审计
- **备份加密**: 备份数据加密存储

### 应用安全
- **输入验证**: 严格的参数验证
- **SQL注入防护**: 使用参数化查询
- **XSS防护**: 输出编码和内容安全策略
- **CSRF防护**: CSRF令牌验证

## 🚀 部署策略

### 高可用部署
- **多副本部署**: 关键服务多副本部署
- **负载均衡**: 智能负载均衡和故障转移
- **自动扩缩容**: 基于指标的自动扩缩容
- **滚动更新**: 零停机滚动更新

### 灰度发布
- **蓝绿部署**: 蓝绿环境切换
- **金丝雀发布**: 渐进式流量切换
- **A/B测试**: 版本对比测试
- **回滚机制**: 快速版本回滚

## 🧪 测试策略

### 性能测试
- **负载测试**: 模拟正常负载
- **压力测试**: 模拟峰值负载
- **稳定性测试**: 长期稳定性测试
- **容量规划**: 资源容量评估

### 安全测试
- **渗透测试**: 模拟攻击测试
- **漏洞扫描**: 自动化漏洞检测
- **安全审计**: 代码安全审计
- **合规检查**: 安全标准合规性检查