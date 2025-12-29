//! # 用户领域模型
//!
//! 本文件定义用户相关的领域模型，包括用户实体和相关值对象。
//!
//! ## 所属层
//! Domain Layer > Model
//!
//! ## 包含内容
//! - `User`: 用户实体（聚合根）
//! - `MembershipTier`: 会员等级值对象
//! - `UserStatus`: 用户状态值对象

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// 用户实体（聚合根）
///
/// 代表系统中的一个用户，是用户聚合的根实体。
/// 包含用户的基本信息、会员等级和状态。
///
/// # 字段
/// - `id`: 用户唯一标识（UUID）
/// - `username`: 用户名
/// - `email`: 电子邮箱
/// - `password_hash`: 密码哈希值
/// - `membership_tier`: 会员等级
/// - `status`: 用户状态
/// - `created_at`: 创建时间
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// 用户唯一标识
    pub id: Uuid,
    /// 用户名
    pub username: String,
    /// 电子邮箱
    pub email: String,
    /// 密码哈希值（不存储明文密码）
    pub password_hash: String,
    /// 会员等级
    pub membership_tier: MembershipTier,
    /// 用户状态
    pub status: UserStatus,
    /// 创建时间（UTC）
    pub created_at: DateTime<Utc>,
}

/// 会员等级（值对象）
///
/// 定义用户的会员等级，不同等级享有不同的权限和功能。
///
/// # 变体
/// - `Free`: 免费用户
/// - `Basic`: 基础会员
/// - `Pro`: 专业会员
/// - `Enterprise`: 企业会员
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MembershipTier {
    /// 免费用户 - 基础功能
    Free,
    /// 基础会员 - 扩展功能
    Basic,
    /// 专业会员 - 高级功能
    Pro,
    /// 企业会员 - 全部功能
    Enterprise,
}

/// 用户状态（值对象）
///
/// 定义用户账户的当前状态。
///
/// # 变体
/// - `Active`: 活跃状态，可正常使用
/// - `Inactive`: 非活跃状态，暂停使用
/// - `Suspended`: 已封禁，禁止使用
#[allow(dead_code)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    /// 活跃状态
    Active,
    /// 非活跃状态
    Inactive,
    /// 已封禁
    Suspended,
}
