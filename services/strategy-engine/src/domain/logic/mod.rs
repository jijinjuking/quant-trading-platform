//! # 领域策略逻辑模块 (Domain Strategy Logic)
//!
//! 策略算法实现与信号生成入口。
//!
//! ## 注意
//! - `strategy_registry` 已迁移到 `domain/service/` 目录
//! - 这里只保留策略算法实现

/// 网格策略逻辑 (Grid Strategy Logic) - 保留兼容
pub mod grid;

/// 均值回归策略逻辑 (Mean Reversion Strategy Logic) - 保留兼容
pub mod mean;

/// 信号生成入口 (Signal Generator)
pub mod signal_generator;

/// 统一策略 Trait (Strategy Trait)
pub mod strategy_trait;

/// 现货策略模块 (Spot Strategies)
pub mod spot;

/// 合约策略模块 (Futures Strategies)
pub mod futures;

/// AI 策略模块 (AI Strategies)
pub mod ai;

/// 高频策略模块 (HFT Strategies)
pub mod hft;
