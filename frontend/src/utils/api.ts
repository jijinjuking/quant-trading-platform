import axios from 'axios'

// 创建axios实例，配置基础URL指向API网关
const api = axios.create({
  baseURL: 'http://localhost:8091', // 通过网关代理所有请求
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  }
})

// 创建直接连接市场数据服务的实例（用于备用）
const marketDataDirectApi = axios.create({
  baseURL: 'http://localhost:8081',
  timeout: 10000,
  headers: {
    'Content-Type': 'application/json',
  }
})

// 请求拦截器
api.interceptors.request.use(
  (config) => {
    // 添加认证token（如果存在）
    const token = localStorage.getItem('auth_token')
    if (token) {
      config.headers.Authorization = `Bearer ${token}`
    }
    
    // 添加请求ID用于追踪
    config.headers['X-Request-ID'] = generateRequestId()
    
    console.log(`API Request: ${config.method?.toUpperCase()} ${config.url}`)
    return config
  },
  (error) => {
    console.error('Request interceptor error:', error)
    return Promise.reject(error)
  }
)

// 响应拦截器
api.interceptors.response.use(
  (response) => {
    console.log(`API Response: ${response.status} ${response.config.url}`)
    return response
  },
  (error) => {
    console.error('API Error:', error.response?.status, error.response?.data || error.message)
    
    // 处理认证错误
    if (error.response?.status === 401) {
      // 清除token并重定向到登录页
      localStorage.removeItem('auth_token')
      window.location.href = '/login'
    }
    
    // 处理网关错误
    if (error.response?.status === 502 || error.response?.status === 503) {
      console.error('Gateway or service unavailable')
    }
    
    return Promise.reject(error)
  }
)

// 生成请求ID
function generateRequestId(): string {
  return `req_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
}

// 市场数据API - 通过网关代理到市场数据服务
export const marketDataApi = {
  // 获取交易对信息
  getSymbols: () => api.get('/api/v1/market-data/symbols'),
  
  // 获取价格信息
  getTickers: () => api.get('/api/v1/market-data/tickers'),
  
  // 获取单个价格信息
  getTicker: (symbol: string) => api.get(`/api/v1/market-data/ticker/${symbol}`),
  
  // 获取订单簿
  getOrderBook: (symbol: string, limit: number = 20) => 
    api.get(`/api/v1/market-data/orderbook/${symbol}`, { params: { limit } }),
  
  // 获取最新成交
  getTrades: (symbol: string, limit: number = 50) =>
    api.get(`/api/v1/market-data/trades/${symbol}`, { params: { limit } }),
  
  // 获取K线数据
  getKlines: (symbol: string, interval: string, limit: number = 500) =>
    api.get(`/api/v1/market-data/klines/${symbol}`, { params: { interval, limit } }),
  
  // 健康检查
  getHealth: () => api.get('/api/v1/market-data/health'),
  
  // 备用：直接连接市场数据服务
  getTickersDirect: () => marketDataDirectApi.get('/api/v1/tickers'),
  getKlinesDirect: (symbol?: string, interval?: string, limit?: number) =>
    marketDataDirectApi.get('/api/v1/klines', { params: { symbol, interval, limit } })
}

// 交易API - 通过网关代理到交易引擎服务
export const tradingApi = {
  // 获取账户信息
  getAccount: () => api.get('/api/v1/trading-engine/account'),
  
  // 获取余额
  getBalances: () => api.get('/api/v1/trading-engine/balances'),
  
  // 下单
  placeOrder: (orderData: any) => api.post('/api/v1/trading-engine/orders', orderData),
  
  // 取消订单
  cancelOrder: (orderId: string) => api.delete(`/api/v1/trading-engine/orders/${orderId}`),
  
  // 修改订单
  updateOrder: (orderId: string, updates: any) => 
    api.put(`/api/v1/trading-engine/orders/${orderId}`, updates),
  
  // 获取订单列表
  getOrders: (params?: { symbol?: string; status?: string; limit?: number; offset?: number }) => 
    api.get('/api/v1/trading-engine/orders', { params }),
  
  // 获取订单详情
  getOrder: (orderId: string) => api.get(`/api/v1/trading-engine/orders/${orderId}`),
  
  // 批量下单
  batchOrders: (orders: any[]) => api.post('/api/v1/trading-engine/orders/batch', { orders }),
  
  // 获取持仓
  getPositions: () => api.get('/api/v1/trading-engine/positions'),
  
  // 获取交易历史
  getTradeHistory: (symbol?: string, limit: number = 100) =>
    api.get('/api/v1/trading-engine/trades', { params: { symbol, limit } }),
  
  // 健康检查
  getHealth: () => api.get('/api/v1/trading-engine/health')
}

// 策略API
export const strategyApi = {
  // 获取策略列表
  getStrategies: () => api.get('/api/v1/strategy/strategies'),
  
  // 创建策略
  createStrategy: (strategyData: any) => api.post('/api/v1/strategy/strategies', strategyData),
  
  // 更新策略
  updateStrategy: (strategyId: string, strategyData: any) => 
    api.put(`/api/v1/strategy/strategies/${strategyId}`, strategyData),
  
  // 删除策略
  deleteStrategy: (strategyId: string) => api.delete(`/api/v1/strategy/strategies/${strategyId}`),
  
  // 启动策略
  startStrategy: (strategyId: string) => api.post(`/api/v1/strategy/strategies/${strategyId}/start`),
  
  // 停止策略
  stopStrategy: (strategyId: string) => api.post(`/api/v1/strategy/strategies/${strategyId}/stop`),
  
  // 获取策略性能
  getStrategyPerformance: (strategyId: string) => 
    api.get(`/api/v1/strategy/strategies/${strategyId}/performance`),
  
  // 回测策略
  backtestStrategy: (strategyData: any) => api.post('/api/v1/strategy/backtest', strategyData)
}

// 用户API
export const userApi = {
  // 登录
  login: (credentials: { username: string; password: string }) => 
    api.post('/api/v1/auth/login', credentials),
  
  // 注册
  register: (userData: any) => api.post('/api/v1/auth/register', userData),
  
  // 刷新token
  refreshToken: () => api.post('/api/v1/auth/refresh'),
  
  // 登出
  logout: () => api.post('/api/v1/auth/logout'),
  
  // 获取用户信息
  getProfile: () => api.get('/api/v1/user/profile'),
  
  // 更新用户信息
  updateProfile: (profileData: any) => api.put('/api/v1/user/profile', profileData)
}

// 健康检查API
export const healthApi = {
  // 网关健康检查
  gatewayHealth: () => api.get('/health'),
  
  // 服务状态
  serviceStatus: () => api.get('/admin/services'),
  
  // WebSocket统计
  websocketStats: () => api.get('/admin/websocket/stats')
}

export default api