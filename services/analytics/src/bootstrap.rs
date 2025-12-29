//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use crate::infrastructure::clickhouse::client::ClickHouseClient;
use crate::application::service::analytics_service::AnalyticsService;

/// 创建分析服务实例
///
/// # 参数
/// - `clickhouse_url`: ClickHouse 连接地址
#[allow(dead_code)]
pub fn create_analytics_service(
    clickhouse_url: String,
) -> AnalyticsService<ClickHouseClient> {
    let client = ClickHouseClient::new(clickhouse_url);
    AnalyticsService::new(client)
}
