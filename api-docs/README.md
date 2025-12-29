# 量化交易系统 API 文档

## 基础信息

- **网关地址**: `http://localhost:8091`
- **API版本**: v1
- **认证方式**: JWT Bearer Token
- **数据格式**: JSON

## 统一响应格式

### 成功响应
```json
{
  "success": true,
  "data": {
    // 具体数据
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z",
  "request_id": "req_123456"
}
```

### 错误响应
```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "用户名或密码错误",
    "details": null
  },
  "timestamp": "2024-12-18T10:30:00Z",
  "request_id": "req_123456"
}
```

## 常用错误码

| 错误码 | 说明 |
|--------|------|
| VALIDATION_ERROR | 参数验证失败 |
| AUTHENTICATION_ERROR | 认证失败 |
| AUTHORIZATION_ERROR | 权限不足 |
| NOT_FOUND | 资源不存在 |
| RATE_LIMIT_EXCEEDED | 请求频率超限 |
| INTERNAL_ERROR | 服务器内部错误 |

## API 模块

- [用户认证](./auth/README.md) - 登录、注册、权限管理
- [交易功能](./trading/README.md) - 下单、查询、持仓管理
- [市场数据](./market/README.md) - K线、价格、订单簿
- [策略管理](./strategy/README.md) - 策略创建、回测
- [风险管理](./risk/README.md) - 风控规则、限额管理
- [系统功能](./system/README.md) - 健康检查、监控

## 快速开始

### 1. 获取访问令牌
```bash
curl -X POST http://localhost:8091/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "your_username",
    "password": "your_password"
  }'
```

### 2. 使用令牌访问API
```bash
curl -X GET http://localhost:8091/api/v1/users/profile \
  -H "Authorization: Bearer YOUR_JWT_TOKEN"
```

## 开发环境

- 网关服务: http://localhost:8091
- 用户服务: http://localhost:8085 (内部)
- 交易服务: http://localhost:8083 (内部)
- 市场数据: http://localhost:8081 (内部)

**注意**: 前端应用只需要连接网关(8091)，其他端口为内部服务端口。