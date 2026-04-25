//! # 策略故障记录 (Failure Record)
//!
//! 记录策略执行过程中的故障信息，支持故障追溯和分析。
//!
//! ## 工程约束
//! - 保留最近 N 次故障记录（默认 10）
//! - 支持故障分类
//! - 支持故障统计

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// 默认保留的故障记录数量
const DEFAULT_MAX_RECORDS: usize = 10;

/// 故障类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FailureType {
    /// 执行超时
    Timeout,
    /// 执行 panic
    Panic,
    /// 逻辑错误（策略内部错误）
    LogicError,
    /// 外部依赖错误（如交易所 API）
    ExternalError,
    /// 资源不足
    ResourceExhausted,
    /// 配置错误
    ConfigError,
    /// 未知错误
    Unknown,
}

impl std::fmt::Display for FailureType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FailureType::Timeout => write!(f, "timeout"),
            FailureType::Panic => write!(f, "panic"),
            FailureType::LogicError => write!(f, "logic_error"),
            FailureType::ExternalError => write!(f, "external_error"),
            FailureType::ResourceExhausted => write!(f, "resource_exhausted"),
            FailureType::ConfigError => write!(f, "config_error"),
            FailureType::Unknown => write!(f, "unknown"),
        }
    }
}

/// 单条故障记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureRecord {
    /// 故障类型
    pub failure_type: FailureType,
    /// 故障时间
    pub timestamp: DateTime<Utc>,
    /// 错误消息
    pub message: String,
    /// 错误上下文（可选的额外信息）
    pub context: Option<String>,
    /// 是否已恢复
    pub recovered: bool,
}

impl FailureRecord {
    /// 创建故障记录
    pub fn new(failure_type: FailureType, message: impl Into<String>) -> Self {
        Self {
            failure_type,
            timestamp: Utc::now(),
            message: message.into(),
            context: None,
            recovered: false,
        }
    }

    /// 添加上下文信息
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// 标记为已恢复
    pub fn mark_recovered(&mut self) {
        self.recovered = true;
    }
}

/// 故障历史记录器
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureHistory {
    /// 故障记录队列（FIFO）
    records: VecDeque<FailureRecord>,
    /// 最大记录数量
    max_records: usize,
    /// 总故障次数（包括已移除的）
    total_failures: u64,
    /// 连续故障次数
    consecutive_failures: u32,
}

impl FailureHistory {
    /// 创建故障历史记录器
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_MAX_RECORDS)
    }

    /// 创建指定容量的故障历史记录器
    pub fn with_capacity(max_records: usize) -> Self {
        Self {
            records: VecDeque::with_capacity(max_records),
            max_records,
            total_failures: 0,
            consecutive_failures: 0,
        }
    }

    /// 记录故障
    pub fn record(&mut self, failure: FailureRecord) {
        self.total_failures += 1;
        self.consecutive_failures += 1;

        if self.records.len() >= self.max_records {
            self.records.pop_front();
        }
        self.records.push_back(failure);
    }

    /// 记录成功执行（重置连续故障计数）
    pub fn record_success(&mut self) {
        self.consecutive_failures = 0;
    }

    /// 获取最近的故障记录
    pub fn recent(&self) -> impl Iterator<Item = &FailureRecord> {
        self.records.iter().rev()
    }

    /// 获取最近一条故障记录
    pub fn last(&self) -> Option<&FailureRecord> {
        self.records.back()
    }

    /// 获取故障记录数量
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// 是否为空
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// 获取总故障次数
    pub fn total_failures(&self) -> u64 {
        self.total_failures
    }

    /// 获取连续故障次数
    pub fn consecutive_failures(&self) -> u32 {
        self.consecutive_failures
    }

    /// 是否超过连续故障阈值
    pub fn exceeds_threshold(&self, threshold: u32) -> bool {
        self.consecutive_failures >= threshold
    }

    /// 按故障类型统计
    pub fn count_by_type(&self, failure_type: FailureType) -> usize {
        self.records
            .iter()
            .filter(|r| r.failure_type == failure_type)
            .count()
    }

    /// 清空历史记录
    pub fn clear(&mut self) {
        self.records.clear();
        self.consecutive_failures = 0;
    }
}

impl Default for FailureHistory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_failure_record_creation() {
        let record = FailureRecord::new(FailureType::Timeout, "执行超时");
        assert_eq!(record.failure_type, FailureType::Timeout);
        assert_eq!(record.message, "执行超时");
        assert!(!record.recovered);
    }

    #[test]
    fn test_failure_history_fifo() {
        let mut history = FailureHistory::with_capacity(3);

        history.record(FailureRecord::new(FailureType::Timeout, "1"));
        history.record(FailureRecord::new(FailureType::Panic, "2"));
        history.record(FailureRecord::new(FailureType::LogicError, "3"));
        history.record(FailureRecord::new(FailureType::Unknown, "4"));

        assert_eq!(history.len(), 3);
        assert_eq!(history.total_failures(), 4);

        let messages: Vec<_> = history.recent().map(|r| r.message.as_str()).collect();
        assert_eq!(messages, vec!["4", "3", "2"]);
    }

    #[test]
    fn test_consecutive_failures() {
        let mut history = FailureHistory::new();

        history.record(FailureRecord::new(FailureType::Timeout, "1"));
        history.record(FailureRecord::new(FailureType::Timeout, "2"));
        assert_eq!(history.consecutive_failures(), 2);

        history.record_success();
        assert_eq!(history.consecutive_failures(), 0);

        history.record(FailureRecord::new(FailureType::Timeout, "3"));
        assert_eq!(history.consecutive_failures(), 1);
    }

    #[test]
    fn test_exceeds_threshold() {
        let mut history = FailureHistory::new();

        history.record(FailureRecord::new(FailureType::Timeout, "1"));
        history.record(FailureRecord::new(FailureType::Timeout, "2"));
        history.record(FailureRecord::new(FailureType::Timeout, "3"));

        assert!(history.exceeds_threshold(3));
        assert!(!history.exceeds_threshold(4));
    }
}
