# 用户管理服务 (user-management) - 架构设计

## 📋 服务概述

### 服务名称
用户管理服务 (User Management Service)

### 服务端口
8081

### 服务职责
- 用户认证 (注册、登录、登出)
- 权限管理 (角色、权限控制)
- 用户资料管理
- KYC验证
- 会话管理

## 🏗️ 服务架构

### 内部架构图
```
services/user-management/
│
├── src/
│   │
│   ├── main.rs                 # 服务入口，启动HTTP服务器
│   │
│   ├── state.rs                # 应用状态管理，持有所有组件的Arc引用
│   │
│   ├── config/                 # 配置管理
│   │   ├── mod.rs              # 配置结构体定义
│   │   └── settings.rs         # 配置加载逻辑
│   │
│   ├── handlers/               # HTTP接口层
│   │   ├── mod.rs              # 路由注册
│   │   ├── auth.rs             # 认证接口
│   │   ├── users.rs            # 用户管理接口
│   │   ├── permissions.rs      # 权限管理接口
│   │   ├── kyc.rs              # KYC验证接口
│   │   └── sessions.rs         # 会话管理接口
│   │
│   ├── auth/                   # 认证模块
│   │   ├── mod.rs              # 认证管理
│   │   ├── jwt.rs              # JWT令牌管理
│   │   ├── password.rs         # 密码处理
│   │   └── oauth.rs            # OAuth集成
│   │
│   ├── rbac/                   # 权限控制
│   │   ├── mod.rs              # RBAC管理
│   │   ├── roles.rs            # 角色管理
│   │   ├── permissions.rs      # 权限管理
│   │   └── access_control.rs   # 访问控制
│   │
│   ├── storage/                # 数据存储层
│   │   ├── mod.rs              # 存储接口
│   │   ├── postgres_store.rs   # PostgreSQL存储
│   │   └── redis_cache.rs      # Redis缓存
│   │
│   ├── models/                 # 数据模型
│   │   ├── mod.rs              # 模型定义
│   │   ├── user.rs             # 用户模型
│   │   ├── role.rs             # 角色模型
│   │   ├── permission.rs       # 权限模型
│   │   ├── session.rs          # 会话模型
│   │   └── kyc.rs              # KYC模型
│   │
│   ├── services/               # 业务逻辑层
│   │   ├── mod.rs              # 服务管理
│   │   ├── user_service.rs     # 用户服务
│   │   ├── auth_service.rs     # 认证服务
│   │   ├── permission_service.rs # 权限服务
│   │   └── kyc_service.rs      # KYC服务
│   │
│   └── utils/                  # 工具函数
│       ├── mod.rs
│       ├── crypto.rs           # 加密工具
│       ├── validation.rs       # 参数验证
│       └── email.rs            # 邮件工具
│
└── Cargo.toml                  # 依赖声明
```

## 🔄 数据流向

### 用户认证流程
```
HTTP请求 (登录)
    ↓
handlers/auth.rs
    ↓
services/auth_service.rs
    ↓
auth/password.rs (密码验证)
    ↓
storage/postgres_store.rs (用户查询)
    ↓
auth/jwt.rs (JWT令牌生成)
    ↓
返回认证结果和令牌
```

### 权限验证流程
```
HTTP请求 (带JWT)
    ↓
中间件 (JWT验证)
    ↓
auth/jwt.rs (令牌验证)
    ↓
storage/redis_cache.rs (缓存验证)
    ↓
rbac/access_control.rs (权限检查)
    ↓
目标接口处理
```

## 📡 API接口设计

### 认证接口
```http
POST /api/v1/auth/register      # 用户注册
POST /api/v1/auth/login         # 用户登录
POST /api/v1/auth/logout        # 用户登出
POST /api/v1/auth/refresh       # 刷新令牌
POST /api/v1/auth/forgot-password # 忘记密码
POST /api/v1/auth/reset-password # 重置密码
GET  /api/v1/auth/verify-email  # 验证邮箱
```

### 用户管理
```http
GET  /api/v1/users/profile     # 获取用户信息
PUT  /api/v1/users/profile     # 更新用户信息
GET  /api/v1/users/{id}        # 获取用户详情
PUT  /api/v1/users/{id}        # 更新用户信息
DELETE /api/v1/users/{id}      # 删除用户
GET  /api/v1/users             # 查询用户列表
```

### 权限管理
```http
GET  /api/v1/users/permissions # 获取用户权限
GET  /api/v1/permissions       # 查询权限列表
GET  /api/v1/roles             # 查询角色列表
GET  /api/v1/users/{id}/roles  # 获取用户角色
PUT  /api/v1/users/{id}/roles  # 设置用户角色
```

### KYC验证
```http
POST /api/v1/kyc/submit         # 提交KYC信息
GET  /api/v1/kyc/status        # 查询KYC状态
GET  /api/v1/kyc/documents     # 获取KYC文档
PUT  /api/v1/kyc/documents     # 更新KYC文档
GET  /api/v1/kyc/users         # 查询KYC用户列表
```

## 🗄️ 数据模型

### 核心数据结构
```rust
// 用户模型
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub phone: Option<String>,
    pub password_hash: String,
    pub status: UserStatus,       // ACTIVE/SUSPENDED/FROZEN
    pub role: UserRole,           // ADMIN/TRADER/VIEWER
    pub created_at: i64,
    pub updated_at: i64,
    pub last_login_at: Option<i64>,
    pub email_verified: bool,
    pub phone_verified: bool,
    pub profile: UserProfile,
}

// 用户资料模型
pub struct UserProfile {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: Option<String>,
    pub country: String,
    pub address: String,
    pub avatar_url: Option<String>,
    pub timezone: String,
}

// 角色模型
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: String,
    pub permissions: Vec<Permission>,
    pub created_at: i64,
}

// 权限模型
pub enum Permission {
    // 交易权限
    TRADE_SPOT,         // 现货交易
    TRADE_MARGIN,       // 保证金交易
    TRADE_FUTURES,      // 期货交易
    
    // 策略权限
    STRATEGY_CREATE,    // 创建策略
    STRATEGY_MANAGE,    // 管理策略
    STRATEGY_BACKTEST,  // 策略回测
    
    // 数据权限
    DATA_VIEW,          // 查看数据
    DATA_EXPORT,        // 导出数据
    
    // 管理权限
    USER_MANAGE,        // 用户管理
    SYSTEM_CONFIG,      // 系统配置
}

// 会话模型
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub token: String,
    pub expires_at: i64,
    pub created_at: i64,
    pub last_accessed_at: i64,
    pub ip_address: String,
    pub user_agent: String,
}

// KYC信息模型
pub struct KycInfo {
    pub id: String,
    pub user_id: String,
    pub status: KycStatus,        // PENDING/APPROVED/REJECTED
    pub verification_level: KycLevel, // LEVEL_1/LEVEL_2/LEVEL_3
    pub personal_info: PersonalInfo,
    pub documents: Vec<KycDocument>,
    pub submitted_at: i64,
    pub reviewed_at: Option<i64>,
    pub reviewed_by: Option<String>,
}

pub struct PersonalInfo {
    pub full_name: String,
    pub date_of_birth: String,
    pub nationality: String,
    pub id_number: String,
    pub address: Address,
}

pub struct KycDocument {
    pub id: String,
    pub document_type: DocumentType, // ID_CARD/PASSPORT/UTILITY_BILL
    pub document_url: String,
    pub status: DocumentStatus,     // PENDING/APPROVED/REJECTED
    pub uploaded_at: i64,
}
```

## 🔧 技术实现要点

### 认证系统
- **JWT令牌**: 安全的JWT令牌生成和验证
- **令牌刷新**: 支持令牌自动刷新机制
- **多因子认证**: 可扩展的MFA支持
- **会话管理**: 安全的会话管理

### 权限控制
- **RBAC模型**: 基于角色的访问控制
- **权限继承**: 角色权限继承机制
- **动态权限**: 运行时权限检查
- **权限缓存**: Redis缓存权限信息

### 数据安全
- **密码加密**: BCrypt密码加密
- **数据脱敏**: 敏感信息脱敏处理
- **访问日志**: 完整的访问日志记录
- **审计追踪**: 用户操作审计

### 性能优化
- **缓存策略**: Redis缓存用户信息和权限
- **数据库索引**: 优化数据库查询性能
- **异步处理**: 邮件发送等异步操作
- **连接池**: 数据库连接池管理

## 📊 监控指标

### 性能指标
- 认证请求延迟
- 数据库查询耗时
- 缓存命中率
- 系统吞吐量

### 业务指标
- 注册转化率
- 登录成功率
- KYC通过率
- 用户活跃度

## 🔐 安全措施

- **认证安全**: 密码强度验证、防暴力破解
- **令牌安全**: JWT令牌安全配置
- **数据加密**: 敏感数据加密存储
- **访问控制**: 严格的权限控制
- **日志审计**: 完整的安全审计日志
- **防爬虫**: 防止自动化攻击

## 🚀 部署配置

### 环境变量
```
USER_MANAGEMENT_PORT=8081
DATABASE_URL=postgresql://user:pass@localhost/users
REDIS_URL=redis://localhost:6379
JWT_SECRET_KEY=your_secret_key
JWT_EXPIRY_HOURS=24
REFRESH_TOKEN_EXPIRY_DAYS=30
BCRYPT_COST=12
EMAIL_SMTP_HOST=smtp.example.com
EMAIL_SMTP_PORT=587
EMAIL_SMTP_USER=your_user
EMAIL_SMTP_PASSWORD=your_password
RECAPTCHA_SITE_KEY=your_recaptcha_site_key
RECAPTCHA_SECRET_KEY=your_recaptcha_secret_key
```

### Docker配置
- 多阶段构建
- 最小化权限
- 资源限制

## 🧪 测试策略

### 单元测试
- 认证逻辑测试
- 权限验证测试
- 数据模型测试

### 集成测试
- 端到端认证流程测试
- 权限控制测试
- KYC流程测试

### 安全测试
- 认证绕过测试
- 权限提升测试
- 数据泄露测试