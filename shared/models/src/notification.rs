use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// 统一的通知渠道定义
/// 这是系统中所有通知渠道的权威定�?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationChannel {
    Email,
    Sms,
    Push,
    WebSocket,
    InApp,
    Webhook,
    Slack,
    Discord,
}

/// 统一的通知类型定义
/// 这是系统中所有通知类型的权威定�?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    System,
    Trading,
    Account,
    Security,
    Marketing,
    Alert,
    Reminder,
    Welcome,
    Verification,
    PasswordReset,
    OrderUpdate,
    TradeExecution,
    RiskAlert,
    MaintenanceNotice,
    PromotionalOffer,
    TradingAlert,
    PriceAlert,
    SystemAlert,
    MarketingMessage,
    AccountUpdate,
}

/// 统一的通知优先级定�?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Critical,
    Emergency,
}

/// 统一的通知状态定�?
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationStatus {
    Pending,
    Scheduled,
    Processing,
    Sent,
    Failed,
    Cancelled,
    Expired,
}

/// 跨服务通知事件
/// 用于服务间通过Kafka等消息队列传递通知事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationEvent {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub notification_type: NotificationType,
    pub title: String,
    pub message: String,
    pub channels: Vec<NotificationChannel>,
    pub priority: NotificationPriority,
    pub metadata: HashMap<String, serde_json::Value>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// 通知设置
/// 用于用户偏好设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSettings {
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub push_enabled: bool,
    pub websocket_enabled: bool,
    pub in_app_enabled: bool,
    pub webhook_enabled: bool,
    pub trade_notifications: bool,
    pub price_alerts: bool,
    pub system_notifications: bool,
    pub marketing_emails: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            email_enabled: true,
            sms_enabled: false,
            push_enabled: true,
            websocket_enabled: true,
            in_app_enabled: true,
            webhook_enabled: false,
            trade_notifications: true,
            price_alerts: true,
            system_notifications: true,
            marketing_emails: false,
        }
    }
}

impl std::str::FromStr for NotificationChannel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "email" => Ok(NotificationChannel::Email),
            "sms" => Ok(NotificationChannel::Sms),
            "push" => Ok(NotificationChannel::Push),
            "websocket" => Ok(NotificationChannel::WebSocket),
            "in_app" | "inapp" => Ok(NotificationChannel::InApp),
            "webhook" => Ok(NotificationChannel::Webhook),
            "slack" => Ok(NotificationChannel::Slack),
            "discord" => Ok(NotificationChannel::Discord),
            _ => Err(format!("Unknown notification channel: {}", s)),
        }
    }
}

impl NotificationChannel {
    /// 检查渠道是否需要用户ID
    pub fn requires_user(&self) -> bool {
        match self {
            NotificationChannel::Email 
            | NotificationChannel::Sms 
            | NotificationChannel::Push 
            | NotificationChannel::InApp => true,
            NotificationChannel::WebSocket 
            | NotificationChannel::Webhook
            | NotificationChannel::Slack
            | NotificationChannel::Discord => false,
        }
    }

    /// 检查渠道是否是实时�?
    pub fn is_real_time(&self) -> bool {
        match self {
            NotificationChannel::WebSocket 
            | NotificationChannel::Push 
            | NotificationChannel::InApp => true,
            NotificationChannel::Email 
            | NotificationChannel::Sms 
            | NotificationChannel::Webhook
            | NotificationChannel::Slack
            | NotificationChannel::Discord => false,
        }
    }

    /// 获取渠道的默认优先级
    pub fn default_priority(&self) -> i32 {
        match self {
            NotificationChannel::WebSocket => 1,
            NotificationChannel::Push => 2,
            NotificationChannel::InApp => 3,
            NotificationChannel::Email => 4,
            NotificationChannel::Sms => 5,
            NotificationChannel::Webhook => 6,
            NotificationChannel::Slack => 7,
            NotificationChannel::Discord => 8,
        }
    }
}

impl NotificationPriority {
    pub fn to_numeric(&self) -> i32 {
        match self {
            NotificationPriority::Low => 1,
            NotificationPriority::Normal => 2,
            NotificationPriority::High => 3,
            NotificationPriority::Critical => 4,
            NotificationPriority::Emergency => 5,
        }
    }
}



