//! # Performance Model - 绩效指标模型
//!
//! ## 模块职责
//! 定义交易绩效相关的领域模型，用于：
//! - 衡量交易策略的表现
//! - 生成绩效报告
//! - 风险评估参考
//!
//! ## 领域概念
//! - **总收益率**: 投资期间的累计收益百分比
//! - **夏普比率**: 风险调整后收益指标
//! - **最大回撤**: 从峰值到谷底的最大跌幅
//! - **胜率**: 盈利交易占总交易的比例

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// 绩效指标
///
/// 用于衡量交易策略或用户账户的表现，
/// 是数据分析服务的核心领域模型
///
/// ## 字段说明
/// - `total_return`: 总收益率，使用 Decimal 保证精度
/// - `sharpe_ratio`: 夏普比率，衡量风险调整后收益
/// - `max_drawdown`: 最大回撤，衡量最大亏损风险
/// - `win_rate`: 胜率，盈利交易的比例
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // 骨架阶段，结构体暂未使用
pub struct PerformanceMetrics {
    /// 总收益率（如 0.15 表示 15% 收益）
    pub total_return: Decimal,
    
    /// 夏普比率（风险调整后收益）
    /// 计算公式: (收益率 - 无风险利率) / 收益率标准差
    /// 一般认为 > 1 为良好，> 2 为优秀
    pub sharpe_ratio: f64,
    
    /// 最大回撤（如 0.08 表示最大亏损 8%）
    /// 衡量从历史最高点到最低点的最大跌幅
    pub max_drawdown: Decimal,
    
    /// 胜率（如 0.65 表示 65% 的交易盈利）
    pub win_rate: f64,
}
