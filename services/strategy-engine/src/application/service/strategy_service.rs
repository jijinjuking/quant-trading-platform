//! # 策略服务 (Strategy Service)
//! 
//! 策略管理的用例编排，只依赖 trait。

use uuid::Uuid;
use crate::domain::model::strategy::Strategy;
use crate::domain::model::signal::Signal;
use crate::domain::port::strategy_repository_port::StrategyRepositoryPort;
use crate::domain::port::message_port::SignalMessagePort;

/// 策略服务 - 用例编排
/// 
/// 泛型参数实现依赖倒置：
/// - `R`: 策略仓储端口
/// - `M`: 消息端口
#[allow(dead_code)]
pub struct StrategyService<R: StrategyRepositoryPort, M: SignalMessagePort> {
    /// 策略仓储
    repository: R,
    /// 消息推送
    messenger: M,
}

impl<R: StrategyRepositoryPort, M: SignalMessagePort> StrategyService<R, M> {
    /// 创建策略服务实例
    #[allow(dead_code)]
    pub fn new(repository: R, messenger: M) -> Self {
        Self { repository, messenger }
    }
    
    /// 创建策略
    #[allow(dead_code)]
    pub fn create_strategy(&self, strategy: &Strategy) -> bool {
        self.repository.save(strategy)
    }
    
    /// 获取策略
    #[allow(dead_code)]
    pub fn get_strategy(&self, id: Uuid) -> Option<Strategy> {
        self.repository.find_by_id(id)
    }
    
    /// 发布交易信号
    #[allow(dead_code)]
    pub fn publish_signal(&self, signal: &Signal) -> bool {
        self.messenger.publish_signal(signal)
    }
}
