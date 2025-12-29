# 后端API完整实现计划

**基于前端代码分析的API需求**

## 🎯 前端需要的核心API接口

### 1. 市场数据服务 (8081)
```typescript
// 前端调用的接口
GET /api/tickers/{symbol}           // 获取价格数据
GET /api/klines/{symbol}            // 获取K线数据  
GET /api/orderbook/{symbol}         // 获取订单簿
GET /health                         // 健康检查
WebSocket /ws                       // 实时数据流
```

### 2. 交易引擎服务 (8082)
```typescript
// 前端调用的接口
POST /api/v1/orders                 // 下单
GET  /api/v1/orders                 // 订单历史
GET  /api/v1/positions              // 持仓信息
GET  /api/v1/account                // 账户信息
DELETE /api/v1/orders/{id}          // 取消订单
GET /health                         // 健康检查
```

### 3. 策略引擎服务 (8083)
```typescript
// 前端调用的接口
GET  /api/strategies                // 策略列表
POST /api/strategies                // 创建策略
GET  /api/signals                   // 交易信号
GET  /api/indicators                // 技术指标
POST /api/backtests                 // 回测
GET /health                         // 健康检查
```

### 4. 用户管理服务 (8084)
```typescript
// 前端调用的接口
POST /api/auth/login                // 登录
GET  /api/users/{id}                // 用户信息
GET  /api/permissions/{id}          // 用户权限
GET /health                         // 健康检查
```

### 5. 风险管理服务 (8085)
```typescript
// 前端调用的接口
GET  /api/risk/{accountId}          // 风险评估
POST /api/limits/{accountId}        // 设置限额
GET  /api/alerts                    // 风险预警
GET /health                         // 健康检查
```

### 6. 通知服务 (8086)
```typescript
// 前端调用的接口
GET  /api/notifications/{userId}    // 获取通知
POST /api/notifications             // 发送通知
PUT  /api/notifications/{id}/read   // 标记已读
GET /health                         // 健康检查
```

### 7. 分析服务 (8087)
```typescript
// 前端调用的接口
GET  /api/v1/metrics                // 系统指标
GET  /api/v1/reports/performance    // 性能报表
POST /api/v1/reports/custom         // 自定义报表
GET  /api/v1/export/{format}        // 数据导出
GET /health                         // 健康检查
```

### 8. AI服务 (8088)
```typescript
// 前端调用的接口
POST /api/v1/predict/price          // 价格预测
POST /api/v1/arbitrage/opportunities // 套利机会
POST /api/v1/signals/generate       // 生成信号
GET  /api/v1/models/status          // 模型状态
GET /health                         // 健康检查
```

## 🚀 实现优先级

### Phase 1: 核心交易功能 (立即实现)
1. **市场数据服务** - 实时价格、K线、订单簿
2. **交易引擎服务** - 下单、持仓、账户管理
3. **用户管理服务** - 登录认证、权限管理

### Phase 2: 高级功能 (本周完成)
4. **策略引擎服务** - 策略管理、信号生成
5. **风险管理服务** - 风险评估、预警系统
6. **通知服务** - 消息推送、模板管理

### Phase 3: 智能分析 (下周完成)
7. **分析服务** - 数据分析、报表生成
8. **AI服务** - 价格预测、套利发现

## 📊 数据库集成

每个服务需要连接到对应的数据库表：

### 市场数据服务 → Redis + ClickHouse
- Redis: 实时价格缓存
- ClickHouse: 历史K线数据

### 交易引擎服务 → PostgreSQL + Redis
- PostgreSQL: 订单、持仓、账户数据
- Redis: 实时价格缓存

### 策略引擎服务 → PostgreSQL + Redis
- PostgreSQL: 策略配置、回测结果
- Redis: 技术指标缓存

### 其他服务 → PostgreSQL
- 用户管理: users, permissions表
- 风险管理: risk_assessments, risk_limits表
- 通知服务: notifications, templates表
- 分析服务: performance_reports, statistics表
- AI服务: price_predictions, arbitrage_opportunities表

## 🔧 实现方式

1. **更新现有服务代码** - 添加缺失的API端点
2. **实现数据库操作** - 连接到实际的PostgreSQL表
3. **集成Redis缓存** - 提高查询性能
4. **添加WebSocket支持** - 实时数据推送
5. **完善错误处理** - 统一的错误响应格式
6. **添加API文档** - OpenAPI规范

## 📋 下一步行动

1. 立即开始实现市场数据服务的完整API
2. 然后实现交易引擎服务的核心功能
3. 逐步完善其他服务的API接口
4. 测试前后端集成
5. 优化性能和错误处理