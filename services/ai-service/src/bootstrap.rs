//! # 依赖注入模块 (Bootstrap)
//!
//! 本模块负责组装应用层服务，完成依赖注入。

use crate::infrastructure::deepseek::client::DeepSeekClient;
use crate::application::service::ai_service::AiService;

/// 创建 AI 服务实例
///
/// # 参数
/// - `api_key`: DeepSeek API 密钥
/// - `base_url`: DeepSeek API 地址
#[allow(dead_code)]
pub fn create_ai_service(
    api_key: String,
    base_url: String,
) -> AiService<DeepSeekClient> {
    let client = DeepSeekClient::new(api_key, base_url);
    AiService::new(client)
}
