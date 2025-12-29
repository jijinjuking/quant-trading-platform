//! # 缓存端口
//!
//! 本模块定义缓存相关的端口（Port）接口。
//!
//! ## 六边形架构
//! 缓存端口是领域层定义的抽象接口，由基础设施层实现。
//!
//! ## 职责
//! - 定义缓存读写接口
//! - 定义限流检查接口
//!
//! ## 实现者
//! - `infrastructure::cache::RedisCache`（Redis 实现）
//! - 内存缓存适配器（测试用）

/// 缓存端口 - 领域层定义的抽象接口
///
/// 定义缓存相关的操作，由基础设施层提供具体实现。
/// 遵循依赖倒置原则，领域层不依赖具体实现。
///
/// # 约束
/// - `Send + Sync`: 支持多线程环境
///
/// # 实现要求
/// 实现者需要处理：
/// - 缓存的序列化/反序列化
/// - TTL 过期管理
/// - 限流计数器
#[allow(dead_code)]
pub trait CachePort: Send + Sync {
    /// 获取缓存值
    ///
    /// # 参数
    /// - `key`: 缓存键
    ///
    /// # 返回值
    /// - `Some(value)`: 缓存命中
    /// - `None`: 缓存未命中或已过期
    fn get(&self, key: &str) -> Option<String>;
    
    /// 设置缓存值
    ///
    /// # 参数
    /// - `key`: 缓存键
    /// - `value`: 缓存值
    /// - `ttl_seconds`: 过期时间（秒）
    ///
    /// # 返回值
    /// - `true`: 设置成功
    /// - `false`: 设置失败
    fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> bool;
    
    /// 删除缓存
    ///
    /// # 参数
    /// - `key`: 缓存键
    ///
    /// # 返回值
    /// - `true`: 删除成功
    /// - `false`: 删除失败或键不存在
    fn delete(&self, key: &str) -> bool;
    
    /// 检查限流
    ///
    /// 基于滑动窗口或令牌桶算法检查请求频率。
    ///
    /// # 参数
    /// - `key`: 限流键（通常包含用户 ID 或 IP）
    /// - `max_requests`: 时间窗口内最大请求数
    ///
    /// # 返回值
    /// - `true`: 未超过限制，允许请求
    /// - `false`: 超过限制，拒绝请求
    fn check_rate_limit(&self, key: &str, max_requests: u32) -> bool;
}
