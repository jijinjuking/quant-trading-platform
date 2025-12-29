import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useTradingStore } from '@/stores/trading'
import { useMarketStore } from '@/stores/market'
import { useWebSocketStore } from '@/stores/websocket'
import { useUserStore } from '@/stores/user'
import { marketDataApi } from '@/utils/api'

export interface SymbolData {
  symbol: string
  price: number
  change: number
  volume: number
  quoteVolume: number
  high: number
  low: number
  open: number
}

export function useTradingData() {
  // 状态管理
  const tradingStore = useTradingStore()
  const marketStore = useMarketStore()
  const wsStore = useWebSocketStore()
  const userStore = useUserStore()

  // 响应式数据
  const selectedSymbol = ref('BTCUSDT')
  const selectedInterval = ref('1m')
  const currentPrice = ref(0)
  const priceChange = ref(0)
  const high24h = ref(0)
  const low24h = ref(0)
  const volume24h = ref(0)
  const quoteVolume24h = ref(0)
  const availableBalance = ref(10000)
  const connectionStatus = ref<'connected' | 'connecting' | 'disconnected'>('disconnected')
  const allSymbols = ref<SymbolData[]>([])

  // 订单簿数据
  const mockBids = ref<number[][]>([
    [49950, 0.5, 0.5],
    [49940, 1.2, 1.7],
    [49930, 0.8, 2.5],
    [49920, 2.1, 4.6],
    [49910, 1.5, 6.1]
  ])

  const mockAsks = ref<number[][]>([
    [50050, 0.3, 0.3],
    [50060, 0.9, 1.2],
    [50070, 1.1, 2.3],
    [50080, 0.7, 3.0],
    [50090, 2.0, 5.0]
  ])

  const recentTrades = ref([
    { id: 1, time: Date.now(), price: 50000, quantity: 0.1, side: 'buy' as const },
    { id: 2, time: Date.now() - 1000, price: 49950, quantity: 0.2, side: 'sell' as const },
    { id: 3, time: Date.now() - 2000, price: 50100, quantity: 0.05, side: 'buy' as const }
  ])

  // 计算属性
  const priceChangeClass = computed(() => {
    return priceChange.value >= 0 ? 'positive' : 'negative'
  })

  // 获取真实市场数据
  const loadRealMarketData = async () => {
    try {
      // 通过网关获取市场数据
      const response = await marketDataApi.getTickers()
      
      if (response.data) {
        allSymbols.value = response.data.map((item: any) => ({
          symbol: item.symbol,
          price: parseFloat(item.price || item.lastPrice || 0),
          change: parseFloat(item.priceChangePercent || 0),
          volume: parseFloat(item.volume || 0),
          quoteVolume: parseFloat(item.quoteVolume || 0),
          high: parseFloat(item.high || 0),
          low: parseFloat(item.low || 0),
          open: parseFloat(item.open || 0)
        }))
        
        // 更新当前选中交易对的数据
        const currentSymbolData = allSymbols.value.find(s => s.symbol === selectedSymbol.value)
        if (currentSymbolData) {
          currentPrice.value = currentSymbolData.price
          priceChange.value = currentSymbolData.change
          high24h.value = currentSymbolData.high
          low24h.value = currentSymbolData.low
          volume24h.value = currentSymbolData.volume
          quoteVolume24h.value = currentSymbolData.quoteVolume
          
          updateOrderBookPrices(currentSymbolData.price)
          
          console.log(`✅ Updated ${selectedSymbol.value} price: ${currentPrice.value.toLocaleString()}`)
        }
        
        console.log('✅ Loaded market data from API gateway:', allSymbols.value.length, 'symbols')
      }
    } catch (error) {
      console.error('❌ Failed to load market data from API:', error)
      
      // 备用：尝试直接连接市场数据服务
      try {
        console.log('🔄 Trying direct market data connection...')
        const backupResponse = await marketDataApi.getTickersDirect()
        
        if (backupResponse.data) {
          allSymbols.value = backupResponse.data.map((item: any) => ({
            symbol: item.symbol,
            price: parseFloat(item.price || 0),
            change: parseFloat(item.change || 0),
            volume: parseFloat(item.volume || 0),
            quoteVolume: parseFloat(item.quoteVolume || 0),
            high: parseFloat(item.high || 0),
            low: parseFloat(item.low || 0),
            open: parseFloat(item.open || 0)
          }))
          
          console.log('✅ Loaded market data from direct connection')
        }
      } catch (backupError) {
        console.error('❌ Direct connection also failed:', backupError)
        
        // 最后备用：使用模拟数据
        loadMockMarketData()
      }
    }
  }

  // 加载模拟市场数据（备用）
  const loadMockMarketData = () => {
    console.log('📊 Loading mock market data as fallback')
    allSymbols.value = [
      { symbol: 'BTCUSDT', price: 50000, change: 2.5, volume: 1000000, quoteVolume: 50000000000, high: 52000, low: 48000, open: 49000 },
      { symbol: 'ETHUSDT', price: 3000, change: -1.2, volume: 500000, quoteVolume: 1500000000, high: 3100, low: 2950, open: 3050 },
      { symbol: 'BNBUSDT', price: 300, change: 3.8, volume: 200000, quoteVolume: 60000000, high: 310, low: 290, open: 295 },
      { symbol: 'ADAUSDT', price: 0.5, change: 5.8, volume: 2000000, quoteVolume: 1000000, high: 0.52, low: 0.47, open: 0.48 },
      { symbol: 'SOLUSDT', price: 100, change: -2.1, volume: 300000, quoteVolume: 30000000, high: 105, low: 98, open: 102 }
    ]
    
    // 设置默认价格
    const btcData = allSymbols.value.find(s => s.symbol === 'BTCUSDT')
    if (btcData) {
      currentPrice.value = btcData.price
      priceChange.value = btcData.change
      high24h.value = btcData.high
      low24h.value = btcData.low
      volume24h.value = btcData.volume
      quoteVolume24h.value = btcData.quoteVolume
    }
  }

  // 更新订单簿价格
  const updateOrderBookPrices = (basePrice: number) => {
    // 生成基于真实价格的买单
    mockBids.value = Array.from({ length: 10 }, (_, i) => {
      const priceOffset = (i + 1) * 0.001
      const price = basePrice * (1 - priceOffset)
      const quantity = Math.random() * 2 + 0.1
      return [price, quantity, quantity * (i + 1)]
    })
    
    // 生成基于真实价格的卖单
    mockAsks.value = Array.from({ length: 10 }, (_, i) => {
      const priceOffset = (i + 1) * 0.001
      const price = basePrice * (1 + priceOffset)
      const quantity = Math.random() * 2 + 0.1
      return [price, quantity, quantity * (i + 1)]
    })
  }

  // 加载用户交易数据
  const loadTradingData = async () => {
    if (!userStore.isAuthenticated) return
    
    try {
      await Promise.all([
        tradingStore.loadUserData(),
        marketStore.loadMarketData()
      ])
      
      if (tradingStore.account) {
        availableBalance.value = tradingStore.balance
      }
    } catch (error) {
      console.error('Failed to load trading data:', error)
    }
  }

  // 连接WebSocket
  const connectWebSocket = async () => {
    try {
      connectionStatus.value = 'connecting'
      
      await wsStore.connect('ws://localhost:8080')
      connectionStatus.value = 'connected'
      
      // 订阅市场数据
      wsStore.subscribe('ticker', selectedSymbol.value, (data: any) => {
        currentPrice.value = data.price || currentPrice.value
        priceChange.value = data.priceChange || priceChange.value
        volume24h.value = data.volume || volume24h.value
      })
      
      wsStore.subscribe('orderbook', selectedSymbol.value, (data: any) => {
        if (data.bids) mockBids.value = data.bids.slice(0, 10)
        if (data.asks) mockAsks.value = data.asks.slice(0, 10)
      })
      
      wsStore.subscribe('trade', selectedSymbol.value, (data: any) => {
        recentTrades.value.unshift(data)
        if (recentTrades.value.length > 50) {
          recentTrades.value = recentTrades.value.slice(0, 50)
        }
      })
      
    } catch (error) {
      console.error('Failed to connect to WebSocket:', error)
      connectionStatus.value = 'disconnected'
    }
  }

  // 断开WebSocket
  const disconnectWebSocket = () => {
    if (wsStore.disconnect) {
      wsStore.disconnect()
    }
    connectionStatus.value = 'disconnected'
  }

  return {
    // 状态
    selectedSymbol,
    selectedInterval,
    currentPrice,
    priceChange,
    high24h,
    low24h,
    volume24h,
    quoteVolume24h,
    availableBalance,
    connectionStatus,
    allSymbols,
    mockBids,
    mockAsks,
    recentTrades,
    
    // 计算属性
    priceChangeClass,
    
    // 方法
    loadRealMarketData,
    loadTradingData,
    connectWebSocket,
    disconnectWebSocket,
    updateOrderBookPrices,
    
    // 状态管理
    tradingStore,
    marketStore,
    wsStore,
    userStore
  }
}