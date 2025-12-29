//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use crate::infrastructure::repository::risk_repository::RiskRepository;
use crate::application::service::risk_check_service::RiskCheckService;

/// 创建风险检查服务实例
#[allow(dead_code)]
pub fn create_risk_check_service() -> RiskCheckService<RiskRepository> {
    let repository = RiskRepository::new();
    RiskCheckService::new(repository)
}
