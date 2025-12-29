//! # AI 分析端口
//!
//! 定义 AI 分析能力的抽象接口（端口）。
//!
//! ## 六边形架构说明
//! 此 trait 是领域层定义的"驱动端口"（Driven Port），
//! 由基础设施层的适配器（如 DeepSeekClient）实现。
//!
//! ## 设计原则
//! - 只使用领域对象作为参数和返回值
//! - 不暴露任何外部 SDK 或框架类型
//! - 保持领域层的纯净性

use crate::domain::model::analysis::MarketAnalysis;

/// AI 分析端口 trait
///
/// 定义 AI 分析和策略生成的抽象接口。
/// 具体实现由基础设施层提供（如 DeepSeek、OpenAI 等）。
///
/// ## 实现要求
/// - 必须实现 `Send + Sync` 以支持异步并发
/// - 实现者负责 SDK 类型与领域类型的转换
#[allow(dead_code)]
pub trait AiAnalysisPort: Send + Sync {
    /// 分析市场数据
    ///
    /// 接收市场数据输入，返回 AI 分析结果。
    ///
    /// # Arguments
    /// * `input` - 市场数据输入（JSON 格式或文本描述）
    ///
    /// # Returns
    /// 市场分析结果，包含趋势、置信度和分析理由
    fn analyze(&self, input: &str) -> MarketAnalysis;
    
    /// 生成策略建议
    ///
    /// 根据上下文信息生成交易策略建议。
    ///
    /// # Arguments
    /// * `context` - 策略生成上下文（市场状况、用户偏好等）
    ///
    /// # Returns
    /// 策略建议文本
    fn generate_strategy(&self, context: &str) -> String;
}
