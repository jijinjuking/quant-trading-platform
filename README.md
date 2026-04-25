# 量化交易平台 (Quant Trading Platform)

一个基于 Rust + Vue3 的全栈量化交易系统，支持实时行情采集、策略执行、风险管理和数据分析。

## 📊 系统状态

**整体完成度: 55%**

| 模块 | 完成度 | 状态 |
|------|--------|------|
| Gateway 网关 | 70% | ✅ 可用 |
| Trading Engine 交易引擎 | 65% | ⚠️ 框架完整，持久化待补 |
| Strategy Engine 策略引擎 | 55% | ⚠️ 基础策略可用，回测未实现 |
| Market Data 行情服务 | 50% | ⚠️ 币安采集完整，多交易所待支持 |
| User Management 用户管理 | 45% | 🔴 框架完整，业务逻辑缺失 |
| Frontend 前端 | 60% | ⚠️ 页面框架完整，数据集成缺失 |
| Shared 共享层 | 70% | ✅ 类型定义完整 |
| Migrations 数据库 | 80% | ✅ 核心表设计完整 |

## 🏗️ 系统架构

```
┌─────────────────┐
│   Frontend      │ (Vue3 + Pinia)
│  TradingDash    │
└────────┬────────┘
         │ HTTP REST
┌────────▼────────────────────────┐
│    API Gateway (Axum)           │
│  Auth / Cache / Rate Limit      │
└────┬──────┬──────┬────────┬─────┘
     │      │      │        │
┌────▼──┐ ┌─▼──┐ ┌─▼──┐ ┌─▼──────┐
│ Trade │ │Strt│ │Mrkt│ │ User   │
│Engine │ │Eng │ │Data│ │Mgmt    │
└───────┘ └────┘ └────┘ └────────┘
     │      │      │
     └──────┼──────┘
            │ Kafka Events
     ┌──────▼──────────┐
     │  Other Services │
     │ Analytics/Risk  │
     │ Notification    │
     └─────────────────┘

数据层: PostgreSQL + Redis + Kafka + ClickHouse
```

## 🚀 快速开始

### 前置条件

- Rust 1.70+
- Node.js 18+
- Docker & Docker Compose
- PostgreSQL 15+
- Redis 7+
- Kafka 3.5+

### 环境配置

```bash
# 1. 复制环境变量配置
cp .env.template .env

# 2. 编辑 .env 文件（设置数据库、Redis、Kafka 等地址）
nano .env

# 3. 启动基础设施
docker-compose -f docker-compose.dev.yml up -d
```

### 运行后端服务

```bash
# 编译全部服务
cargo build --release

# 或单独运行某个服务
cargo run -p gateway
cargo run -p trading-engine
cargo run -p strategy-engine
cargo run -p market-data
```

### 运行前端

```bash
cd frontend
npm install
npm run dev
```

## 📁 项目结构

```
.
├── services/                    # 后端微服务
│   ├── gateway/                # API 网关
│   ├── trading-engine/         # 交易引擎
│   ├── strategy-engine/        # 策略引擎
│   ├── market-data/            # 行情数据采集
│   ├── user-management/        # 用户管理
│   ├── ai-service/             # AI 分析服务
│   ├── analytics/              # 数据分析
│   ├── risk-management/        # 风险管理
│   └── notification/           # 通知服务
├── frontend/                   # Vue3 前端项目
├── shared/                     # 共享库（类型、错误、事件）
├── migrations/                 # 数据库迁移文件
├── config/                     # 配置文件
├── docker-compose.*.yml        # Docker Compose 配置
└── docs/                       # 文档
```

## 🔑 核心功能

### ✅ 已实现

- [x] 实时行情数据采集 (Binance WebSocket)
- [x] 策略信号生成 (Grid Trading, Mean Reversion)
- [x] 订单下发与执行 (Binance REST API)
- [x] 基础风控检查 (单笔限额、头寸限制)
- [x] API 网关与认证 (JWT)
- [x] 缓存层 (Redis)
- [x] 事件流处理 (Kafka)
- [x] 用户认证框架

### 🚧 进行中

- [ ] Trading Engine 数据持久化 (订单、成交、持仓入库)
- [ ] Strategy Engine 回测框架
- [ ] Frontend 与 Backend 真实集成
- [ ] ClickHouse 行情数据存储
- [ ] 高级风控规则引擎

### ⏳ 未来规划

- [ ] 多交易所支持 (Coinbase, Kraken, OKX)
- [ ] AI 策略分析与推荐
- [ ] 期权与融资融券支持
- [ ] 高频交易 (HFT) 策略
- [ ] 性能优化与压力测试

## 🛠️ 技术栈

### 后端

- **语言**: Rust 1.70+
- **Web 框架**: Axum 0.7
- **异步运行时**: Tokio 1.35+
- **数据库**: PostgreSQL 15+
- **缓存**: Redis 7+
- **消息队列**: Kafka 3.5+
- **时序数据库**: ClickHouse 23+
- **API 文档**: OpenAPI 3.0

### 前端

- **框架**: Vue 3.3+
- **状态管理**: Pinia 2.1+
- **构建工具**: Vite 5+
- **UI 组件**: Element Plus
- **图表**: ECharts + Recharts
- **样式**: SCSS + Tailwind CSS

### 基础设施

- **容器化**: Docker + Docker Compose
- **编排**: Kubernetes (可选)
- **监控**: Prometheus + Grafana
- **反向代理**: Nginx
- **日志**: Tracing

## 📖 API 文档

API 文档位于 `api-docs/openapi.yaml`，可用以下方式查看：

```bash
# 启动 Swagger UI
docker run -p 8080:8080 -e SWAGGER_JSON=/app/openapi.yaml \
  -v $(pwd)/api-docs/openapi.yaml:/app/openapi.yaml \
  swaggerapi/swagger-ui
```

## 🧪 测试

```bash
# 运行单元测试
cargo test

# 运行集成测试
cargo test --test '*' -- --test-threads=1

# 运行前端测试
cd frontend && npm run test
```

## 📊 优先级任务

### Phase 1 (本周) - 数据持久化与基础集成

1. **Trading Engine 数据库实装** (3-4 天)
   - 订单 INSERT/UPDATE/SELECT
   - 成交数据持久化
   - 持仓更新逻辑
   - Git Branch: `feat/trading-engine-persistence`

2. **Frontend 与 Backend 集成** (2-3 天)
   - 登录流程真实化
   - 下单表单集成
   - 订单列表实时查询
   - Git Branch: `feat/frontend-backend-integration`

3. **User Management 业务逻辑** (2 天)
   - 用户注册入库
   - 登录认证实现
   - Token 生成和验证
   - Git Branch: `feat/user-auth`

### Phase 2 (下周) - 策略回测与数据存储

1. **Strategy Engine 回测框架** (5-6 天)
   - 历史数据加载
   - 策略模拟执行
   - 收益率计算
   - Git Branch: `feat/strategy-backtest`

2. **ClickHouse 行情存储** (2-3 天)
   - 行情数据持久化
   - 历史查询接口
   - Git Branch: `feat/clickhouse-integration`

### Phase 3 (之后) - Advanced Features

- Notification 邮件/推送实现
- Analytics 报表生成
- Risk Management 高级规则
- AI Service 完整实现

## 🔍 当前已知问题

1. **Trading Engine 无持久化**
   - 订单下发后数据丢失，无法查询历史订单
   - 影响: 无法追踪交易历史

2. **Strategy Engine 无回测**
   - 无法验证策略有效性，上线风险高
   - 影响: 高危策略可能无利润

3. **Frontend 数据流转**
   - 页面仅显示虚拟数据
   - 影响: 无法进行真实端到端测试

4. **Market Data 单交易所**
   - 仅支持币安，不支持其他交易所
   - 影响: 交易对选择受限

## 🤝 贡献指南

1. 创建 feature branch: `git checkout -b feat/your-feature`
2. 提交更改: `git commit -m "feat: description"`
3. 推送到远程: `git push origin feat/your-feature`
4. 提交 PR 到 main 分支

### Commit 规则

```
feat:    新功能
fix:     bug 修复
docs:    文档
style:   格式
refactor: 重构
perf:    性能优化
test:    测试
chore:   杂务
```

## 📝 文档

- [系统架构设计](dev-blueprint/SYSTEM_ARCHITECTURE_OVERVIEW.md)
- [API 文档](api-docs/README.md)
- [数据库设计](infrastructure/database-schema.sql)
- [交易引擎流程](docs/BINANCE_ORDER_FLOW_GUIDE.md)
- [E2E 测试指南](docs/E2E_TESTING_GUIDE.md)

## 📧 联系方式

- GitHub: https://github.com/jijinjuking/quant-trading-platform
- Issues: https://github.com/jijinjuking/quant-trading-platform/issues

## 📄 许可

MIT License

## 🙏 致谢

感谢所有为本项目贡献代码和想法的人。

---

**最后更新**: 2026-04-25  
**版本**: 0.1.0 (Beta)  
**作者**: Quant Trading Platform Team
