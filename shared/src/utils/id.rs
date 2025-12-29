//! ID Utilities

use uuid::Uuid;

/// 生成新的UUID
pub fn new_id() -> Uuid {
    Uuid::new_v4()
}

/// 从字符串解析UUID
pub fn parse_id(s: &str) -> Option<Uuid> {
    Uuid::parse_str(s).ok()
}
