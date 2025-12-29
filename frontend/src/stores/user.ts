import { defineStore } from 'pinia'
import { ref, computed, readonly } from 'vue'
import axios from 'axios'

export interface User {
  id: string
  username: string
  email: string
  avatar?: string
  role: 'USER' | 'VIP' | 'ADMIN'
  status: 'ACTIVE' | 'SUSPENDED' | 'PENDING'
  createdAt: number
  lastLoginAt: number
  preferences: UserPreferences
  subscription: UserSubscription
  kyc: KYCInfo
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'auto'
  language: 'zh-CN' | 'en-US' | 'ja-JP'
  timezone: string
  currency: string
  notifications: {
    email: boolean
    push: boolean
    sms: boolean
    trading: boolean
    news: boolean
    system: boolean
  }
  trading: {
    confirmOrders: boolean
    soundEnabled: boolean
    defaultLeverage: number
    riskLevel: 'LOW' | 'MEDIUM' | 'HIGH'
    autoClose: boolean
  }
  display: {
    showBalance: boolean
    showPnL: boolean
    compactMode: boolean
    chartType: 'candlestick' | 'line' | 'area'
  }
}

export interface UserSubscription {
  plan: 'FREE' | 'BASIC' | 'PRO' | 'ENTERPRISE'
  status: 'ACTIVE' | 'EXPIRED' | 'CANCELED'
  startDate: number
  endDate: number
  features: string[]
  limits: {
    maxStrategies: number
    maxPositions: number
    apiCallsPerDay: number
    dataRetention: number
  }
}

export interface KYCInfo {
  status: 'NONE' | 'PENDING' | 'APPROVED' | 'REJECTED'
  level: 0 | 1 | 2 | 3
  documents: Array<{
    type: 'ID' | 'PASSPORT' | 'DRIVER_LICENSE' | 'PROOF_OF_ADDRESS'
    status: 'PENDING' | 'APPROVED' | 'REJECTED'
    uploadedAt: number
  }>
  limits: {
    dailyWithdraw: number
    monthlyWithdraw: number
    maxLeverage: number
  }
}

export interface WatchlistItem {
  symbol: string
  addedAt: number
  alerts: Array<{
    id: string
    type: 'PRICE_ABOVE' | 'PRICE_BELOW' | 'VOLUME_SPIKE' | 'CUSTOM'
    value: number
    enabled: boolean
    triggered: boolean
  }>
}

export interface TradingStrategy {
  id: string
  name: string
  description: string
  type: 'AI' | 'MANUAL' | 'COPY'
  status: 'ACTIVE' | 'INACTIVE' | 'PAUSED'
  symbols: string[]
  parameters: Record<string, any>
  performance: {
    totalReturn: number
    winRate: number
    maxDrawdown: number
    sharpeRatio: number
  }
  createdAt: number
  updatedAt: number
}

export interface UserStats {
  totalTrades: number
  totalVolume: number
  totalPnL: number
  winRate: number
  avgHoldTime: number
  bestTrade: number
  worstTrade: number
  tradingDays: number
  favoriteSymbols: string[]
  tradingHours: Record<string, number>
}

export const useUserStore = defineStore('user', () => {
  // 状态
  const user = ref<User | null>(null)
  const watchlist = ref<WatchlistItem[]>([])
  const strategies = ref<TradingStrategy[]>([])
  const userStats = ref<UserStats | null>(null)
  const isAuthenticated = ref(false)
  const isLoading = ref(false)
  const lastError = ref<string | null>(null)

  // 计算属性
  const userRole = computed(() => user.value?.role || 'USER')
  
  const isVIP = computed(() => user.value?.role === 'VIP' || user.value?.role === 'ADMIN')
  
  const isAdmin = computed(() => user.value?.role === 'ADMIN')
  
  const subscriptionPlan = computed(() => user.value?.subscription.plan || 'FREE')
  
  const kycLevel = computed(() => user.value?.kyc.level || 0)
  
  const canTrade = computed(() => {
    return isAuthenticated.value && 
           user.value?.status === 'ACTIVE' && 
           user.value?.kyc.level >= 1
  })
  
  const watchlistSymbols = computed(() => {
    return watchlist.value.map(item => item.symbol)
  })
  
  const activeStrategies = computed(() => {
    return strategies.value.filter(s => s.status === 'ACTIVE')
  })

  // 登录
  const login = async (credentials: { email: string; password: string }): Promise<boolean> => {
    try {
      isLoading.value = true
      lastError.value = null

      // 模拟登录验证
      const testAccounts = {
        'admin@quantnexus.com': { password: 'Admin123456', role: 'ADMIN', username: 'Admin' },
        'user@quantnexus.com': { password: 'User123456', role: 'USER', username: 'User' },
        'trader@quantnexus.com': { password: 'Trader123456', role: 'VIP', username: 'Trader' }
      }

      const account = testAccounts[credentials.email as keyof typeof testAccounts]
      
      if (!account || account.password !== credentials.password) {
        throw new Error('邮箱或密码错误')
      }

      // 模拟API延迟
      await new Promise(resolve => setTimeout(resolve, 1000))

      // 创建模拟用户数据
      const mockUser: User = {
        id: Math.random().toString(36).substring(2, 11),
        username: account.username,
        email: credentials.email,
        avatar: `https://api.dicebear.com/7.x/avataaars/svg?seed=${account.username}`,
        role: account.role as 'USER' | 'VIP' | 'ADMIN',
        status: 'ACTIVE',
        createdAt: Date.now() - 86400000 * 30, // 30天前
        lastLoginAt: Date.now(),
        preferences: {
          theme: 'dark',
          language: 'zh-CN',
          timezone: 'Asia/Shanghai',
          currency: 'USDT',
          notifications: {
            email: true,
            push: true,
            sms: false,
            trading: true,
            news: true,
            system: true
          },
          trading: {
            confirmOrders: true,
            soundEnabled: true,
            defaultLeverage: 10,
            riskLevel: 'MEDIUM',
            autoClose: false
          },
          display: {
            showBalance: true,
            showPnL: true,
            compactMode: false,
            chartType: 'candlestick'
          }
        },
        subscription: {
          plan: account.role === 'ADMIN' ? 'ENTERPRISE' : account.role === 'VIP' ? 'PRO' : 'BASIC',
          status: 'ACTIVE',
          startDate: Date.now() - 86400000 * 30,
          endDate: Date.now() + 86400000 * 365,
          features: ['realtime_data', 'advanced_charts', 'api_access'],
          limits: {
            maxStrategies: account.role === 'ADMIN' ? 999 : account.role === 'VIP' ? 50 : 10,
            maxPositions: account.role === 'ADMIN' ? 999 : account.role === 'VIP' ? 100 : 20,
            apiCallsPerDay: account.role === 'ADMIN' ? 999999 : account.role === 'VIP' ? 10000 : 1000,
            dataRetention: account.role === 'ADMIN' ? 999 : account.role === 'VIP' ? 365 : 90
          }
        },
        kyc: {
          status: account.role === 'ADMIN' ? 'APPROVED' : 'PENDING',
          level: account.role === 'ADMIN' ? 3 : account.role === 'VIP' ? 2 : 1,
          documents: [],
          limits: {
            dailyWithdraw: account.role === 'ADMIN' ? 1000000 : account.role === 'VIP' ? 100000 : 10000,
            monthlyWithdraw: account.role === 'ADMIN' ? 10000000 : account.role === 'VIP' ? 1000000 : 100000,
            maxLeverage: account.role === 'ADMIN' ? 100 : account.role === 'VIP' ? 50 : 20
          }
        }
      }

      // 保存模拟token
      const mockToken = `mock_token_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
      localStorage.setItem('auth_token', mockToken)

      user.value = mockUser
      isAuthenticated.value = true

      // 立即保存用户信息到localStorage以确保持久化
      localStorage.setItem('user_data', JSON.stringify(mockUser))

      // 加载模拟用户数据
      await loadMockUserData()
      
      console.log('✅ Login successful, user authenticated:', isAuthenticated.value)
      return true
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '登录失败'
      console.error('Login failed:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 注册
  const register = async (userData: {
    username: string
    email: string
    password: string
    confirmPassword: string
  }): Promise<boolean> => {
    try {
      isLoading.value = true
      lastError.value = null

      // 模拟注册验证
      if (userData.password !== userData.confirmPassword) {
        throw new Error('两次输入的密码不一致')
      }

      if (userData.password.length < 8) {
        throw new Error('密码长度不能少于8位')
      }

      // 模拟API延迟
      await new Promise(resolve => setTimeout(resolve, 1500))

      // 创建新用户
      const newUser: User = {
        id: Math.random().toString(36).substring(2, 11),
        username: userData.username,
        email: userData.email,
        avatar: `https://api.dicebear.com/7.x/avataaars/svg?seed=${userData.username}`,
        role: 'USER',
        status: 'ACTIVE',
        createdAt: Date.now(),
        lastLoginAt: Date.now(),
        preferences: {
          theme: 'dark',
          language: 'zh-CN',
          timezone: 'Asia/Shanghai',
          currency: 'USDT',
          notifications: {
            email: true,
            push: true,
            sms: false,
            trading: true,
            news: true,
            system: true
          },
          trading: {
            confirmOrders: true,
            soundEnabled: true,
            defaultLeverage: 5,
            riskLevel: 'LOW',
            autoClose: false
          },
          display: {
            showBalance: true,
            showPnL: true,
            compactMode: false,
            chartType: 'candlestick'
          }
        },
        subscription: {
          plan: 'FREE',
          status: 'ACTIVE',
          startDate: Date.now(),
          endDate: Date.now() + 86400000 * 30, // 30天试用
          features: ['basic_charts'],
          limits: {
            maxStrategies: 3,
            maxPositions: 5,
            apiCallsPerDay: 100,
            dataRetention: 30
          }
        },
        kyc: {
          status: 'NONE',
          level: 0,
          documents: [],
          limits: {
            dailyWithdraw: 1000,
            monthlyWithdraw: 10000,
            maxLeverage: 5
          }
        }
      }

      // 保存模拟token
      const mockToken = `mock_token_${Date.now()}_${Math.random().toString(36).substring(2, 11)}`
      localStorage.setItem('auth_token', mockToken)

      user.value = newUser
      isAuthenticated.value = true

      // 立即保存用户信息到localStorage以确保持久化
      localStorage.setItem('user_data', JSON.stringify(newUser))
      
      await loadMockUserData()
      
      console.log('✅ Registration successful, user authenticated:', isAuthenticated.value)
      return true
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '注册失败'
      console.error('Registration failed:', error)
      return false
    } finally {
      isLoading.value = false
    }
  }

  // 登出
  const logout = async (): Promise<void> => {
    try {
      await axios.post('/api/auth/logout')
    } catch (error) {
      console.error('Logout error:', error)
    } finally {
      // 清除本地数据
      user.value = null
      watchlist.value = []
      strategies.value = []
      userStats.value = null
      isAuthenticated.value = false
      
      localStorage.removeItem('auth_token')
      localStorage.removeItem('user_data')
      delete axios.defaults.headers.common['Authorization']
      
      console.log('🚪 User logged out successfully')
    }
  }

  // 检查认证状态
  const checkAuth = async (): Promise<void> => {
    const token = localStorage.getItem('auth_token')
    
    console.log('🔍 CheckAuth called, token exists:', !!token, 'isAuthenticated:', isAuthenticated.value)
    
    if (!token || !token.startsWith('mock_token_')) {
      // 没有有效token，清除认证状态
      console.log('❌ No valid token, clearing auth state')
      user.value = null
      isAuthenticated.value = false
      return
    }

    // 如果已经认证且有用户信息，直接返回
    if (isAuthenticated.value && user.value) {
      console.log('✅ Already authenticated with user data')
      return
    }

    try {
      // 模拟检查token有效性
      await new Promise(resolve => setTimeout(resolve, 50))
      
      // 恢复登录状态
      console.log('🔄 Restoring user session from token')
      
      // 尝试从localStorage恢复用户数据
      const savedUserData = localStorage.getItem('user_data')
      let restoredUser: User
      
      if (savedUserData) {
        try {
          restoredUser = JSON.parse(savedUserData)
          console.log('📦 Restored user from localStorage:', restoredUser.username)
        } catch (error) {
          console.warn('⚠️ Failed to parse saved user data, using default')
          restoredUser = createDefaultUser()
        }
      } else {
        console.log('🔧 No saved user data, creating default user')
        restoredUser = createDefaultUser()
      }
      
      user.value = restoredUser
      isAuthenticated.value = true
      
      // 加载模拟数据
      await loadMockUserData()
      
      console.log('✅ User session restored successfully')
    } catch (error) {
      console.error('❌ Auth check failed:', error)
      await logout()
    }
  }

  // 创建默认用户的辅助函数
  const createDefaultUser = (): User => {
    return {
      id: 'restored_user',
      username: 'Trader',
      email: 'trader@quantnexus.com',
      avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Trader',
      role: 'VIP',
      status: 'ACTIVE',
      createdAt: Date.now() - 86400000 * 30,
      lastLoginAt: Date.now(),
      preferences: {
        theme: 'dark',
        language: 'zh-CN',
        timezone: 'Asia/Shanghai',
        currency: 'USDT',
        notifications: {
          email: true,
          push: true,
          sms: false,
          trading: true,
          news: true,
          system: true
        },
        trading: {
          confirmOrders: true,
          soundEnabled: true,
          defaultLeverage: 10,
          riskLevel: 'MEDIUM',
          autoClose: false
        },
        display: {
          showBalance: true,
          showPnL: true,
          compactMode: false,
          chartType: 'candlestick'
        }
      },
      subscription: {
        plan: 'PRO',
        status: 'ACTIVE',
        startDate: Date.now() - 86400000 * 30,
        endDate: Date.now() + 86400000 * 365,
        features: ['realtime_data', 'advanced_charts', 'api_access'],
        limits: {
          maxStrategies: 50,
          maxPositions: 100,
          apiCallsPerDay: 10000,
          dataRetention: 365
        }
      },
      kyc: {
        status: 'APPROVED',
        level: 2,
        documents: [],
        limits: {
          dailyWithdraw: 100000,
          monthlyWithdraw: 1000000,
          maxLeverage: 50
        }
      }
    }
  }
              chartType: 'candlestick'
            }
          },
          subscription: {
            plan: 'PRO',
            status: 'ACTIVE',
            startDate: Date.now() - 86400000 * 30,
            endDate: Date.now() + 86400000 * 365,
            features: ['realtime_data', 'advanced_charts', 'api_access'],
            limits: {
              maxStrategies: 50,
              maxPositions: 100,
              apiCallsPerDay: 10000,
              dataRetention: 365
            }
          },
          kyc: {
            status: 'APPROVED',
            level: 2,
            documents: [],
            limits: {
              dailyWithdraw: 100000,
              monthlyWithdraw: 1000000,
              maxLeverage: 50
            }
          }
        }
        
        user.value = mockUser
        isAuthenticated.value = true
        
        // 加载模拟数据
        await loadMockUserData()
        
        console.log('✅ User session restored from token')
      }
    } catch (error) {
      console.error('Auth check failed:', error)
      await logout()
    }
  }

  // 加载用户数据
  const loadUserData = async (): Promise<void> => {
    if (!isAuthenticated.value) return

    try {
      const [watchlistRes, strategiesRes, statsRes] = await Promise.all([
        axios.get('/api/user/watchlist'),
        axios.get('/api/user/strategies'),
        axios.get('/api/user/stats')
      ])

      watchlist.value = watchlistRes.data
      strategies.value = strategiesRes.data
      userStats.value = statsRes.data
    } catch (error) {
      console.error('Failed to load user data:', error)
    }
  }

  // 加载模拟用户数据
  const loadMockUserData = async (): Promise<void> => {
    if (!isAuthenticated.value) return

    // 模拟延迟
    await new Promise(resolve => setTimeout(resolve, 500))

    // 模拟自选列表
    watchlist.value = [
      {
        symbol: 'BTCUSDT',
        addedAt: Date.now() - 86400000,
        alerts: []
      },
      {
        symbol: 'ETHUSDT',
        addedAt: Date.now() - 86400000 * 2,
        alerts: []
      },
      {
        symbol: 'BNBUSDT',
        addedAt: Date.now() - 86400000 * 3,
        alerts: []
      }
    ]

    // 模拟策略列表
    strategies.value = [
      {
        id: 'strategy_1',
        name: 'BTC网格策略',
        description: '基于BTC价格波动的网格交易策略',
        type: 'AI',
        status: 'ACTIVE',
        symbols: ['BTCUSDT'],
        parameters: { gridSize: 100, maxOrders: 10 },
        performance: {
          totalReturn: 15.6,
          winRate: 68.5,
          maxDrawdown: -8.2,
          sharpeRatio: 1.45
        },
        createdAt: Date.now() - 86400000 * 7,
        updatedAt: Date.now() - 86400000
      },
      {
        id: 'strategy_2',
        name: 'ETH趋势跟踪',
        description: '基于技术指标的ETH趋势跟踪策略',
        type: 'MANUAL',
        status: 'ACTIVE',
        symbols: ['ETHUSDT'],
        parameters: { ma_period: 20, rsi_threshold: 70 },
        performance: {
          totalReturn: 23.4,
          winRate: 72.1,
          maxDrawdown: -12.5,
          sharpeRatio: 1.78
        },
        createdAt: Date.now() - 86400000 * 14,
        updatedAt: Date.now() - 86400000 * 2
      }
    ]

    // 模拟用户统计
    userStats.value = {
      totalTrades: 156,
      totalVolume: 2450000,
      totalPnL: 18750.50,
      winRate: 65.4,
      avgHoldTime: 4.2,
      bestTrade: 2340.80,
      worstTrade: -890.20,
      tradingDays: 45,
      favoriteSymbols: ['BTCUSDT', 'ETHUSDT', 'BNBUSDT'],
      tradingHours: {
        '0': 12, '1': 8, '2': 5, '3': 3, '4': 2, '5': 1,
        '6': 3, '7': 8, '8': 15, '9': 25, '10': 32, '11': 28,
        '12': 22, '13': 35, '14': 42, '15': 38, '16': 33, '17': 29,
        '18': 25, '19': 22, '20': 28, '21': 35, '22': 28, '23': 18
      }
    }
  }

  // 更新用户信息
  const updateProfile = async (updates: Partial<User>): Promise<void> => {
    try {
      isLoading.value = true
      
      const response = await axios.put('/api/user/profile', updates)
      user.value = response.data
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '更新用户信息失败'
      console.error('Failed to update profile:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 更新偏好设置
  const updatePreferences = async (preferences: Partial<UserPreferences>): Promise<void> => {
    try {
      const response = await axios.put('/api/user/preferences', preferences)
      
      if (user.value) {
        user.value.preferences = { ...user.value.preferences, ...preferences }
      }
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '更新偏好设置失败'
      console.error('Failed to update preferences:', error)
      throw error
    }
  }

  // 添加到自选
  const addToWatchlist = async (symbol: string): Promise<void> => {
    try {
      await axios.post('/api/user/watchlist', { symbol })
      
      const newItem: WatchlistItem = {
        symbol,
        addedAt: Date.now(),
        alerts: []
      }
      
      watchlist.value.push(newItem)
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '添加自选失败'
      console.error('Failed to add to watchlist:', error)
      throw error
    }
  }

  // 从自选移除
  const removeFromWatchlist = async (symbol: string): Promise<void> => {
    try {
      await axios.delete(`/api/user/watchlist/${symbol}`)
      
      const index = watchlist.value.findIndex(item => item.symbol === symbol)
      if (index !== -1) {
        watchlist.value.splice(index, 1)
      }
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '移除自选失败'
      console.error('Failed to remove from watchlist:', error)
      throw error
    }
  }

  // 切换自选状态
  const toggleWatchlist = async (symbol: string): Promise<void> => {
    const isInWatchlist = watchlist.value.some(item => item.symbol === symbol)
    
    if (isInWatchlist) {
      await removeFromWatchlist(symbol)
    } else {
      await addToWatchlist(symbol)
    }
  }

  // 设置价格提醒
  const setAlert = async (symbol: string, alert: {
    type: 'PRICE_ABOVE' | 'PRICE_BELOW' | 'VOLUME_SPIKE' | 'CUSTOM'
    value: number
  }): Promise<void> => {
    try {
      const response = await axios.post(`/api/user/alerts`, { symbol, ...alert })
      const newAlert = response.data
      
      const watchlistItem = watchlist.value.find(item => item.symbol === symbol)
      if (watchlistItem) {
        watchlistItem.alerts.push(newAlert)
      }
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '设置提醒失败'
      console.error('Failed to set alert:', error)
      throw error
    }
  }

  // 删除提醒
  const removeAlert = async (alertId: string): Promise<void> => {
    try {
      await axios.delete(`/api/user/alerts/${alertId}`)
      
      // 从本地数据中移除
      watchlist.value.forEach(item => {
        const index = item.alerts.findIndex(alert => alert.id === alertId)
        if (index !== -1) {
          item.alerts.splice(index, 1)
        }
      })
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '删除提醒失败'
      console.error('Failed to remove alert:', error)
      throw error
    }
  }

  // 上传头像
  const uploadAvatar = async (file: File): Promise<void> => {
    try {
      isLoading.value = true
      
      const formData = new FormData()
      formData.append('avatar', file)
      
      const response = await axios.post('/api/user/avatar', formData, {
        headers: { 'Content-Type': 'multipart/form-data' }
      })
      
      if (user.value) {
        user.value.avatar = response.data.avatarUrl
      }
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '上传头像失败'
      console.error('Failed to upload avatar:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 修改密码
  const changePassword = async (passwords: {
    currentPassword: string
    newPassword: string
    confirmPassword: string
  }): Promise<void> => {
    try {
      isLoading.value = true
      
      await axios.put('/api/user/password', passwords)
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '修改密码失败'
      console.error('Failed to change password:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 获取认证头 - 用于WebSocket连接
  const getAuthToken = (): string | null => {
    return localStorage.getItem('auth_token')
  }

  // 获取认证头字符串
  const getAuthHeader = (): string | null => {
    const token = getAuthToken()
    return token ? `Bearer ${token}` : null
  }

  // KYC认证
  const submitKYC = async (documents: File[]): Promise<void> => {
    try {
      isLoading.value = true
      
      const formData = new FormData()
      documents.forEach((file, index) => {
        formData.append(`document_${index}`, file)
      })
      
      const response = await axios.post('/api/user/kyc', formData, {
        headers: { 'Content-Type': 'multipart/form-data' }
      })
      
      if (user.value) {
        user.value.kyc = response.data
      }
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : 'KYC提交失败'
      console.error('Failed to submit KYC:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  // 获取用户统计
  const refreshStats = async (): Promise<void> => {
    try {
      const response = await axios.get('/api/user/stats')
      userStats.value = response.data
    } catch (error) {
      console.error('Failed to refresh stats:', error)
    }
  }

  return {
    // 状态
    user: readonly(user),
    watchlist: readonly(watchlist),
    strategies: readonly(strategies),
    userStats: readonly(userStats),
    isAuthenticated: readonly(isAuthenticated),
    isLoading: readonly(isLoading),
    lastError: readonly(lastError),
    
    // 计算属性
    userRole,
    isVIP,
    isAdmin,
    subscriptionPlan,
    kycLevel,
    canTrade,
    watchlistSymbols,
    activeStrategies,
    
    // 方法
    login,
    register,
    logout,
    checkAuth,
    loadUserData,
    updateProfile,
    updatePreferences,
    addToWatchlist,
    removeFromWatchlist,
    toggleWatchlist,
    setAlert,
    removeAlert,
    uploadAvatar,
    changePassword,
    submitKYC,
    refreshStats,
    getAuthToken,
    getAuthHeader
  }
})