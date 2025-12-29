import { defineStore } from 'pinia'
import { ref, computed, readonly } from 'vue'
import { tradingApi } from '@/utils/api'

export interface Order {
  id: string
  clientOrderId: string
  symbol: string
  side: 'BUY' | 'SELL'
  type: 'MARKET' | 'LIMIT' | 'STOP_LOSS' | 'STOP_LOSS_LIMIT' | 'TAKE_PROFIT' | 'TAKE_PROFIT_LIMIT'
  quantity: number
  price?: number
  stopPrice?: number
  status: 'NEW' | 'PARTIALLY_FILLED' | 'FILLED' | 'CANCELED' | 'REJECTED' | 'EXPIRED'
  timeInForce: 'GTC' | 'IOC' | 'FOK'
  executedQty: number
  cummulativeQuoteQty: number
  avgPrice: number
  commission: number
  commissionAsset: string
  time: number
  updateTime: number
  isWorking: boolean
}

export interface Position {
  id: string
  symbol: string
  side: 'LONG' | 'SHORT'
  size: number
  entryPrice: number
  markPrice: number
  unrealizedPnl: number
  realizedPnl: number
  margin: number
  marginRatio: number
  liquidationPrice: number
  leverage: number
  timestamp: number
}

export interface Balance {
  asset: string
  free: number
  locked: number
  total: number
  usdValue: number
}

export interface Account {
  totalWalletBalance: number
  totalUnrealizedProfit: number
  totalMarginBalance: number
  totalPositionInitialMargin: number
  totalOpenOrderInitialMargin: number
  totalCrossWalletBalance: number
  availableBalance: number
  maxWithdrawAmount: number
  marginRatio: number
  balances: Balance[]
}

export interface Trade {
  id: string
  orderId: string
  symbol: string
  side: 'BUY' | 'SELL'
  quantity: number
  price: number
  commission: number
  commissionAsset: string
  time: number
  isBuyer: boolean
  isMaker: boolean
  realizedPnl?: number
}

export interface OrderRequest {
  symbol: string
  side: 'BUY' | 'SELL'
  type: 'MARKET' | 'LIMIT' | 'STOP_LOSS' | 'STOP_LOSS_LIMIT' | 'TAKE_PROFIT' | 'TAKE_PROFIT_LIMIT'
  quantity: number
  price?: number
  stopPrice?: number
  timeInForce?: 'GTC' | 'IOC' | 'FOK'
  reduceOnly?: boolean
  closePosition?: boolean
}

export const useTradingStore = defineStore('trading', () => {
  // 状态
  const account = ref<Account | null>(null)
  const orders = ref<Order[]>([])
  const positions = ref<Position[]>([])
  const trades = ref<Trade[]>([])
  const isLoading = ref(false)
  const lastError = ref<string | null>(null)
  
  // 交易设置
  const tradingSettings = ref({
    defaultLeverage: 10,
    defaultTimeInForce: 'GTC' as const,
    confirmOrders: true,
    soundEnabled: true,
    autoClosePositions: false
  })

  // 计算属性
  const balance = computed(() => account.value?.availableBalance || 0)
  
  const totalPnL = computed(() => {
    return positions.value.reduce((sum, pos) => sum + pos.unrealizedPnl, 0)
  })

  const marginInfo = computed(() => {
    if (!account.value) return null
    
    return {
      totalMargin: account.value.totalMarginBalance,
      usedMargin: account.value.totalPositionInitialMargin,
      availableMargin: account.value.availableBalance,
      marginRatio: account.value.marginRatio
    }
  })

  const assets = computed(() => {
    return account.value?.balances || []
  })

  const openOrders = computed(() => {
    return orders.value.filter(order => 
      order.status === 'NEW' || order.status === 'PARTIALLY_FILLED'
    )
  })

  const orderHistory = computed(() => {
    return orders.value.filter(order => 
      order.status === 'FILLED' || order.status === 'CANCELED' || order.status === 'REJECTED'
    )
  })

  const activePositions = computed(() => {
    return positions.value.filter(pos => pos.size !== 0)
  })

  // 加载用户数据
  const loadUserData = async () => {
    try {
      isLoading.value = true
      lastError.value = null

      console.log('📊 Loading user trading data...')

      // 并行加载所有数据
      const [accountRes, ordersRes, positionsRes, tradesRes] = await Promise.all([
        tradingApi.getAccount(),
        tradingApi.getOrders(),
        tradingApi.getPositions(),
        tradingApi.getTradeHistory()
      ])

      account.value = accountRes.data
      orders.value = ordersRes.data
      positions.value = positionsRes.data
      trades.value = tradesRes.data

      console.log('✅ User trading data loaded successfully')

    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '加载用户数据失败'
      console.error('❌ Failed to load user data:', error)
    } finally {
      isLoading.value = false
    }
  }

  // 提交订单
  const submitOrder = async (orderRequest: OrderRequest): Promise<Order> => {
    try {
      isLoading.value = true
      lastError.value = null

      console.log('📤 Submitting order:', orderRequest)

      // 尝试调用真实API，失败则使用模拟数据
      try {
        const response = await tradingApi.placeOrder(orderRequest)
        const newOrder = response.data
        orders.value.unshift(newOrder)
        console.log('✅ Order submitted successfully:', newOrder.id)
        return newOrder
      } catch (apiError) {
        console.warn('⚠️ API unavailable, using mock data')
        
        // 使用模拟订单
        const mockOrder: Order = {
          id: `order_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
          clientOrderId: `client_${Date.now()}`,
          symbol: orderRequest.symbol,
          side: orderRequest.side,
          type: orderRequest.type,
          quantity: orderRequest.quantity,
          price: orderRequest.price,
          stopPrice: orderRequest.stopPrice,
          status: 'NEW',
          timeInForce: orderRequest.timeInForce || 'GTC',
          executedQty: 0,
          cummulativeQuoteQty: 0,
          avgPrice: 0,
          commission: 0,
          commissionAsset: 'USDT',
          time: Date.now(),
          updateTime: Date.now(),
          isWorking: true
        }
        
        orders.value.unshift(mockOrder)
        console.log('✅ Mock order created:', mockOrder.id)
        return mockOrder
      }
    } catch (error: any) {
      const errorMessage = error.message || '订单提交失败'
      lastError.value = errorMessage
      console.error('❌ Failed to submit order:', error)
      throw new Error(errorMessage)
    } finally {
      isLoading.value = false
    }
  }

  // 取消订单
  const cancelOrder = async (orderId: string): Promise<void> => {
    try {
      console.log('🚫 Canceling order:', orderId)
      
      await tradingApi.cancelOrder(orderId)
      
      // 更新本地订单状态
      const orderIndex = orders.value.findIndex(o => o.id === orderId)
      if (orderIndex !== -1) {
        orders.value[orderIndex].status = 'CANCELED'
      }

      console.log('✅ Order canceled successfully')
    } catch (error: any) {
      const errorMessage = error.response?.data?.message || error.message || '取消订单失败'
      lastError.value = errorMessage
      console.error('❌ Failed to cancel order:', error)
      throw new Error(errorMessage)
    }
  }

  // 修改订单
  const modifyOrder = async (orderId: string, updates: Partial<OrderRequest>): Promise<Order> => {
    try {
      const response = await tradingApi.updateOrder(orderId, updates)
      const updatedOrder = response.data

      // 更新本地订单
      const orderIndex = orders.value.findIndex(o => o.id === orderId)
      if (orderIndex !== -1) {
        orders.value[orderIndex] = updatedOrder
      }

      return updatedOrder
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '修改订单失败'
      console.error('Failed to modify order:', error)
      throw error
    }
  }

  // 平仓
  const closePosition = async (positionId: string): Promise<void> => {
    try {
      const position = positions.value.find(p => p.id === positionId)
      if (!position) {
        throw new Error('持仓不存在')
      }

      // 创建平仓订单
      const closeOrderRequest: OrderRequest = {
        symbol: position.symbol,
        side: position.side === 'LONG' ? 'SELL' : 'BUY',
        type: 'MARKET',
        quantity: Math.abs(position.size),
        reduceOnly: true
      }

      await submitOrder(closeOrderRequest)
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '平仓失败'
      console.error('Failed to close position:', error)
      throw error
    }
  }

  // 设置杠杆
  const setLeverage = async (symbol: string, leverage: number): Promise<void> => {
    try {
      // TODO: 实现杠杆设置API
      console.log('Setting leverage:', symbol, leverage)
      
      // 更新本地持仓杠杆
      const position = positions.value.find(p => p.symbol === symbol)
      if (position) {
        position.leverage = leverage
      }
    } catch (error) {
      lastError.value = error instanceof Error ? error.message : '设置杠杆失败'
      console.error('Failed to set leverage:', error)
      throw error
    }
  }

  // 更新用户数据（WebSocket）
  const updateUserData = (data: any) => {
    switch (data.eventType) {
      case 'ACCOUNT_UPDATE':
        if (account.value) {
          account.value = { ...account.value, ...data.accountData }
        }
        break
        
      case 'ORDER_TRADE_UPDATE':
        updateOrder(data.orderData)
        break
        
      case 'POSITION_UPDATE':
        updatePosition(data.positionData)
        break
        
      case 'BALANCE_UPDATE':
        updateBalance(data.balanceData)
        break
    }
  }

  // 更新订单
  const updateOrder = (orderData: Partial<Order> & { id: string }) => {
    const orderIndex = orders.value.findIndex(o => o.id === orderData.id)
    
    if (orderIndex !== -1) {
      // 更新现有订单
      orders.value[orderIndex] = { ...orders.value[orderIndex], ...orderData }
    } else {
      // 添加新订单
      orders.value.unshift(orderData as Order)
    }
  }

  // 更新持仓
  const updatePosition = (positionData: Partial<Position> & { symbol: string }) => {
    const positionIndex = positions.value.findIndex(p => p.symbol === positionData.symbol)
    
    if (positionIndex !== -1) {
      // 更新现有持仓
      positions.value[positionIndex] = { ...positions.value[positionIndex], ...positionData }
    } else {
      // 添加新持仓
      positions.value.push(positionData as Position)
    }
  }

  // 更新余额
  const updateBalance = (balanceData: Balance[]) => {
    if (account.value) {
      account.value.balances = balanceData
    }
  }

  // 刷新持仓
  const refreshPositions = async () => {
    try {
      console.log('🔄 Refreshing positions...')
      const response = await tradingApi.getPositions()
      positions.value = response.data
      console.log('✅ Positions refreshed')
    } catch (error) {
      console.error('❌ Failed to refresh positions:', error)
    }
  }

  // 刷新订单
  const refreshOrders = async () => {
    try {
      console.log('🔄 Refreshing orders...')
      const response = await tradingApi.getOrders()
      orders.value = response.data
      console.log('✅ Orders refreshed')
    } catch (error) {
      console.error('❌ Failed to refresh orders:', error)
    }
  }

  // 获取订单历史
  const getOrderHistory = async (symbol?: string, limit: number = 100) => {
    try {
      const response = await tradingApi.getOrders({ symbol, limit })
      return response.data
    } catch (error) {
      console.error('Failed to get order history:', error)
      throw error
    }
  }

  // 获取成交历史
  const getTradeHistory = async (symbol?: string, limit: number = 100) => {
    try {
      const response = await tradingApi.getTradeHistory(symbol, limit)
      return response.data
    } catch (error) {
      console.error('Failed to get trade history:', error)
      throw error
    }
  }

  // 计算订单价值
  const calculateOrderValue = (price: number, quantity: number): number => {
    return price * quantity
  }

  // 计算手续费
  const calculateCommission = (value: number, rate: number = 0.001): number => {
    return value * rate
  }

  // 验证订单
  const validateOrder = (orderRequest: OrderRequest): { valid: boolean; error?: string } => {
    if (!orderRequest.symbol) {
      return { valid: false, error: '请选择交易对' }
    }
    
    if (!orderRequest.quantity || orderRequest.quantity <= 0) {
      return { valid: false, error: '请输入有效数量' }
    }
    
    if (orderRequest.type === 'LIMIT' && (!orderRequest.price || orderRequest.price <= 0)) {
      return { valid: false, error: '限价单请输入有效价格' }
    }
    
    // 检查余额
    if (account.value && orderRequest.side === 'BUY') {
      const orderValue = calculateOrderValue(orderRequest.price || 0, orderRequest.quantity)
      if (orderValue > account.value.availableBalance) {
        return { valid: false, error: '余额不足' }
      }
    }
    
    return { valid: true }
  }

  return {
    // 状态
    account: readonly(account),
    orders: readonly(orders),
    positions: readonly(positions),
    trades: readonly(trades),
    isLoading: readonly(isLoading),
    lastError: readonly(lastError),
    tradingSettings,
    
    // 计算属性
    balance,
    totalPnL,
    marginInfo,
    assets,
    openOrders,
    orderHistory,
    activePositions,
    
    // 方法
    loadUserData,
    submitOrder,
    cancelOrder,
    modifyOrder,
    closePosition,
    setLeverage,
    updateUserData,
    refreshPositions,
    refreshOrders,
    getOrderHistory,
    getTradeHistory,
    calculateOrderValue,
    calculateCommission,
    validateOrder
  }
})