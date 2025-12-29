//! # 回撤检查逻辑 (Drawdown Check Logic)
//!
//! 本模块属于领域层，实现回撤相关的风险检查规则。
//!
//! ## 职责
//! - 检查当前回撤是否超过允许的最大回撤
//! - 提供回撤相关的业务规则
//!
//! ## 回撤说明
//! 回撤（Drawdown）是指从历史最高点到当前点的跌幅，
//! 是衡量投资风险的重要指标。

use anyhow::Result;
use rust_decimal::Decimal;

/// 检查回撤是否在允许范围内
///
/// 比较当前回撤与最大允许回撤，判断是否符合风控要求。
///
/// # 参数
/// - `current_drawdown`: 当前回撤比例（如 0.15 表示 15% 回撤）
/// - `max_drawdown`: 允许的最大回撤比例
///
/// # 返回值
/// - `Ok(true)`: 回撤在允许范围内
/// - `Ok(false)`: 回撤超过允许范围
///
/// # 示例
/// ```ignore
/// use rust_decimal_macros::dec;
/// let result = check_drawdown(dec!(0.15), dec!(0.20));
/// assert!(result.unwrap()); // 15% 回撤 <= 20% 最大回撤
/// ```
#[allow(dead_code)]
pub fn check_drawdown(current_drawdown: Decimal, max_drawdown: Decimal) -> Result<bool> {
    Ok(current_drawdown <= max_drawdown)
}
