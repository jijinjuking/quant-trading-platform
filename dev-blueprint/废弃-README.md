# 量化交易平台开发蓝图

## 📋 项目概述

这是一个完整的企业级量化交易平台的开发蓝图文档集合。该平台采用微服务架构，使用Rust语言开发，支持多交易所接入、实时数据分析、智能策略执行和全面风险管理。

## 🏗️ 整体架构

完整的系统架构图请参见: `SYSTEM_ARCHITECTURE_OVERVIEW.md`

该架构包含5个主要层级:
1. 前端展示层 (Vue3 + Vite)
2. API网关层 (Rust + Axum)
3. 业务服务层 (10个微服务)
4. 共持基础设施层 (模型/协议/工具)
5. 数据存储层 (PostgreSQL/Redis/ClickHouse/Kafka)

## 📦 服务模块

### 1. [市场数据服务](market-data/architecture/废弃-ARCHITECTURE.md) (端口: 8083)
- 实时市场数据接入 (WebSocket)
- 多交易所连接管理 (Binance/OKX/Huobi)
- 数据处理与标准化
- 数据分发与缓存

### 2. [交易引擎服务](trading-engine/architecture/废弃-ARCHITECTURE.md) (端口: 8082)
- 订单管理 (创建、修改、取消)
- 账户管理 (余额、持仓)
- 交易执行 (订单匹配、成交)
- 风险控制 (保证金、强平)

### 3. [策略引擎服务](strategy-engine/architecture/废弃-ARCHITECTURE.md) (端口: 8084)
- 策略管理 (创建、配置、启动/停止)
- 信号生成 (技术指标、交易信号)
- 回测系统 (历史数据回测)
- 策略执行 (自动交易)

### 4. [用户管理服务](user-management/architecture/废弃-ARCHITECTURE.md) (端口: 8081)
- 用户认证 (注册、登录、登出)
- 权限管理 (角色、权限控制)
- 用户资料管理
- KYC验证

### 5. [风险管理服务](risk-management/architecture/废弃-ARCHITECTURE.md) (端口: 8085)
- 实时风险监控
- 风险评估与计算
- 风险预警系统
- 风险限额管理

### 6. [通知服务](notification/architecture/废弃-ARCHITECTURE.md) (端口: 8086)
- 多渠道通知发送 (邮件、短信、推送、WebSocket)
- 模板管理 (通知模板、变量替换)
- 订阅管理 (用户订阅设置、频率控制)

### 7. [分析服务](analytics/architecture/废弃-ARCHITECTURE.md) (端口: 8087)
- 性能分析 (策略、投资组合)
- 风险分析 (多维度风险评估)
- 统计报表 (各类分析报表)
- 数据导出 (Excel、PDF、CSV)

### 8. [AI智能服务](ai-service/architecture/废弃-ARCHITECTURE.md) (端口: 8088)
- 价格预测 (机器学习模型)
- 套利机会发现
- 智能信号生成
- 模型管理 (加载、更新、版本控制)

### 9. [API网关](gateway/architecture/废弃-ARCHITECTURE.md) (端口: 8080)
- 统一入口管理
- 请求路由和负载均衡
- 认证和授权
- 限流和熔断

### 10. 管理后台服务 (端口: 8089)
- 系统管理
- 配置管理
- 用户管理

## 📁 文档结构

每个服务模块下包含以下子目录：
- `architecture/` - 架构设计文档
- `api/` - API接口文档
- `database/` - 数据库设计
- `business-logic/` - 业务逻辑设计
- `integration/` - 集成测试
- `deployment/` - 部署文档
- `monitoring/` - 监控配置
- `security/` - 安全配置

## 🚀 技术栈

### 后端技术栈
- **语言**: Rust (高性能、内存安全)
- **框架**: Axum (异步Web框架)
- **运行时**: Tokio (异步运行时)
- **数据库**: Sqlx (编译时SQL检查)
- **消息队列**: Tonic (gRPC框架)

### 前端技术栈
- **框架**: Vue 3 (响应式UI框架)
- **状态管理**: Pinia (轻量级状态管理)
- **构建工具**: Vite (快速构建工具)
- **UI组件**: 自定义组件库

### 基础设施
- **容器化**: Docker + Docker Compose
- **数据库**: PostgreSQL, Redis, ClickHouse
- **消息队列**: Kafka
- **监控**: Prometheus + Grafana
- **代理**: Nginx

## 📊 开发进度

- [x] 系统架构设计
- [x] 各服务架构文档
- [x] API接口设计
- [x] 数据模型设计
- [x] 部署架构设计
- [x] 监控体系设计
- [x] 安全体系设计

## 📖 使用说明

1. 从 `废弃-PROJECT_BLUEPRINT.md` 开始了解整体项目规划
2. 查看 `SYSTEM_ARCHITECTURE_OVERVIEW.md` 了解系统架构全景
3. 根据需要查看各服务的具体架构文档
4. 参考各子目录中的详细设计文档

## 🎯 目标

本开发蓝图旨在为量化交易平台的开发提供完整的技术指导，确保开发团队可以按照统一的架构标准进行开发，实现高性能、高可用、高安全的量化交易系统。