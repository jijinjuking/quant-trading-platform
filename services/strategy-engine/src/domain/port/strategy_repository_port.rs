//! # 策略仓储端口 (Strategy Repository Port)
//! 
//! 定义策略持久化的抽象接口。

use uuid::Uuid;
use crate::domain::model::strategy::Strategy;

/// 策略仓储端口 - Domain 层定义的抽象接口
pub trait StrategyRepositoryPort: Send + Sync {
    /// 保存策略
    fn save(&self, strategy: &Strategy) -> bool;
    
    /// 根据ID查询策略
    fn find_by_id(&self, id: Uuid) -> Option<Strategy>;
    
    /// 查询用户所有策略
    fn find_by_user_id(&self, user_id: Uuid) -> Vec<Strategy>;
    
    /// 查询所有激活的策略
    fn find_active(&self) -> Vec<Strategy>;
}
