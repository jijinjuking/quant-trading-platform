// src/lib.rs

pub mod common;
pub mod market;
pub mod notification;
pub mod risk;
pub mod strategy;
pub mod trading;
pub mod user;
pub mod standards;

// 避免重复导出，使用具体导出
pub use common::*;
pub use market::{MarketTick, Kline, OrderBook, MarketDataSubscription, MarketDataType, MarketDataQuery, MarketDataMessage, MarketDataStats, Trade, Ticker24hr, PriceInfo, DepthSnapshot, OrderBookLevel};
pub use notification::{NotificationChannel as NotificationChannelType, NotificationSettings as NotificationSettingsType, NotificationEvent, NotificationType, NotificationPriority, NotificationStatus};
pub use risk::{RiskRule, RiskLevel, RiskMetric, RiskConfig, RiskEvent, RiskSeverity, RiskDecision, RiskCheckResult, RiskViolation, RiskWarning, LogicalOperator, NotificationChannel as RiskNotificationChannel};
pub use strategy::{Strategy, StrategySignal, BacktestConfig, BacktestResult};
pub use trading::{Order, Position, Balance, Trade as TradingTrade, OrderSide, OrderType, OrderStatus, Account, AccountType};
pub use user::{User, UpdateUserRequest, ChangePasswordRequest, LoginRequest, LoginResponse, CreateUserRequest, RegisterRequest, UserStatus, ApiKey, ApiPermission, UserSession, DeviceInfo};
pub use standards::{AI_SPEC, print_spec};



