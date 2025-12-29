//! # 市场分析模型
//!
//! 定义 AI 市场分析的核心领域模型。
//!
//! ## 模型说明
//! - `MarketAnalysis`: 市场分析结果聚合
//! - `TrendDirection`: 趋势方向值对象

use serde::{Deserialize, Serialize};

/// 市场分析结果
///
/// 表示 AI 对市场的分析结论，包含趋势判断、
/// 置信度评分和分析理由。
///
/// ## 字段说明
/// - `trend`: 市场趋势方向（看涨/看跌/中性）
/// - `confidence`: 分析置信度（0.0 - 1.0）
/// - `reasoning`: AI 给出的分析理由
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MarketAnalysis {
    /// 趋势方向
    pub trend: TrendDirection,
    /// 置信度（0.0 - 1.0）
    pub confidence: f64,
    /// 分析理由
    pub reasoning: String,
}

/// 趋势方向枚举
///
/// 表示市场的整体趋势方向。
///
/// ## 变体
/// - `Bullish`: 看涨（牛市）
/// - `Bearish`: 看跌（熊市）
/// - `Neutral`: 中性（震荡）
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum TrendDirection {
    /// 看涨趋势
    Bullish,
    /// 看跌趋势
    Bearish,
    /// 中性/震荡
    Neutral,
}
