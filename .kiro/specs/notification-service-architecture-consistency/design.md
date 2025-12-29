# 通知服务架构一致性设计文档

## 概述

本设计文档定义了通知服务架构一致性的解决方案，旨在建立统一的概念模型、命名规范和分层架构。通过系统性的重构，消除现有的类型冲突、命名不一致和架构混乱问题。

## 架构

### 分层架构设计

```
┌─────────────────────────────────────────────────────────────┐
│                    Handler Layer                            │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │ Notifications   │  │ Channels        │  │ Templates    │ │
│  │ Handler         │  │ Handler         │  │ Handler      │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Service Layer                            │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │ Notification    │  │ Channel         │  │ Template     │ │
│  │ Service         │  │ Service         │  │ Service      │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Store Layer                              │
│  ┌─────────────────┐  ┌─────────────────┐  ┌──────────────┐ │
│  │ Notification    │  │ Channel         │  │ Template     │ │
│  │ Store           │  │ Store           │  │ Store        │ │
│  └─────────────────┘  └─────────────────┘  └──────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Database Layer                           │
│           PostgreSQL with Notification Tables              │
└─────────────────────────────────────────────────────────────┘
```

### 核心概念模型

#### 1. 通知渠道概念层次
```
NotificationChannel (枚举)
├── Email
├── Sms  
├── Push
├── WebSocket
├── InApp
└── Webhook

ChannelConfig (实体)
├── id: Uuid
├── channel: NotificationChannel
├── name: String
├── configuration: JsonValue
└── ... (其他配置字段)
```

#### 2. 统一命名规范
- **资源名称**: 使用单数形式 (channel, notification, template)
- **CRUD操作**: create_*, get_*, list_*, update_*, delete_*
- **特殊操作**: 使用描述性动词 (check_*_health, validate_*, render_*)
- **请求类型**: Create*Request, Update*Request
- **响应类型**: *Response, *Stats, *Metrics

## 组件和接口

### 1. Handler层接口规范

```rust
// 统一的Handler方法签名模式
pub async fn create_channel_config(
    State(state): State<AppState>,
    Json(request): Json<CreateChannelConfigRequest>,
) -> Result<Json<Value>, StatusCode>

pub async fn get_channel_config(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Value>, StatusCode>

pub async fn list_channel_configs(
    State(state): State<AppState>,
    Query(filter): Query<ChannelConfigFilter>,
) -> Result<Json<Value>, StatusCode>
```

### 2. Service层接口规范

```rust
// 统一的Service方法签名模式
impl ChannelService {
    pub async fn create_channel_config(&self, request: CreateChannelConfigRequest) -> Result<ChannelConfig>
    pub async fn get_channel_config(&self, id: Uuid) -> Result<Option<ChannelConfig>>
    pub async fn list_channel_configs(&self, filter: &ChannelConfigFilter) -> Result<Vec<ChannelConfig>>
    pub async fn update_channel_config(&self, id: Uuid, request: UpdateChannelConfigRequest) -> Result<bool>
    pub async fn delete_channel_config(&self, id: Uuid) -> Result<bool>
    
    // 特殊操作
    pub async fn check_channel_health(&self, channel: NotificationChannel) -> Result<ChannelHealthCheck>
    pub async fn validate_channel_config(&self, config: &ChannelConfig) -> Result<ValidationResult>
}
```

### 3. Store层接口规范

```rust
// 统一的Store方法签名模式
impl ChannelStore {
    pub async fn create(&self, config: &ChannelConfig) -> Result<()>
    pub async fn get_by_id(&self, id: Uuid) -> Result<Option<ChannelConfig>>
    pub async fn list(&self, filter: &ChannelConfigFilter) -> Result<Vec<ChannelConfig>>
    pub async fn update(&self, id: Uuid, request: &UpdateChannelConfigRequest) -> Result<bool>
    pub async fn delete(&self, id: Uuid) -> Result<bool>
    
    // 查询方法
    pub async fn get_by_channel_type(&self, channel: NotificationChannel) -> Result<Vec<ChannelConfig>>
    pub async fn get_active_configs(&self) -> Result<Vec<ChannelConfig>>
}
```

## 数据模型

### 1. 核心实体模型

```rust
// 渠道配置实体
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ChannelConfig {
    pub id: Uuid,
    pub channel: NotificationChannel,
    pub name: String,
    pub is_enabled: bool,
    pub priority: i32,
    pub configuration: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// 渠道类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "notification_channel", rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
    WebSocket,
    InApp,
    Webhook,
}
```

### 2. 请求/响应模型

```rust
// 创建请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChannelConfigRequest {
    pub channel: NotificationChannel,
    pub name: String,
    pub priority: Option<i32>,
    pub configuration: serde_json::Value,
}

// 更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChannelConfigRequest {
    pub name: Option<String>,
    pub is_enabled: Option<bool>,
    pub priority: Option<i32>,
    pub configuration: Option<serde_json::Value>,
}

// 过滤器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfigFilter {
    pub channel: Option<NotificationChannel>,
    pub is_enabled: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}
```

### 3. 状态和指标模型

```rust
// 渠道健康检查
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelHealthCheck {
    pub channel: NotificationChannel,
    pub is_healthy: bool,
    pub response_time_ms: u64,
    pub error_message: Option<String>,
    pub checked_at: DateTime<Utc>,
}

// 渠道指标 (重命名以避免冲突)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfigMetrics {
    pub channel: NotificationChannel,
    pub total_sent: i64,
    pub successful: i64,
    pub failed: i64,
    pub success_rate: f64,
    pub average_response_time: f64,
}
```

## 正确性属性

*属性是一个特征或行为，应该在系统的所有有效执行中保持为真——本质上是关于系统应该做什么的正式声明。属性作为人类可读规范和机器可验证正确性保证之间的桥梁。*

### 属性反思

在编写正确性属性之前，我需要识别和消除冗余：

**冗余分析**：
- 属性1.1和1.2都涉及命名一致性，可以合并为一个综合的命名一致性属性
- 属性3.1-3.5都是关于CRUD方法命名模式，可以合并为一个方法命名规范属性
- 属性5.1和5.3都涉及跨层数据一致性，可以合并
- 属性6.1-6.3都是关于错误处理一致性，可以合并

**合并后的属性**：

#### 属性 1: 命名一致性
*对于任何*概念或操作，在Handler、Service、Store三层中应使用一致的术语和类型名称
**验证: 需求 1.1, 1.2, 1.4**

#### 属性 2: 分层架构完整性  
*对于任何*业务操作，Handler层应只调用Service层方法，Service层应只调用Store层方法，不允许跨层直接访问
**验证: 需求 2.1, 2.4**

#### 属性 3: CRUD方法命名规范
*对于任何*资源的CRUD操作，方法名应遵循统一模式：create_*, get_*, list_*, update_*, delete_*
**验证: 需求 3.1, 3.2, 3.3, 3.4, 3.5**

#### 属性 4: 类型系统完整性
*对于任何*模型类型，应避免同名冲突，枚举应有完整定义，编译应无歧义错误
**验证: 需求 4.1, 4.2, 4.4**

#### 属性 5: 跨层数据一致性
*对于任何*数据传递，字段名称、类型和验证规则在所有层应保持一致
**验证: 需求 5.1, 5.2, 5.3, 5.5**

#### 属性 6: 错误处理一致性
*对于任何*错误情况，错误类型、消息格式、日志记录和响应结构应在所有层保持一致
**验证: 需求 6.1, 6.2, 6.3, 6.4**

#### 属性 7: API接口规范性
*对于任何*API端点，应遵循RESTful原则，使用一致的JSON结构和标准HTTP状态码
**验证: 需求 7.1, 7.2, 7.3**

#### 属性 8: 测试覆盖完整性
*对于任何*Service方法，应有对应的单元测试，集成测试应覆盖完整的请求-响应流程
**验证: 需求 8.1, 8.2, 8.3**

## 错误处理

### 统一错误类型定义

```rust
#[derive(Debug, thiserror::Error)]
pub enum NotificationError {
    #[error("Channel config not found: {id}")]
    ChannelConfigNotFound { id: Uuid },
    
    #[error("Invalid channel configuration: {reason}")]
    InvalidChannelConfig { reason: String },
    
    #[error("Channel health check failed: {channel:?}")]
    ChannelHealthCheckFailed { channel: NotificationChannel },
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}
```

### 错误响应格式

```rust
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: ErrorDetail,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: String,
    pub message: String,
    pub details: Option<serde_json::Value>,
}
```

## 测试策略

### 双重测试方法

本设计采用单元测试和属性测试相结合的方法：

**单元测试**：
- 验证具体的业务逻辑实现
- 测试错误处理路径
- 验证边界条件

**属性测试**：
- 验证架构一致性属性
- 测试命名规范遵循情况
- 验证跨层数据一致性

### 测试框架选择

- **单元测试**: 使用Rust标准测试框架
- **属性测试**: 使用QuickCheck或Proptest
- **集成测试**: 使用axum-test进行HTTP API测试
- **架构测试**: 使用自定义的静态分析工具

### 测试配置要求

- 每个属性测试运行最少100次迭代
- 每个属性测试必须标注对应的设计文档属性编号
- 测试标注格式：`**Feature: notification-service-architecture-consistency, Property {number}: {property_text}**`

## 实施计划

### 阶段1: 概念模型统一
1. 确定统一的资源命名（ChannelConfig vs Channel）
2. 重构所有相关类型定义
3. 更新导入和导出声明

### 阶段2: 接口规范化
1. 统一Handler层方法签名
2. 统一Service层方法命名
3. 统一Store层CRUD接口

### 阶段3: 类型冲突解决
1. 解决ChannelMetrics等同名冲突
2. 补充缺失的枚举值定义
3. 修复编译错误

### 阶段4: 测试完善
1. 编写架构一致性属性测试
2. 补充单元测试覆盖
3. 添加集成测试

### 阶段5: 文档同步
1. 更新API文档
2. 生成架构规范文档
3. 建立持续集成检查