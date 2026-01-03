//! # 服务状态处理器
//!
//! 检查后端服务的健康状态

use axum::{
    extract::State,
    Json,
};
use serde::Serialize;
use crate::state::AppState;

/// 服务状态响应
#[derive(Debug, Serialize)]
pub struct ServicesStatusResponse {
    /// Gateway 状态
    pub gateway: ServiceStatus,
    /// 策略引擎状态
    pub strategy_engine: ServiceStatus,
    /// 交易引擎状态
    pub trading_engine: ServiceStatus,
}

/// 单个服务状态
#[derive(Debug, Serialize)]
pub struct ServiceStatus {
    /// 服务名称
    pub name: String,
    /// 服务地址
    pub url: String,
    /// 是否健康
    pub healthy: bool,
    /// 错误信息（如果有）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 检查所有后端服务状态
pub async fn check_services(
    State(state): State<AppState>,
) -> Json<ServicesStatusResponse> {
    // 检查策略引擎
    let strategy_status = check_service_health(
        &state.http_client,
        "strategy-engine",
        &state.config.services.strategy_engine,
    ).await;
    
    // 检查交易引擎
    let trading_status = check_service_health(
        &state.http_client,
        "trading-engine",
        &state.config.services.trading_engine,
    ).await;
    
    Json(ServicesStatusResponse {
        gateway: ServiceStatus {
            name: "gateway".to_string(),
            url: "http://localhost:8080".to_string(),
            healthy: true,
            error: None,
        },
        strategy_engine: strategy_status,
        trading_engine: trading_status,
    })
}

/// 检查单个服务健康状态
async fn check_service_health(
    client: &reqwest::Client,
    name: &str,
    base_url: &str,
) -> ServiceStatus {
    let url = format!("{}/health", base_url);
    
    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                ServiceStatus {
                    name: name.to_string(),
                    url: base_url.to_string(),
                    healthy: true,
                    error: None,
                }
            } else {
                ServiceStatus {
                    name: name.to_string(),
                    url: base_url.to_string(),
                    healthy: false,
                    error: Some(format!("HTTP {}", response.status())),
                }
            }
        }
        Err(e) => {
            ServiceStatus {
                name: name.to_string(),
                url: base_url.to_string(),
                healthy: false,
                error: Some(e.to_string()),
            }
        }
    }
}
