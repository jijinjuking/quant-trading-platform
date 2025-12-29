//! # AI 应用服务
//!
//! 应用层服务，负责编排 AI 分析和策略生成的用例流程。
//!
//! ## 设计原则
//! - 只依赖 `AiAnalysisPort` trait，不依赖具体实现
//! - 通过依赖注入获取端口实现
//! - 负责用例编排，不包含业务规则
//!
//! ## 骨架阶段说明
//! 当前为骨架实现，方法直接委托给端口。
//! 后续可添加：日志记录、缓存、事务管理等横切关注点。

use crate::domain::model::analysis::MarketAnalysis;
use crate::domain::port::ai_analysis_port::AiAnalysisPort;

/// AI 应用服务
///
/// 泛型参数 `P` 必须实现 `AiAnalysisPort` trait，
/// 通过依赖注入实现与具体 AI 后端的解耦。
#[allow(dead_code)]
pub struct AiService<P: AiAnalysisPort> {
    /// AI 分析端口（抽象接口）
    port: P,
}

impl<P: AiAnalysisPort> AiService<P> {
    /// 创建新的 AI 服务实例
    ///
    /// # Arguments
    /// * `port` - AI 分析端口的具体实现
    ///
    /// # Returns
    /// AI 服务实例
    #[allow(dead_code)]
    pub fn new(port: P) -> Self {
        Self { port }
    }
    
    /// 执行市场分析用例
    ///
    /// 接收分析输入，调用 AI 端口进行分析，
    /// 返回市场分析结果。
    ///
    /// # Arguments
    /// * `input` - 分析输入（市场数据、交易对等）
    ///
    /// # Returns
    /// 市场分析结果，包含趋势、置信度和分析理由
    #[allow(dead_code)]
    pub fn execute_analysis(&self, input: &str) -> MarketAnalysis {
        self.port.analyze(input)
    }
    
    /// 执行策略生成用例
    ///
    /// 根据上下文信息，调用 AI 端口生成策略建议。
    ///
    /// # Arguments
    /// * `context` - 策略生成上下文（市场状况、用户偏好等）
    ///
    /// # Returns
    /// 生成的策略建议文本
    #[allow(dead_code)]
    pub fn execute_strategy_generation(&self, context: &str) -> String {
        self.port.generate_strategy(context)
    }
}
