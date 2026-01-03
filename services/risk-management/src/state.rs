//! # 应用状态模块 (Application State)
//!
//! 本模块定义风险管理服务的全局应用状态，包括配置信息、数据库连接池等。
//! 应用状态在服务启动时初始化，并在各个请求处理器之间共享。

use anyhow::Result;
use rust_decimal::Decimal;
use std::sync::Arc;

/// 应用状态
///
/// 包含服务运行所需的全局状态，如配置、数据库连接等。
/// 使用 `Arc` 包装以支持多线程安全共享。
#[derive(Clone)]
pub struct AppState {
    /// 应用配置（线程安全共享）
    pub config: Arc<AppConfig>,
}

/// 应用配置
///
/// 存储服务运行所需的配置参数。
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// 允许的交易对列表（空表示允许所有）
    pub allowed_symbols: Vec<String>,
    /// 最小数量
    pub min_quantity: Decimal,
    /// 最大数量
    pub max_quantity: Decimal,
    /// 最大名义价值
    pub max_notional: Decimal,
    /// 最大持仓
    pub max_position: Decimal,
    /// 最大杠杆
    pub max_leverage: Decimal,
    /// 最大回撤
    pub max_drawdown: Decimal,
}

impl AppState {
    /// 创建新的应用状态实例
    ///
    /// 从环境变量读取配置，初始化数据库连接等资源。
    pub async fn new() -> Result<Self> {
        let allowed_symbols = std::env::var("RISK_ALLOW_SYMBOLS")
            .ok()
            .map(|s| s.split(',').map(|x| x.trim().to_uppercase()).collect())
            .unwrap_or_default();

        let config = AppConfig {
            allowed_symbols,
            min_quantity: read_decimal_env("RISK_MIN_QTY", Decimal::new(1, 4)), // 0.0001
            max_quantity: read_decimal_env("RISK_MAX_QTY", Decimal::new(10, 0)), // 10
            max_notional: read_decimal_env("RISK_MAX_NOTIONAL", Decimal::new(100000, 0)), // 100000
            max_position: read_decimal_env("RISK_MAX_POSITION", Decimal::new(100, 0)), // 100
            max_leverage: read_decimal_env("RISK_MAX_LEVERAGE", Decimal::new(10, 0)), // 10x
            max_drawdown: read_decimal_env("RISK_MAX_DRAWDOWN", Decimal::new(20, 2)), // 0.20 = 20%
        };

        Ok(Self {
            config: Arc::new(config),
        })
    }
}

fn read_decimal_env(key: &str, default: Decimal) -> Decimal {
    std::env::var(key)
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(default)
}
