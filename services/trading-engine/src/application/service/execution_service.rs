//! # 执行服务
//! 
//! ## 功能层级: 【应用层 Application】
//! ## 职责: 订单执行用例编排
//! 
//! ## 依赖规则:
//! - 只依赖domain::port中的trait（ExchangePort, OrderRepositoryPort）
//! - 不依赖具体实现（BinanceConnector, OrderRepository）

// ============================================================
// 领域层依赖导入（只导入trait和model）
// ============================================================
use crate::domain::model::order::Order;                        // 订单模型
use crate::domain::model::trade::Trade;                        // 成交模型
use crate::domain::port::exchange_port::ExchangePort;          // 交易所端口trait
use crate::domain::port::order_repository_port::OrderRepositoryPort;  // 订单仓储端口trait

// ============================================================
// 执行服务结构体
// ============================================================

/// # ExecutionService - 执行服务
/// 
/// ## 泛型参数:
/// - E: 实现ExchangePort trait的类型
/// - R: 实现OrderRepositoryPort trait的类型
/// 
/// ## 说明:
/// - 使用泛型实现依赖倒置
/// - 不依赖具体实现，只依赖trait
/// - 具体实现在main.rs中注入
pub struct ExecutionService<E: ExchangePort, R: OrderRepositoryPort> {
    /// 交易所端口（trait对象）
    exchange: E,
    /// 订单仓储端口（trait对象）
    repository: R,
}

// ============================================================
// ExecutionService 实现
// ============================================================

impl<E: ExchangePort, R: OrderRepositoryPort> ExecutionService<E, R> {
    /// # 创建新的执行服务
    /// 
    /// ## 参数:
    /// - exchange: 交易所端口实现
    /// - repository: 订单仓储端口实现
    /// 
    /// ## 返回:
    /// - ExecutionService实例
    pub fn new(exchange: E, repository: R) -> Self {
        Self { exchange, repository }
    }
    
    /// # 执行订单
    /// 
    /// ## 参数:
    /// - order: 待执行的订单
    /// 
    /// ## 返回:
    /// - Some(Trade): 执行成功，返回成交记录
    /// - None: 执行失败
    /// 
    /// ## 执行流程:
    /// 1. 保存订单到仓储
    /// 2. 通过交易所端口下单
    /// 3. 返回成交结果
    pub fn execute_order(&self, order: &Order) -> Option<Trade> {
        // 步骤1: 保存订单到仓储
        self.repository.save(order);
        
        // 步骤2: 通过交易所端口下单
        // 步骤3: 返回成交结果
        self.exchange.place_order(order)
    }
}
