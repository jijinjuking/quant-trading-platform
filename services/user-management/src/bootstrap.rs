//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use crate::infrastructure::repository::user_repository::UserRepository;
use crate::application::service::user_service::UserService;

/// 创建用户服务实例
#[allow(dead_code)]
pub fn create_user_service() -> UserService<UserRepository> {
    let repository = UserRepository::new();
    UserService::new(repository)
}
