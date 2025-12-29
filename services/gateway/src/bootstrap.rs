//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use crate::infrastructure::auth::jwt_auth::JwtAuthAdapter;
use crate::infrastructure::cache::redis_cache::RedisCache;
use crate::application::service::auth_service::AuthService;

/// 创建认证服务实例
///
/// # 参数
/// - `jwt_secret`: JWT 签名密钥
/// - `redis_url`: Redis 连接地址
#[allow(dead_code)]
pub fn create_auth_service(
    jwt_secret: String,
    redis_url: String,
) -> AuthService<JwtAuthAdapter, RedisCache> {
    let auth = JwtAuthAdapter::new(jwt_secret);
    let cache = RedisCache::new(redis_url);
    AuthService::new(auth, cache)
}
