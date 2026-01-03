# 📋 用户管理服务开发任务书

> **任务类型**: 用户 CRUD + 认证
> **负责服务**: `user-management` (8084)
> **验收人**: Kiro（架构师）
> **必读文档**: `TEAM_DEVELOPMENT_GUIDE.md`
> **优先级**: 🟡 中（基础功能）

---

## 一、任务概述

实现用户管理服务，负责：
- 用户注册/登录
- JWT Token 生成
- 用户信息 CRUD
- API Key 管理（用于交易所）

```
Gateway → User Management (8084) → PostgreSQL
              ↓
         用户认证 / Token 生成
```

---

## 二、当前状态

```
services/user-management/src/
├── main.rs
├── lib.rs
├── state.rs               # ⚠️ 需要完善
├── bootstrap.rs           # ⚠️ 需要完善
│
├── application/
│   └── service/           # ❌ 空
│
├── domain/
│   ├── model/             # ❌ 空
│   ├── port/              # ❌ 空
│   └── service/           # ❌ 空
│
├── infrastructure/
│   └── repository/        # ❌ 空
│
└── interface/http/
    ├── routes.rs          # ⚠️ 骨架
    └── handlers/          # ❌ 空
```

---

## 三、待开发任务清单

### 任务 U1: 创建 Domain Model

**文件**: `services/user-management/src/domain/model/user.rs` (新建)

```rust
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用户实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户 ID
    pub id: Uuid,
    /// 用户名
    pub username: String,
    /// 邮箱
    pub email: String,
    /// 密码哈希
    pub password_hash: String,
    /// 角色
    pub role: UserRole,
    /// 状态
    pub status: UserStatus,
    /// 创建时间
    pub created_at: DateTime<Utc>,
    /// 更新时间
    pub updated_at: DateTime<Utc>,
}

/// 用户角色
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    /// 普通用户
    User,
    /// VIP 用户
    Vip,
    /// 管理员
    Admin,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::User => write!(f, "user"),
            UserRole::Vip => write!(f, "vip"),
            UserRole::Admin => write!(f, "admin"),
        }
    }
}

/// 用户状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserStatus {
    /// 活跃
    Active,
    /// 禁用
    Disabled,
    /// 待验证
    Pending,
}

/// 用户配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    /// 用户 ID
    pub user_id: Uuid,
    /// 昵称
    pub nickname: Option<String>,
    /// 头像 URL
    pub avatar_url: Option<String>,
    /// 时区
    pub timezone: String,
    /// 语言
    pub language: String,
}
```

**文件**: `services/user-management/src/domain/model/api_key.rs` (新建)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// API Key（用于交易所）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// Key ID
    pub id: Uuid,
    /// 用户 ID
    pub user_id: Uuid,
    /// 交易所名称
    pub exchange: String,
    /// API Key（加密存储）
    pub api_key_encrypted: String,
    /// Secret Key（加密存储）
    pub secret_key_encrypted: String,
    /// 备注
    pub label: String,
    /// 权限
    pub permissions: Vec<ApiKeyPermission>,
    /// 是否启用
    pub is_active: bool,
    /// 创建时间
    pub created_at: DateTime<Utc>,
}

/// API Key 权限
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApiKeyPermission {
    /// 只读
    ReadOnly,
    /// 现货交易
    SpotTrade,
    /// 合约交易
    FuturesTrade,
    /// 提现（危险）
    Withdraw,
}
```

**文件**: `services/user-management/src/domain/model/mod.rs`

```rust
pub mod user;
pub mod api_key;

pub use user::*;
pub use api_key::*;
```

---

### 任务 U2: 创建 Domain Port

**文件**: `services/user-management/src/domain/port/user_repository_port.rs` (新建)

```rust
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::domain::model::user::User;

/// 用户仓储端口
#[async_trait]
pub trait UserRepositoryPort: Send + Sync {
    /// 根据 ID 查找用户
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    
    /// 根据用户名查找用户
    async fn find_by_username(&self, username: &str) -> Result<Option<User>>;
    
    /// 根据邮箱查找用户
    async fn find_by_email(&self, email: &str) -> Result<Option<User>>;
    
    /// 保存用户
    async fn save(&self, user: &User) -> Result<()>;
    
    /// 更新用户
    async fn update(&self, user: &User) -> Result<()>;
    
    /// 删除用户
    async fn delete(&self, id: Uuid) -> Result<()>;
    
    /// 获取用户列表
    async fn list(&self, offset: i64, limit: i64) -> Result<Vec<User>>;
}
```

**文件**: `services/user-management/src/domain/port/api_key_repository_port.rs` (新建)

```rust
use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;

use crate::domain::model::api_key::ApiKey;

/// API Key 仓储端口
#[async_trait]
pub trait ApiKeyRepositoryPort: Send + Sync {
    /// 根据 ID 查找
    async fn find_by_id(&self, id: Uuid) -> Result<Option<ApiKey>>;
    
    /// 根据用户 ID 查找所有 Key
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<ApiKey>>;
    
    /// 保存
    async fn save(&self, api_key: &ApiKey) -> Result<()>;
    
    /// 更新
    async fn update(&self, api_key: &ApiKey) -> Result<()>;
    
    /// 删除
    async fn delete(&self, id: Uuid) -> Result<()>;
}
```

**文件**: `services/user-management/src/domain/port/mod.rs`

```rust
pub mod user_repository_port;
pub mod api_key_repository_port;

pub use user_repository_port::*;
pub use api_key_repository_port::*;
```

---

### 任务 U3: 创建 Domain Service

**文件**: `services/user-management/src/domain/service/password_service.rs` (新建)

```rust
use anyhow::{Context, Result};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

/// 密码服务
pub struct PasswordService;

impl PasswordService {
    /// 哈希密码
    pub fn hash_password(password: &str) -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        let hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("密码哈希失败: {}", e))?;
        
        Ok(hash.to_string())
    }

    /// 验证密码
    pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| anyhow::anyhow!("解析密码哈希失败: {}", e))?;
        
        let argon2 = Argon2::default();
        
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}
```

**文件**: `services/user-management/src/domain/service/jwt_service.rs` (新建)

```rust
use anyhow::Result;
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT Claims
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    /// 用户 ID
    pub sub: Uuid,
    /// 用户名
    pub username: String,
    /// 角色
    pub role: String,
    /// 过期时间
    pub exp: i64,
    /// 签发时间
    pub iat: i64,
}

/// JWT 服务
pub struct JwtService {
    secret: String,
    expiry_hours: i64,
}

impl JwtService {
    pub fn new(secret: String, expiry_hours: i64) -> Self {
        Self { secret, expiry_hours }
    }

    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
        let expiry_hours: i64 = std::env::var("JWT_EXPIRY_HOURS")
            .unwrap_or_else(|_| "24".to_string())
            .parse()
            .unwrap_or(24);
        Self::new(secret, expiry_hours)
    }

    /// 生成 Token
    pub fn generate_token(&self, user_id: Uuid, username: &str, role: &str) -> Result<String> {
        let now = Utc::now().timestamp();
        let exp = now + self.expiry_hours * 3600;

        let claims = Claims {
            sub: user_id,
            username: username.to_string(),
            role: role.to_string(),
            exp,
            iat: now,
        };

        let header = Header::new(Algorithm::HS256);
        let key = EncodingKey::from_secret(self.secret.as_bytes());

        encode(&header, &claims, &key)
            .map_err(|e| anyhow::anyhow!("生成 JWT 失败: {}", e))
    }

    /// 验证 Token
    pub fn verify_token(&self, token: &str) -> Result<Claims> {
        let key = DecodingKey::from_secret(self.secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        let token_data = decode::<Claims>(token, &key, &validation)
            .map_err(|e| anyhow::anyhow!("JWT 验证失败: {}", e))?;

        Ok(token_data.claims)
    }

    /// 刷新 Token
    pub fn refresh_token(&self, token: &str) -> Result<String> {
        let claims = self.verify_token(token)?;
        self.generate_token(claims.sub, &claims.username, &claims.role)
    }
}
```

**文件**: `services/user-management/src/domain/service/mod.rs`

```rust
pub mod password_service;
pub mod jwt_service;

pub use password_service::*;
pub use jwt_service::*;
```

---

### 任务 U4: 创建 Application Service

**文件**: `services/user-management/src/application/service/auth_service.rs` (新建)

```rust
use std::sync::Arc;
use anyhow::{Context, Result};
use uuid::Uuid;

use crate::domain::model::user::{User, UserRole, UserStatus};
use crate::domain::port::user_repository_port::UserRepositoryPort;
use crate::domain::service::jwt_service::JwtService;
use crate::domain::service::password_service::PasswordService;

/// 认证服务
pub struct AuthService<R: UserRepositoryPort> {
    user_repo: Arc<R>,
    jwt_service: JwtService,
}

impl<R: UserRepositoryPort> AuthService<R> {
    pub fn new(user_repo: Arc<R>) -> Self {
        Self {
            user_repo,
            jwt_service: JwtService::from_env(),
        }
    }

    /// 用户注册
    pub async fn register(
        &self,
        username: &str,
        email: &str,
        password: &str,
    ) -> Result<User> {
        // 检查用户名是否已存在
        if self.user_repo.find_by_username(username).await?.is_some() {
            anyhow::bail!("用户名已存在");
        }

        // 检查邮箱是否已存在
        if self.user_repo.find_by_email(email).await?.is_some() {
            anyhow::bail!("邮箱已被注册");
        }

        // 哈希密码
        let password_hash = PasswordService::hash_password(password)?;

        // 创建用户
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: email.to_string(),
            password_hash,
            role: UserRole::User,
            status: UserStatus::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        self.user_repo.save(&user).await?;

        Ok(user)
    }

    /// 用户登录
    pub async fn login(&self, username: &str, password: &str) -> Result<(User, String)> {
        // 查找用户
        let user = self.user_repo
            .find_by_username(username)
            .await?
            .ok_or_else(|| anyhow::anyhow!("用户不存在"))?;

        // 检查状态
        if user.status != UserStatus::Active {
            anyhow::bail!("用户已被禁用");
        }

        // 验证密码
        if !PasswordService::verify_password(password, &user.password_hash)? {
            anyhow::bail!("密码错误");
        }

        // 生成 Token
        let token = self.jwt_service.generate_token(
            user.id,
            &user.username,
            &user.role.to_string(),
        )?;

        Ok((user, token))
    }

    /// 刷新 Token
    pub async fn refresh_token(&self, token: &str) -> Result<String> {
        self.jwt_service.refresh_token(token)
    }
}
```

**文件**: `services/user-management/src/application/service/user_service.rs` (新建)

```rust
use std::sync::Arc;
use anyhow::Result;
use uuid::Uuid;

use crate::domain::model::user::User;
use crate::domain::port::user_repository_port::UserRepositoryPort;

/// 用户服务
pub struct UserService<R: UserRepositoryPort> {
    user_repo: Arc<R>,
}

impl<R: UserRepositoryPort> UserService<R> {
    pub fn new(user_repo: Arc<R>) -> Self {
        Self { user_repo }
    }

    /// 获取用户
    pub async fn get_user(&self, id: Uuid) -> Result<Option<User>> {
        self.user_repo.find_by_id(id).await
    }

    /// 获取用户列表
    pub async fn list_users(&self, offset: i64, limit: i64) -> Result<Vec<User>> {
        self.user_repo.list(offset, limit).await
    }

    /// 更新用户
    pub async fn update_user(&self, user: &User) -> Result<()> {
        self.user_repo.update(user).await
    }

    /// 删除用户
    pub async fn delete_user(&self, id: Uuid) -> Result<()> {
        self.user_repo.delete(id).await
    }
}
```

**文件**: `services/user-management/src/application/service/mod.rs`

```rust
pub mod auth_service;
pub mod user_service;

pub use auth_service::*;
pub use user_service::*;
```

---

### 任务 U5: 创建 DTO

**文件**: `services/user-management/src/interface/http/dto/mod.rs` (新建)

```rust
pub mod auth;
pub mod user;
```

**文件**: `services/user-management/src/interface/http/dto/auth.rs` (新建)

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 注册请求
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

/// 登录请求
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

/// 登录响应
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user_id: Uuid,
    pub username: String,
    pub token: String,
    pub expires_in: i64,
}

/// 刷新 Token 请求
#[derive(Debug, Deserialize)]
pub struct RefreshTokenRequest {
    pub token: String,
}

/// Token 响应
#[derive(Debug, Serialize)]
pub struct TokenResponse {
    pub token: String,
    pub expires_in: i64,
}
```

**文件**: `services/user-management/src/interface/http/dto/user.rs` (新建)

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用户响应
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub role: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// 更新用户请求
#[derive(Debug, Deserialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub role: Option<String>,
    pub status: Option<String>,
}

/// 通用 API 响应
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    pub fn err(msg: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(msg.into()),
        }
    }
}
```

---

### 任务 U6: 创建 Handler

**文件**: `services/user-management/src/interface/http/handlers/auth.rs` (新建)

```rust
use axum::{extract::State, Json};

use crate::interface::http::dto::auth::*;
use crate::interface::http::dto::user::ApiResponse;
use crate::state::AppState;

/// POST /api/v1/auth/register
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    match state.auth_service.register(&req.username, &req.email, &req.password).await {
        Ok(user) => {
            // 注册成功后自动登录
            match state.auth_service.login(&req.username, &req.password).await {
                Ok((_, token)) => Json(ApiResponse::ok(LoginResponse {
                    user_id: user.id,
                    username: user.username,
                    token,
                    expires_in: 24 * 3600,
                })),
                Err(e) => Json(ApiResponse::err(e.to_string())),
            }
        }
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// POST /api/v1/auth/login
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Json<ApiResponse<LoginResponse>> {
    match state.auth_service.login(&req.username, &req.password).await {
        Ok((user, token)) => Json(ApiResponse::ok(LoginResponse {
            user_id: user.id,
            username: user.username,
            token,
            expires_in: 24 * 3600,
        })),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// POST /api/v1/auth/refresh
pub async fn refresh_token(
    State(state): State<AppState>,
    Json(req): Json<RefreshTokenRequest>,
) -> Json<ApiResponse<TokenResponse>> {
    match state.auth_service.refresh_token(&req.token).await {
        Ok(token) => Json(ApiResponse::ok(TokenResponse {
            token,
            expires_in: 24 * 3600,
        })),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
```

**文件**: `services/user-management/src/interface/http/handlers/users.rs` (新建)

```rust
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::interface::http::dto::user::*;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

/// GET /api/v1/users
pub async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Json<ApiResponse<Vec<UserResponse>>> {
    let offset = query.offset.unwrap_or(0);
    let limit = query.limit.unwrap_or(20).min(100);

    match state.user_service.list_users(offset, limit).await {
        Ok(users) => {
            let responses: Vec<UserResponse> = users
                .into_iter()
                .map(|u| UserResponse {
                    id: u.id,
                    username: u.username,
                    email: u.email,
                    role: u.role.to_string(),
                    status: format!("{:?}", u.status),
                    created_at: u.created_at,
                })
                .collect();
            Json(ApiResponse::ok(responses))
        }
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// GET /api/v1/users/{id}
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<UserResponse>> {
    match state.user_service.get_user(id).await {
        Ok(Some(user)) => Json(ApiResponse::ok(UserResponse {
            id: user.id,
            username: user.username,
            email: user.email,
            role: user.role.to_string(),
            status: format!("{:?}", user.status),
            created_at: user.created_at,
        })),
        Ok(None) => Json(ApiResponse::err("用户不存在")),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}

/// DELETE /api/v1/users/{id}
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<ApiResponse<String>> {
    match state.user_service.delete_user(id).await {
        Ok(_) => Json(ApiResponse::ok("用户已删除".to_string())),
        Err(e) => Json(ApiResponse::err(e.to_string())),
    }
}
```

**文件**: `services/user-management/src/interface/http/handlers/mod.rs`

```rust
pub mod auth;
pub mod users;

pub use auth::*;
pub use users::*;
```

---

### 任务 U7: 更新路由

**文件**: `services/user-management/src/interface/http/routes.rs`

```rust
use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::interface::http::handlers;
use crate::state::AppState;

/// 创建路由
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // 认证（公开）
        .route("/api/v1/auth/register", post(handlers::register))
        .route("/api/v1/auth/login", post(handlers::login))
        .route("/api/v1/auth/refresh", post(handlers::refresh_token))
        
        // 用户管理
        .route("/api/v1/users", get(handlers::list_users))
        .route("/api/v1/users/:id", get(handlers::get_user))
        .route("/api/v1/users/:id", delete(handlers::delete_user))
        
        // 健康检查
        .route("/health", get(health_check))
        
        .with_state(state)
}

async fn health_check() -> &'static str {
    "OK"
}
```

---

## 四、需要添加的依赖

**Cargo.toml**:
```toml
argon2 = "0.5"
jsonwebtoken = "9"
```

---

## 五、环境变量

```env
USER_MANAGEMENT_PORT=8084
JWT_SECRET=your-secret-key-change-in-production
JWT_EXPIRY_HOURS=24
DATABASE_URL=postgres://postgres:password@localhost:5432/trading
```

---

## 六、API 接口汇总

| 方法 | 路径 | 说明 | 认证 |
|------|------|------|------|
| POST | `/api/v1/auth/register` | 用户注册 | ❌ |
| POST | `/api/v1/auth/login` | 用户登录 | ❌ |
| POST | `/api/v1/auth/refresh` | 刷新 Token | ❌ |
| GET | `/api/v1/users` | 用户列表 | ✅ |
| GET | `/api/v1/users/{id}` | 用户详情 | ✅ |
| DELETE | `/api/v1/users/{id}` | 删除用户 | ✅ Admin |
| GET | `/health` | 健康检查 | ❌ |

---

## 七、禁止事项（红线）

| 禁止项 | 说明 |
|--------|------|
| ❌ `unwrap()` | 用 `?` 或 `ok_or()` |
| ❌ `expect()` | 同上 |
| ❌ `panic!()` | 用 `anyhow::bail!()` |
| ❌ 明文存储密码 | 必须用 argon2 哈希 |
| ❌ 硬编码 JWT Secret | 从环境变量读取 |

---

## 八、验收标准

### 8.1 编译检查
```bash
cargo check -p user-management
```

### 8.2 功能验收
- [ ] 用户注册正常
- [ ] 用户登录返回 JWT
- [ ] Token 刷新正常
- [ ] 用户 CRUD 正常
- [ ] 密码正确哈希存储

### 8.3 测试方法
```bash
# 注册
curl -X POST http://localhost:8084/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username":"test","email":"test@example.com","password":"123456"}'

# 登录
curl -X POST http://localhost:8084/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"test","password":"123456"}'
```

---

**有问题先问，不要猜！**
