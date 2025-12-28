# 通知服务 (notification) - 架构设计

## 📋 服务概述

### 服务名称
通知服务 (Notification Service)

### 服务端口
8086

### 服务职责
- 多渠道通知发送 (邮件、短信、推送、WebSocket)
- 模板管理 (通知模板、变量替换)
- 订阅管理 (用户订阅设置、频率控制)
- 通知历史管理
- 实时推送服务

## 🏗️ 服务架构

### 内部架构图
```
services/notification/
│
├── src/
│   │
│   ├── main.rs                 # 服务入口，启动HTTP服务器和WebSocket服务
│   │
│   ├── state.rs                # 应用状态管理，持有所有组件的Arc引用
│   │
│   ├── config/                 # 配置管理
│   │   ├── mod.rs              # 配置结构体定义
│   │   └── settings.rs         # 配置加载逻辑
│   │
│   ├── handlers/               # HTTP接口层
│   │   ├── mod.rs              # 路由注册
│   │   ├── notifications.rs    # 通知管理接口
│   │   ├── templates.rs        # 模板管理接口
│   │   ├── subscriptions.rs    # 订阅管理接口
│   │   ├── channels.rs         # 通知渠道接口
│   │   └── websocket.rs        # WebSocket接口
│   │
│   ├── services/               # 业务逻辑层
│   │   ├── mod.rs              # 服务管理
│   │   ├── notification_service.rs # 通知核心服务
│   │   ├── template_service.rs     # 模板服务
│   │   ├── channel_service.rs      # 渠道服务
│   │   ├── delivery_service.rs     # 投递服务
│   │   └── websocket_service.rs    # WebSocket服务
│   │
│   ├── channels/               # 通知渠道实现
│   │   ├── mod.rs              # 渠道管理
│   │   ├── email.rs            # 邮件渠道
│   │   ├── sms.rs              # 短信渠道
│   │   ├── push.rs             # 推送渠道
│   │   ├── websocket.rs        # WebSocket渠道
│   │   └── webhook.rs          # Webhook渠道
│   │
│   ├── templates/              # 模板引擎
│   │   ├── mod.rs              # 模板管理
│   │   ├── engine.rs           # 模板引擎
│   │   ├── variable_resolver.rs # 变量解析
│   │   └── renderer.rs         # 模板渲染
│   │
│   ├── storage/                # 数据存储层
│   │   ├── mod.rs              # 存储接口
│   │   ├── postgres_store.rs   # PostgreSQL存储
│   │   └── redis_cache.rs      # Redis缓存
│   │
│   ├── models/                 # 数据模型
│   │   ├── mod.rs              # 模型定义
│   │   ├── notification.rs     # 通知模型
│   │   ├── template.rs         # 模板模型
│   │   ├── subscription.rs     # 订阅模型
│   │   ├── channel.rs          # 渠道模型
│   │   └── delivery.rs         # 投递模型
│   │
│   └── utils/                  # 工具函数
│       ├── mod.rs
│       ├── rate_limiter.rs     # 频率限制器
│       ├── priority_queue.rs   # 优先级队列
│       └── notification_filter.rs # 通知过滤器
│
└── Cargo.toml                  # 依赖声明
```

## 🔄 数据流向

### 通知发送流程
```
HTTP请求 (创建通知)
    ↓
handlers/notifications.rs
    ↓
services/notification_service.rs
    ↓
templates/engine.rs (模板渲染)
    ↓
channels/ (多渠道投递)
    ↓
storage/postgres_store.rs (记录投递状态)
    ↓
返回发送结果
```

### 实时推送流程
```
WebSocket连接
    ↓
handlers/websocket.rs
    ↓
services/websocket_service.rs
    ↓
WebSocket消息广播
    ↓
前端客户端接收
```

## 📡 API接口设计

### 通知管理
```http
POST /api/v1/notifications      # 创建通知
GET  /api/v1/notifications      # 查询通知列表
GET  /api/v1/notifications/{id} # 查询通知详情
PUT  /api/v1/notifications/{id} # 更新通知
DELETE /api/v1/notifications/{id} # 删除通知
POST /api/v1/notifications/{id}/send # 发送通知
POST /api/v1/notifications/batch # 批量创建通知
GET  /api/v1/notifications/pending # 查询待发送通知
```

### 模板管理
```http
GET  /api/v1/templates          # 查询模板列表
POST /api/v1/templates          # 创建模板
GET  /api/v1/templates/{id}     # 查询模板详情
PUT  /api/v1/templates/{id}     # 更新模板
DELETE /api/v1/templates/{id}   # 删除模板
GET  /api/v1/templates/{name}/preview # 模板预览
POST /api/v1/templates/validate # 模板验证
```

### 订阅管理
```http
GET  /api/v1/subscriptions      # 查询订阅列表
PUT  /api/v1/subscriptions      # 更新订阅设置
GET  /api/v1/subscriptions/{user_id} # 查询用户订阅
POST /api/v1/subscriptions/{user_id}/unsubscribe # 取消订阅
POST /api/v1/subscriptions/{user_id}/resubscribe # 重新订阅
GET  /api/v1/subscriptions/types # 查询订阅类型
```

### 渠道管理
```http
GET  /api/v1/channels           # 查询渠道列表
GET  /api/v1/channels/{type}    # 查询渠道状态
PUT  /api/v1/channels/{type}    # 更新渠道配置
GET  /api/v1/channels/stats     # 查询渠道统计
POST /api/v1/channels/test      # 测试渠道
```

### WebSocket接口
```http
GET  /ws/notifications          # WebSocket连接
POST /api/v1/push/{user_id}     # 推送消息到用户
POST /api/v1/broadcast          # 广播消息
GET  /api/v1/connection-status  # 连接状态
```

## 🗄️ 数据模型

### 核心数据结构
```rust
// 通知模型
pub struct Notification {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub content: String,
    pub notification_type: NotificationType,
    pub channels: Vec<NotificationChannel>,
    pub priority: Priority,
    pub status: NotificationStatus,
    pub variables: HashMap<String, serde_json::Value>,
    pub scheduled_at: Option<i64>,
    pub sent_at: Option<i64>,
    pub read_at: Option<i64>,
    pub delivery_attempts: u32,
    pub last_delivery_attempt: Option<i64>,
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum NotificationType {
    // 交易通知
    OrderFilled,        // 订单成交
    OrderCanceled,      // 订单取消
    PositionOpened,     // 开仓通知
    PositionClosed,     // 平仓通知
    MarginCall,         // 追加保证金
    Liquidation,        // 强制平仓
    
    // 风险通知
    RiskWarning,        // 风险预警
    LimitBreached,      // 限额突破
    HighRiskAlert,      // 高风险警告
    
    // 策略通知
    StrategySignal,     // 策略信号
    StrategyError,      // 策略错误
    BacktestComplete,   // 回测完成
    StrategyPerformance, // 策略表现
    
    // 系统通知
    SystemMaintenance,  // 系统维护
    SecurityAlert,      // 安全警报
    AccountUpdate,      // 账户更新
    Deposit,            // 充值通知
    Withdrawal,         // 提现通知
}

pub enum NotificationChannel {
    Email,              // 邮件通知
    SMS,                // 短信通知
    Push,               // 移动端推送
    WebSocket,          // WebSocket推送
    Webhook,            // Webhook回调
}

pub enum Priority {
    Low,                // 低优先级
    Normal,             // 普通优先级
    High,               // 高优先级
    Critical,           // 紧急优先级
}

pub enum NotificationStatus {
    Draft,              // 草稿
    Scheduled,          // 已计划
    Pending,            // 待发送
    Sending,            // 发送中
    Sent,               // 已发送
    Failed,             // 发送失败
    Read,               // 已读取
}

// 模板模型
pub struct NotificationTemplate {
    pub id: String,
    pub name: String,
    pub title_template: String,     // 标题模板
    pub content_template: String,   // 内容模板
    pub channel: NotificationChannel, // 适用渠道
    pub notification_type: NotificationType, // 通知类型
    pub language: String,           // 语言
    pub variables: Vec<String>,     // 可用变量
    pub is_active: bool,            // 是否启用
    pub created_at: i64,
    pub updated_at: i64,
}

// 订阅模型
pub struct Subscription {
    pub user_id: String,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub is_subscribed: bool,
    pub frequency: NotificationFrequency, // 发送频率
    pub timezone: String,
    pub created_at: i64,
    pub updated_at: i64,
}

pub enum NotificationFrequency {
    Immediate,          // 立即发送
    Daily,              // 每日汇总
    Weekly,             // 每周汇总
    Never,              // 不发送
}

// 渠道配置模型
pub struct ChannelConfig {
    pub channel_type: NotificationChannel,
    pub is_enabled: bool,
    pub rate_limit: u32,           // 频率限制 (每小时)
    pub retry_attempts: u32,       // 重试次数
    pub config: serde_json::Value, // 渠道特定配置
    pub updated_at: i64,
}
```

## 🔧 技术实现要点

### 多渠道支持
- **邮件渠道**: SMTP邮件发送
- **短信渠道**: 第三方短信服务
- **推送渠道**: 移动端推送服务
- **WebSocket**: 实时消息推送
- **Webhook**: 自定义回调

### 模板引擎
- **变量替换**: 支持动态变量替换
- **多语言支持**: 国际化模板
- **模板版本**: 模板版本控制
- **实时预览**: 模板预览功能

### 频率控制
- **令牌桶算法**: 平滑流量控制
- **用户级限制**: 按用户限制发送频率
- **渠道级限制**: 按渠道限制发送频率
- **智能调度**: 优先级调度

### 性能优化
- **异步处理**: 异步通知发送
- **批量处理**: 批量通知处理
- **缓存策略**: Redis缓存模板和配置
- **连接池**: 邮件/短信连接池

## 📊 监控指标

### 性能指标
- 通知发送延迟
- 消息队列长度
- 渠道成功率
- 系统吞吐量

### 业务指标
- 通知送达率
- 用户阅读率
- 渠道使用率
- 频率限制触发次数

## 🔐 安全措施

- **认证授权**: JWT认证 + RBAC权限控制
- **内容过滤**: 恶意内容过滤
- **频率限制**: 防止垃圾通知
- **审计日志**: 完整的通知发送审计日志

## 🚀 部署配置

### 环境变量
```
NOTIFICATION_PORT=8086
DATABASE_URL=postgresql://user:pass@localhost/notification
REDIS_URL=redis://localhost:6379
SMTP_HOST=smtp.example.com
SMTP_PORT=587
SMTP_USER=your_smtp_user
SMTP_PASSWORD=your_smtp_password
SMS_API_KEY=your_sms_api_key
PUSH_API_KEY=your_push_api_key
NOTIFICATION_RATE_LIMIT=100
MAX_RETRY_ATTEMPTS=3
WEBSOCKET_MAX_CONNECTIONS=10000
EMAIL_TEMPLATE_PATH=/templates/email
PUSH_CERT_PATH=/certs/push.p12
```

### Docker配置
- 多阶段构建
- 资源限制
- 健康检查

## 🧪 测试策略

### 单元测试
- 模板渲染测试
- 频率限制测试
- 数据模型测试

### 集成测试
- 端到端通知发送测试
- 多渠道投递测试
- WebSocket连接测试

### 压力测试
- 高并发通知发送测试
- 大量WebSocket连接测试
- 系统稳定性测试