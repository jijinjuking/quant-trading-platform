# 📋 API 网关开发任务书

> **任务类型**: 网关路由 + 认证
> **负责服务**: `gateway` (8080)
> **验收人**: Kiro（架构师）
> **必读文档**: `TEAM_DEVELOPMENT_GUIDE.md`
> **优先级**: 🟡 中（统一入口）

---

## 一、任务概述

实现 API 网关，作为所有外部请求的统一入口，负责：
- 路由转发到各微服务
- JWT 认证
- 请求限流（可选）
- 日志记录

```
客户端 → Gateway (8080) → 各微服务
                ↓
         JWT 认证 / 路由转发
```

---

## 二、当前状态

```
services/gateway/src/
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
│   └── port/              # ❌ 空
│
├── infrastructure/
│   ├── auth/              # ❌ 需要实现 JWT
│   └── cache/             # ❌ 需要实现 Redis
│
└── interface/http/
    ├── routes.rs          # ⚠️ 骨架
    └── handlers/          # ❌ 空
```

---

## 三、待开发任务清单

### 任务 G1: 实现 JWT 认证中间件

**文件**: `services/gateway/src/infrastructure/auth/jwt.rs` (新建)

```rust
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
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
    /// 过期时间（Unix 时间戳）
    pub exp: i64,
    /// 签发时间
    pub iat: i64,
}

/// JWT 配置
#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub algorithm: Algorithm,
}

impl JwtConfig {
    pub fn from_env() -> Self {
        let secret = std::env::var("JWT_SECRET")
            .unwrap_or_else(|_| "your-secret-key-change-in-production".to_string());
        Self {
            secret,
            algorithm: Algorithm::HS256,
        }
    }
}

/// JWT 认证中间件
pub async fn jwt_auth_middleware(
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 跳过不需要认证的路径
    let path = request.uri().path();
    if is_public_path(path) {
        return Ok(next.run(request).await);
    }

    // 获取 Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(h) if h.starts_with("Bearer ") => &h[7..],
        _ => {
            tracing::warn!("缺少或无效的 Authorization header");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 验证 token
    let config = JwtConfig::from_env();
    let validation = Validation::new(config.algorithm);
    let key = DecodingKey::from_secret(config.secret.as_bytes());

    match decode::<Claims>(token, &key, &validation) {
        Ok(token_data) => {
            tracing::debug!("JWT 验证成功: user={}", token_data.claims.username);
            // TODO: 将 claims 注入到请求扩展中
            Ok(next.run(request).await)
        }
        Err(e) => {
            tracing::warn!("JWT 验证失败: {}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// 判断是否为公开路径（不需要认证）
fn is_public_path(path: &str) -> bool {
    let public_paths = [
        "/health",
        "/api/v1/auth/login",
        "/api/v1/auth/register",
        "/api/v1/auth/refresh",
    ];
    public_paths.iter().any(|p| path.starts_with(p))
}

/// 生成 JWT Token
pub fn generate_token(user_id: Uuid, username: &str, role: &str) -> anyhow::Result<String> {
    use jsonwebtoken::{encode, EncodingKey, Header};
    
    let config = JwtConfig::from_env();
    let now = chrono::Utc::now().timestamp();
    let exp = now + 24 * 3600; // 24 小时过期

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        role: role.to_string(),
        exp,
        iat: now,
    };

    let header = Header::new(config.algorithm);
    let key = EncodingKey::from_secret(config.secret.as_bytes());

    encode(&header, &claims, &key)
        .map_err(|e| anyhow::anyhow!("生成 JWT 失败: {}", e))
}
```

**需要添加的依赖** (Cargo.toml):
```toml
jsonwebtoken = "9"
```

---

### 任务 G2: 实现代理转发

**文件**: `services/gateway/src/infrastructure/proxy/service_proxy.rs` (新建)

```rust
use axum::{
    body::Body,
    extract::Request,
    http::{uri::Uri, StatusCode},
    response::{IntoResponse, Response},
};
use reqwest::Client;
use std::collections::HashMap;

/// 服务路由配置
#[derive(Clone)]
pub struct ServiceRouter {
    /// 服务名 -> 服务地址
    routes: HashMap<String, String>,
    /// HTTP 客户端
    client: Client,
}

impl ServiceRouter {
    pub fn new() -> Self {
        let mut routes = HashMap::new();
        
        // 从环境变量读取服务地址
        routes.insert(
            "trading".to_string(),
            std::env::var("TRADING_ENGINE_URL")
                .unwrap_or_else(|_| "http://localhost:8081".to_string()),
        );
        routes.insert(
            "market".to_string(),
            std::env::var("MARKET_DATA_URL")
                .unwrap_or_else(|_| "http://localhost:8082".to_string()),
        );
        routes.insert(
            "strategy".to_string(),
            std::env::var("STRATEGY_ENGINE_URL")
                .unwrap_or_else(|_| "http://localhost:8083".to_string()),
        );
        routes.insert(
            "user".to_string(),
            std::env::var("USER_MANAGEMENT_URL")
                .unwrap_or_else(|_| "http://localhost:8084".to_string()),
        );
        routes.insert(
            "risk".to_string(),
            std::env::var("RISK_MANAGEMENT_URL")
                .unwrap_or_else(|_| "http://localhost:8085".to_string()),
        );
        routes.insert(
            "notification".to_string(),
            std::env::var("NOTIFICATION_URL")
                .unwrap_or_else(|_| "http://localhost:8086".to_string()),
        );
        routes.insert(
            "ai".to_string(),
            std::env::var("AI_SERVICE_URL")
                .unwrap_or_else(|_| "http://localhost:8087".to_string()),
        );
        routes.insert(
            "analytics".to_string(),
            std::env::var("ANALYTICS_URL")
                .unwrap_or_else(|_| "http://localhost:8088".to_string()),
        );

        Self {
            routes,
            client: Client::new(),
        }
    }

    /// 根据路径确定目标服务
    pub fn resolve_service(&self, path: &str) -> Option<&str> {
        // 路由规则
        if path.starts_with("/api/v1/trading") || path.starts_with("/api/v1/orders") {
            return self.routes.get("trading").map(|s| s.as_str());
        }
        if path.starts_with("/api/v1/market") {
            return self.routes.get("market").map(|s| s.as_str());
        }
        if path.starts_with("/api/v1/strategy") || path.starts_with("/api/v1/strategies") {
            return self.routes.get("strategy").map(|s| s.as_str());
        }
        if path.starts_with("/api/v1/users") || path.starts_with("/api/v1/auth") {
            return self.routes.get("user").map(|s| s.as_str());
        }
        if path.starts_with("/api/v1/risk") {
            return self.routes.get("risk").map(|s| s.as_str());
        }
        if path.starts_with("/api/v1/notifications") {
            return self.routes.get("notification").map(|s| s.as_str());
        }
        if path.starts_with("/api/v1/ai") {
            return self.routes.get("ai").map(|s| s.as_str());
        }
        if path.starts_with("/api/v1/analytics") {
            return self.routes.get("analytics").map(|s| s.as_str());
        }
        None
    }

    /// 转发请求
    pub async fn forward(&self, req: Request) -> Result<Response, StatusCode> {
        let path = req.uri().path();
        let query = req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();
        
        let target_base = self.resolve_service(path)
            .ok_or_else(|| {
                tracing::warn!("无法路由请求: {}", path);
                StatusCode::NOT_FOUND
            })?;

        let target_url = format!("{}{}{}", target_base, path, query);
        tracing::debug!("转发请求: {} -> {}", path, target_url);

        // 构建转发请求
        let method = req.method().clone();
        let headers = req.headers().clone();
        let body = axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .map_err(|_| StatusCode::BAD_REQUEST)?;

        let mut forward_req = self.client.request(method, &target_url);
        
        // 复制 headers（排除 host）
        for (key, value) in headers.iter() {
            if key != "host" {
                forward_req = forward_req.header(key, value);
            }
        }

        // 发送请求
        let response = forward_req
            .body(body)
            .send()
            .await
            .map_err(|e| {
                tracing::error!("转发请求失败: {}", e);
                StatusCode::BAD_GATEWAY
            })?;

        // 构建响应
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;

        let mut res = Response::builder().status(status);
        for (key, value) in headers.iter() {
            res = res.header(key, value);
        }

        res.body(Body::from(body))
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl Default for ServiceRouter {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### 任务 G3: 实现路由和 Handler

**文件**: `services/gateway/src/interface/http/handlers/proxy.rs` (新建)

```rust
use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::Response,
};

use crate::state::AppState;

/// 代理转发 Handler
pub async fn proxy_handler(
    State(state): State<AppState>,
    req: Request,
) -> Result<Response, StatusCode> {
    state.router.forward(req).await
}
```

**文件**: `services/gateway/src/interface/http/handlers/health.rs` (新建)

```rust
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
}

/// 健康检查
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        service: "gateway".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
```

**文件**: `services/gateway/src/interface/http/handlers/mod.rs`

```rust
pub mod proxy;
pub mod health;

pub use proxy::*;
pub use health::*;
```

---

### 任务 G4: 更新路由

**文件**: `services/gateway/src/interface/http/routes.rs`

```rust
use axum::{
    middleware,
    routing::{any, get},
    Router,
};

use crate::infrastructure::auth::jwt::jwt_auth_middleware;
use crate::interface::http::handlers;
use crate::state::AppState;

/// 创建路由
pub fn create_routes(state: AppState) -> Router {
    Router::new()
        // 健康检查（不需要认证）
        .route("/health", get(handlers::health_check))
        
        // 所有 API 请求转发（需要认证）
        .route("/api/*path", any(handlers::proxy_handler))
        
        // 添加 JWT 认证中间件
        .layer(middleware::from_fn(jwt_auth_middleware))
        
        .with_state(state)
}
```

---

### 任务 G5: 更新 AppState

**文件**: `services/gateway/src/state.rs`

```rust
use crate::infrastructure::proxy::service_proxy::ServiceRouter;

/// 应用状态
#[derive(Clone)]
pub struct AppState {
    /// 服务路由器
    pub router: ServiceRouter,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            router: ServiceRouter::new(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
```

---

### 任务 G6: 更新 Main

**文件**: `services/gateway/src/main.rs`

```rust
use anyhow::Result;
use std::net::SocketAddr;
use tracing_subscriber;

mod application;
mod domain;
mod infrastructure;
mod interface;
mod bootstrap;
mod state;

use interface::http::routes::create_routes;
use state::AppState;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    tracing_subscriber::fmt::init();
    
    // 加载环境变量
    dotenv::dotenv().ok();

    tracing::info!("Gateway 启动中...");

    // 创建应用状态
    let state = AppState::new();

    // 创建路由
    let app = create_routes(state);

    // 获取端口
    let port: u16 = std::env::var("GATEWAY_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Gateway 监听: {}", addr);

    // 启动服务
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
```

---

### 任务 G7: 更新模块结构

**文件**: `services/gateway/src/infrastructure/mod.rs`

```rust
pub mod auth;
pub mod cache;
pub mod proxy;
```

**文件**: `services/gateway/src/infrastructure/auth/mod.rs`

```rust
pub mod jwt;
```

**文件**: `services/gateway/src/infrastructure/proxy/mod.rs` (新建)

```rust
pub mod service_proxy;
```

---

## 四、路由规则汇总

| 路径前缀 | 目标服务 | 端口 |
|----------|----------|------|
| `/api/v1/trading`, `/api/v1/orders` | trading-engine | 8081 |
| `/api/v1/market` | market-data | 8082 |
| `/api/v1/strategy`, `/api/v1/strategies` | strategy-engine | 8083 |
| `/api/v1/users`, `/api/v1/auth` | user-management | 8084 |
| `/api/v1/risk` | risk-management | 8085 |
| `/api/v1/notifications` | notification | 8086 |
| `/api/v1/ai` | ai-service | 8087 |
| `/api/v1/analytics` | analytics | 8088 |

---

## 五、环境变量

```env
# Gateway
GATEWAY_PORT=8080
JWT_SECRET=your-secret-key-change-in-production

# 后端服务地址
TRADING_ENGINE_URL=http://localhost:8081
MARKET_DATA_URL=http://localhost:8082
STRATEGY_ENGINE_URL=http://localhost:8083
USER_MANAGEMENT_URL=http://localhost:8084
RISK_MANAGEMENT_URL=http://localhost:8085
NOTIFICATION_URL=http://localhost:8086
AI_SERVICE_URL=http://localhost:8087
ANALYTICS_URL=http://localhost:8088
```

---

## 六、禁止事项（红线）

| 禁止项 | 说明 |
|--------|------|
| ❌ `unwrap()` | 用 `?` 或 `ok_or()` |
| ❌ `expect()` | 同上 |
| ❌ `panic!()` | 用 `anyhow::bail!()` |
| ❌ 业务逻辑 | Gateway 只做路由和认证 |
| ❌ 数据存储 | 不存储业务数据 |
| ❌ 直接调用数据库 | 只转发请求 |

---

## 七、验收标准

### 7.1 编译检查
```bash
cargo check -p gateway
```

### 7.2 功能验收
- [ ] 健康检查返回正常
- [ ] JWT 认证正常工作
- [ ] 请求能正确转发到各服务
- [ ] 公开路径不需要认证
- [ ] 日志输出清晰

### 7.3 测试方法
```bash
# 启动 Gateway
cargo run -p gateway

# 测试健康检查
curl http://localhost:8080/health

# 测试无认证访问（应返回 401）
curl http://localhost:8080/api/v1/strategies

# 测试带认证访问
curl -H "Authorization: Bearer <token>" http://localhost:8080/api/v1/strategies
```

---

**有问题先问，不要猜！**
