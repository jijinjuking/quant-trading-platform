//! # 缓存基础设施模块
//!
//! 本模块包含缓存相关的基础设施实现。
//!
//! ## 职责
//! - 提供 `CachePort` 的具体实现
//! - 封装 Redis 客户端操作
//!
//! ## 子模块
//! - `redis_cache`: Redis 缓存适配器

/// Redis 缓存适配器
pub mod redis_cache;
