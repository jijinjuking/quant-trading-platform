//! Time Utilities

use chrono::{DateTime, Utc};

/// 获取当前UTC时间
pub fn now() -> DateTime<Utc> {
    Utc::now()
}

/// 时间戳转DateTime
pub fn from_timestamp(ts: i64) -> DateTime<Utc> {
    DateTime::from_timestamp(ts, 0).unwrap_or_else(Utc::now)
}
