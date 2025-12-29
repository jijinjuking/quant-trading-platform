//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。
//!
//! ## 六边形架构说明
//! 在 main.rs 或 bootstrap 中完成：
//! - 创建 infrastructure adapter 实例
//! - 将 adapter 注入到 application service
//!
//! ## 依赖方向
//! ```text
//! bootstrap → infrastructure (创建 adapter)
//!          → application (注入 adapter)
//! ```

use crate::infrastructure::exchange::binance::BinanceConnector;
use crate::infrastructure::repository::order_repository::OrderRepository;
use crate::application::service::execution_service::ExecutionService;

/// 创建执行服务实例
///
/// 完成依赖注入：
/// - 创建 BinanceConnector (实现 ExchangePort)
/// - 创建 OrderRepository (实现 OrderRepositoryPort)
/// - 注入到 ExecutionService
///
/// # 参数
/// - `api_key`: 币安 API 密钥
/// - `secret_key`: 币安 API 密钥签名
///
/// # 返回
/// 配置好的 ExecutionService 实例
#[allow(dead_code)]
pub fn create_execution_service(
    api_key: String,
    secret_key: String,
) -> ExecutionService<BinanceConnector, OrderRepository> {
    // 创建 infrastructure adapter
    let exchange = BinanceConnector::new(api_key, secret_key);
    let repository = OrderRepository::new();
    
    // 依赖注入：将 adapter 注入到 application service
    ExecutionService::new(exchange, repository)
}
