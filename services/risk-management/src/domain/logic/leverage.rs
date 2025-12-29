//! # 杠杆检查逻辑 (Leverage Check Logic)
//!
//! 本模块属于领域层，实现杠杆相关的风险检查规则。
//!
//! ## 职责
//! - 检查当前杠杆是否超过允许的最大杠杆
//! - 提供杠杆相关的业务规则

use anyhow::Result;
use rust_decimal::Decimal;

/// 检查杠杆是否在允许范围内
///
/// 比较当前杠杆与最大允许杠杆，判断是否符合风控要求。
///
/// # 参数
/// - `current_leverage`: 当前杠杆倍数
/// - `max_leverage`: 允许的最大杠杆倍数
///
/// # 返回值
/// - `Ok(true)`: 杠杆在允许范围内
/// - `Ok(false)`: 杠杆超过允许范围
///
/// # 示例
/// ```ignore
/// use rust_decimal_macros::dec;
/// let result = check_leverage(dec!(5.0), dec!(10.0));
/// assert!(result.unwrap()); // 5 倍杠杆 <= 10 倍最大杠杆
/// ```
#[allow(dead_code)]
pub fn check_leverage(current_leverage: Decimal, max_leverage: Decimal) -> Result<bool> {
    Ok(current_leverage <= max_leverage)
}
