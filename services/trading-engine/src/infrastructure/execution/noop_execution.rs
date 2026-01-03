//! # Noop 执行适配器 (Noop Execution Adapter) - v1 占位骨架
//!
//! 占位实现，仅记录日志，不执行任何真实交易。
//!
//! ## 架构位置
//! Infrastructure Layer > Execution Adapter
//!
//! ## 版本说明
//! v1 仅作为结构性通道，不对接任何交易所。
//!
//! ## 职责
//! - 实现 ExecutionPort trait
//! - 仅记录指令信息（日志输出）
//! - 永远返回 Ok
//!
//! ## 规则
//! - 不允许保存状态
//! - 不允许写缓存
//! - 不允许访问数据库
//! - 不允许访问网络
//! - 不允许对接交易所

use async_trait::async_trait;
use crate::domain::port::execution_port::{ExecutionPort, ExecutionCommand};
use tracing::info;

/// Noop 执行适配器 - v1 占位骨架
///
/// 占位实现，用于架构验证。
/// 未来可替换为真实交易所执行实现。
pub struct NoopExecution;

impl NoopExecution {
    /// 创建 NoopExecution 实例
    pub fn new() -> Self {
        Self
    }
}

impl Default for NoopExecution {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl ExecutionPort for NoopExecution {
    async fn execute(&self, command: &ExecutionCommand) -> anyhow::Result<()> {
        // v1: 仅记录日志，始终成功
        println!(
            "[NoopExecution] ✅ EXECUTE: symbol={}, side={}, quantity={} | (v1 noop - no real order)",
            command.symbol, command.side, command.quantity
        );
        info!(
            symbol = %command.symbol,
            side = %command.side,
            quantity = %command.quantity,
            "NoopExecution: execute received (v1 always succeeds)"
        );
        Ok(())
    }
}
