# 用户认证 API

## 1. 用户登录

### 请求
```http
POST /api/v1/auth/login
Content-Type: application/json
```

### 请求体
```json
{
  "username": "john_doe",
  "password": "your_password"
}
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "user_123",
      "username": "john_doe",
      "email": "john@example.com",
      "role": "trader",
      "created_at": "2024-01-01T00:00:00Z"
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 86400
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

### 错误响应 (401)
```json
{
  "success": false,
  "data": null,
  "error": {
    "code": "AUTHENTICATION_ERROR",
    "message": "用户名或密码错误"
  },
  "timestamp": "2024-12-18T10:30:00Z"
}
```

### 前端调用示例

**JavaScript/Fetch**
```javascript
async function login(username, password) {
  const response = await fetch('http://localhost:8091/api/v1/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json'
    },
    body: JSON.stringify({ username, password })
  });
  
  const result = await response.json();
  
  if (result.success) {
    // 保存token
    localStorage.setItem('token', result.data.token);
    localStorage.setItem('user', JSON.stringify(result.data.user));
    return result.data;
  } else {
    throw new Error(result.error.message);
  }
}
```

**React + Axios**
```javascript
import axios from 'axios';

const login = async (username, password) => {
  try {
    const response = await axios.post('http://localhost:8091/api/v1/auth/login', {
      username,
      password
    });
    
    if (response.data.success) {
      const { token, user } = response.data.data;
      localStorage.setItem('token', token);
      return { token, user };
    }
  } catch (error) {
    console.error('登录失败:', error.response?.data?.error?.message);
    throw error;
  }
};
```

---

## 2. 用户注册

### 请求
```http
POST /api/v1/auth/register
Content-Type: application/json
```

### 请求体
```json
{
  "username": "new_user",
  "email": "user@example.com",
  "password": "secure_password",
  "confirm_password": "secure_password"
}
```

### 成功响应 (201)
```json
{
  "success": true,
  "data": {
    "user": {
      "id": "user_124",
      "username": "new_user",
      "email": "user@example.com",
      "role": "trader",
      "created_at": "2024-12-18T10:30:00Z"
    },
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 86400
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 3. 退出登录

### 请求
```http
POST /api/v1/auth/logout
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "message": "退出成功"
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 4. 刷新令牌

### 请求
```http
POST /api/v1/auth/refresh
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "expires_in": 86400
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 5. 获取当前用户信息

### 请求
```http
GET /api/v1/users/profile
Authorization: Bearer YOUR_JWT_TOKEN
```

### 成功响应 (200)
```json
{
  "success": true,
  "data": {
    "id": "user_123",
    "username": "john_doe",
    "email": "john@example.com",
    "role": "trader",
    "balance": 10000.00,
    "created_at": "2024-01-01T00:00:00Z",
    "last_login": "2024-12-18T10:30:00Z"
  },
  "error": null,
  "timestamp": "2024-12-18T10:30:00Z"
}
```

---

## 认证流程说明

1. **登录获取Token**: 调用 `/api/v1/auth/login` 获取JWT token
2. **保存Token**: 将token保存到localStorage或sessionStorage
3. **携带Token**: 后续所有请求在Header中携带 `Authorization: Bearer TOKEN`
4. **Token过期**: 收到401错误时，调用 `/api/v1/auth/refresh` 刷新token
5. **退出登录**: 调用 `/api/v1/auth/logout` 并清除本地token

## 通用请求拦截器示例

**Axios拦截器**
```javascript
import axios from 'axios';

const api = axios.create({
  baseURL: 'http://localhost:8091'
});

// 请求拦截器 - 自动添加token
api.interceptors.request.use(config => {
  const token = localStorage.getItem('token');
  if (token) {
    config.headers.Authorization = `Bearer ${token}`;
  }
  return config;
});

// 响应拦截器 - 处理token过期
api.interceptors.response.use(
  response => response,
  async error => {
    if (error.response?.status === 401) {
      // Token过期，跳转到登录页
      localStorage.removeItem('token');
      window.location.href = '/login';
    }
    return Promise.reject(error);
  }
);

export default api;
```