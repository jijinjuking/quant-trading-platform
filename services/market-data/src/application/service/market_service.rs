//! # 行情服务 (Market Service)
//! 
//! 行情数据的用例编排服务。
//! 
//! ## Hexagonal 架构说明
//! - 只依赖 domain::port 中定义的 trait
//! - 不直接依赖 infrastructure 的具体实现
//! - 通过泛型参数注入具体实现
//! 
//! ## 用例
//! - 获取并推送行情数据

// ============================================================================
// 领域层依赖导入（只导入 trait 和 model）
// ============================================================================

use crate::domain::port::exchange_port::MarketExchangePort;  // 交易所端口 trait
use crate::domain::port::message_port::MessagePort;          // 消息端口 trait

// ============================================================================
// 行情服务结构体
// ============================================================================

/// 行情服务 - 用例编排
/// 
/// 使用泛型参数实现依赖倒置：
/// - `E`: 实现 MarketExchangePort 的交易所适配器
/// - `M`: 实现 MessagePort 的消息适配器
/// 
/// # 示例
/// ```ignore
/// let service = MarketService::new(binance_ws, kafka_producer);
/// service.fetch_and_publish("BTCUSDT");
/// ```
#[allow(dead_code)]  // 骨架阶段允许未使用结构体
pub struct MarketService<E: MarketExchangePort, M: MessagePort> {
    /// 交易所端口 - 获取行情数据
    exchange: E,
    /// 消息端口 - 推送行情事件
    messenger: M,
}

// ============================================================================
// 行情服务实现
// ============================================================================

impl<E: MarketExchangePort, M: MessagePort> MarketService<E, M> {
    /// 创建新的行情服务实例
    /// 
    /// # 参数
    /// - `exchange`: 交易所端口实现
    /// - `messenger`: 消息端口实现
    /// 
    /// # 返回
    /// - 配置好的 MarketService 实例
    #[allow(dead_code)]  // 骨架阶段允许未使用函数
    pub fn new(exchange: E, messenger: M) -> Self {
        Self { exchange, messenger }
    }
    
    /// 获取并推送行情数据
    /// 
    /// 用例流程：
    /// 1. 从交易所获取最新 Tick 数据
    /// 2. 通过消息队列推送给订阅者
    /// 
    /// # 参数
    /// - `symbol`: 交易对符号（如 "BTCUSDT"）
    /// 
    /// # 返回
    /// - `true`: 获取并推送成功
    /// - `false`: 获取失败或推送失败
    #[allow(dead_code)]  // 骨架阶段允许未使用函数
    pub fn fetch_and_publish(&self, symbol: &str) -> bool {
        // 1. 从交易所获取最新 Tick
        if let Some(tick) = self.exchange.get_latest_tick(symbol) {
            // 2. 推送到消息队列
            return self.messenger.publish_tick(&tick);
        }
        false
    }
}
