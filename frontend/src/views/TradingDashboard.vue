<template>
  <div class="binance-style-dashboard">
    <!-- 确认对话框 -->
    <ConfirmDialog
      v-model:visible="confirmDialog.visible"
      :title="confirmDialog.title"
      :message="confirmDialog.message"
      :type="confirmDialog.type"
      :loading="confirmDialog.loading"
      @confirm="confirmDialog.onConfirm"
      @cancel="confirmDialog.onCancel"
    />

    <!-- 顶部导航栏 -->
    <div class="top-navigation">
      <!-- 左侧：价格信息 -->
      <div class="nav-left">
        <PriceTicker
          :symbol="selectedSymbol"
          :price="currentPrice"
          :price-change="priceChange"
          :high24h="high24h"
          :low24h="low24h"
          :volume24h="volume24h"
          :quote-volume24h="quoteVolume24h"
        />
      </div>
      
      <!-- 右侧：用户信息 -->
      <div class="nav-right">
        <UserHeader />
      </div>
    </div>

    <!-- 主交易区域 -->
    <div class="main-trading-layout">
      <!-- 左侧：市场列表 -->
      <MarketSidebar
        :symbols="filteredSymbols"
        :selected-symbol="selectedSymbol"
        :active-tab="activeMarketTab"
        :search-keyword="searchKeyword"
        @tab-change="activeMarketTab = $event"
        @search="searchKeyword = $event"
        @symbol-select="onSymbolSelect"
      />

      <!-- 中间：图表区域 -->
      <ChartSection
        :symbol="selectedSymbol"
        :price="currentPrice"
        :price-change="priceChange"
        :selected-interval="selectedInterval"
        :connection-status="connectionStatus"
        @interval-change="onIntervalChange"
        @toggle-fullscreen="toggleFullscreen"
        @chart-ready="onChartReady"
      />

      <!-- 右侧：交易和订单簿 -->
      <div class="trading-sidebar">
        <!-- 交易面板 -->
        <TradingPanel
          :symbol="selectedSymbol"
          :current-price="currentPrice"
          :order-type="orderType"
          :order-price="orderPrice"
          :order-quantity="orderQuantity"
          :order-amount="orderAmount"
          :balance="tradingStore.balance || availableBalance"
          :total-pn-l="tradingStore.totalPnL || 0"
          :is-submitting-order="isSubmittingOrder"
          :can-trade="userStore.canTrade"
          @order-type-change="orderType = $event"
          @price-change="onPriceChange"
          @quantity-change="onQuantityChange"
          @amount-change="onAmountChange"
          @percentage-click="onPercentageClick"
          @buy-click="onBuyClick"
          @sell-click="onSellClick"
        />
        
        <!-- 订单簿 -->
        <OrderBook
          :symbol="selectedSymbol"
          :current-price="currentPrice"
          :price-change="priceChange"
          :bids="mockBids"
          :asks="mockAsks"
          @price-click="onOrderBookPriceClick"
        />
      </div>
    </div>

    <!-- 底部：订单和交易历史 -->
    <BottomPanel
      :active-tab="bottomActiveTab"
      :open-orders="tradingStore.openOrders"
      :order-history="tradingStore.orderHistory"
      :recent-trades="recentTrades"
      @tab-change="bottomActiveTab = $event"
      @cancel-order="cancelOrder"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue'
import PriceTicker from '@/components/trading/PriceTicker.vue'
import MarketSidebar from '@/components/trading/MarketSidebar.vue'
import ChartSection from '@/components/trading/ChartSection.vue'
import TradingPanel from '@/components/trading/TradingPanel.vue'
import OrderBook from '@/components/trading/OrderBook.vue'
import BottomPanel from '@/components/trading/BottomPanel.vue'
import UserHeader from '@/components/trading/UserHeader.vue'
import ConfirmDialog from '@/components/common/ConfirmDialog.vue'
import { useTradingData } from '@/composables/useTradingData'

// 使用组合函数
const {
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
  loadRealMarketData,
  loadTradingData,
  connectWebSocket,
  disconnectWebSocket,
  tradingStore,
  userStore
} = useTradingData()

// 本地状态
const activeMarketTab = ref('usdt')
const bottomActiveTab = ref('open-orders')
const searchKeyword = ref('')

// 交易表单数据
const orderType = ref<'LIMIT' | 'MARKET'>('LIMIT')
const orderPrice = ref('')
const orderQuantity = ref('')
const orderAmount = ref('')
const isSubmittingOrder = ref(false)

// 确认对话框
const confirmDialog = ref({
  visible: false,
  title: '确认订单',
  message: '',
  type: 'warning' as 'info' | 'warning' | 'error' | 'success',
  loading: false,
  onConfirm: () => {},
  onCancel: () => {}
})

// 计算属性
const filteredSymbols = computed(() => {
  let symbols = allSymbols.value
  
  if (activeMarketTab.value === 'usdt') {
    symbols = symbols.filter(s => s.symbol.endsWith('USDT'))
  } else if (activeMarketTab.value === 'btc') {
    symbols = symbols.filter(s => s.symbol.endsWith('BTC'))
  } else if (activeMarketTab.value === 'eth') {
    symbols = symbols.filter(s => s.symbol.endsWith('ETH'))
  }
  
  if (searchKeyword.value) {
    symbols = symbols.filter(s => 
      s.symbol.toLowerCase().includes(searchKeyword.value.toLowerCase())
    )
  }
  
  return symbols
})

// 事件处理
const onSymbolSelect = (symbol: string) => {
  selectedSymbol.value = symbol
  const symbolData = allSymbols.value.find(s => s.symbol === symbol)
  if (symbolData) {
    currentPrice.value = symbolData.price
    priceChange.value = symbolData.change
  }
}

const onIntervalChange = (interval: string) => {
  selectedInterval.value = interval
}

const onChartReady = (chart: any) => {
  console.log('Chart ready:', chart)
}

const onOrderBookPriceClick = (price: number) => {
  orderPrice.value = price.toString()
}

const toggleFullscreen = () => {
  if (document.fullscreenElement) {
    document.exitFullscreen()
  } else {
    document.documentElement.requestFullscreen()
  }
}

// 交易功能
const onBuyClick = async () => {
  await submitOrder('BUY')
}

const onSellClick = async () => {
  await submitOrder('SELL')
}

const submitOrder = async (side: 'BUY' | 'SELL') => {
  if (!userStore.isAuthenticated) {
    alert('请先登录')
    return
  }

  if (!userStore.canTrade) {
    alert('账户未完成KYC认证，无法交易')
    return
  }

  // 验证输入
  if (!orderQuantity.value || parseFloat(orderQuantity.value) <= 0) {
    alert('请输入有效的数量')
    return
  }

  if (orderType.value === 'LIMIT' && (!orderPrice.value || parseFloat(orderPrice.value) <= 0)) {
    alert('限价单请输入有效的价格')
    return
  }

  // 确认订单（如果启用）
  const shouldConfirm = userStore.user?.preferences?.trading?.confirmOrders ?? true
  if (shouldConfirm) {
    const orderValue = orderType.value === 'LIMIT' 
      ? parseFloat(orderPrice.value) * parseFloat(orderQuantity.value)
      : currentPrice.value * parseFloat(orderQuantity.value)
    
    const confirmMessage = `交易对: ${selectedSymbol.value}
类型: ${orderType.value === 'LIMIT' ? '限价单' : '市价单'}
${orderType.value === 'LIMIT' ? `价格: ${orderPrice.value} USDT` : ''}
数量: ${orderQuantity.value}
预估金额: ${orderValue.toFixed(2)} USDT`
    
    // 使用专业对话框
    const confirmed = await new Promise<boolean>((resolve) => {
      confirmDialog.value = {
        visible: true,
        title: `确认${side === 'BUY' ? '买入' : '卖出'}订单`,
        message: confirmMessage,
        type: 'warning',
        loading: false,
        onConfirm: () => {
          confirmDialog.value.visible = false
          resolve(true)
        },
        onCancel: () => {
          confirmDialog.value.visible = false
          resolve(false)
        }
      }
    })
    
    if (!confirmed) {
      return
    }
  }

  try {
    isSubmittingOrder.value = true

    const orderRequest = {
      symbol: selectedSymbol.value,
      side: side,
      type: orderType.value,
      quantity: parseFloat(orderQuantity.value),
      ...(orderType.value === 'LIMIT' && { price: parseFloat(orderPrice.value) }),
      timeInForce: 'GTC' as const
    }

    console.log('Submitting order to backend:', orderRequest)

    const result = await tradingStore.submitOrder(orderRequest)
    
    if (result) {
      // 播放提示音（如果启用）
      const soundEnabled = userStore.user?.preferences?.trading?.soundEnabled ?? false
      if (soundEnabled) {
        playOrderSound()
      }
      
      // 使用专业对话框显示成功
      const successMessage = `订单ID: ${result.id}
交易对: ${result.symbol}
方向: ${result.side === 'BUY' ? '买入' : '卖出'}
类型: ${result.type}
数量: ${result.quantity}
${result.price ? `价格: ${result.price} USDT` : ''}
状态: ${result.status}`

      confirmDialog.value = {
        visible: true,
        title: '订单提交成功',
        message: successMessage,
        type: 'success',
        loading: false,
        onConfirm: () => {
          confirmDialog.value.visible = false
        },
        onCancel: () => {
          confirmDialog.value.visible = false
        }
      }
      
      // 清空表单
      orderQuantity.value = ''
      orderAmount.value = ''
      if (orderType.value === 'MARKET') {
        orderPrice.value = ''
      }
      
      // 刷新数据
      await Promise.all([
        tradingStore.refreshOrders(),
        tradingStore.loadUserData()
      ])
    }
  } catch (error: any) {
    console.error('❌ Order submission failed:', error)
    
    // 更详细的错误提示
    let errorMessage = '订单提交失败'
    let errorDetail = ''
    
    if (error.response) {
      // 服务器返回错误
      const status = error.response.status
      errorMessage = `服务器错误 (${status})`
      
      if (error.response.data?.message) {
        errorDetail = error.response.data.message
      } else if (error.response.data?.error) {
        errorDetail = error.response.data.error
      } else if (status === 500) {
        errorDetail = '服务器内部错误，请稍后重试或联系管理员'
      } else if (status === 401) {
        errorDetail = '未授权，请重新登录'
      } else if (status === 403) {
        errorDetail = '没有权限执行此操作'
      } else if (status === 404) {
        errorDetail = '请求的资源不存在'
      }
    } else if (error.request) {
      // 请求发送但没有响应
      errorMessage = '网络错误'
      errorDetail = '无法连接到服务器，请检查网络连接或后端服务是否启动'
    } else if (error.message) {
      errorDetail = error.message
    }
    
    // 使用专业对话框显示错误
    confirmDialog.value = {
      visible: true,
      title: errorMessage,
      message: errorDetail || '未知错误',
      type: 'error',
      loading: false,
      onConfirm: () => {
        confirmDialog.value.visible = false
      },
      onCancel: () => {
        confirmDialog.value.visible = false
      }
    }
  } finally {
    isSubmittingOrder.value = false
  }
}

// 播放订单提示音
const playOrderSound = () => {
  try {
    const audio = new Audio('/sounds/order-success.mp3')
    audio.volume = 0.3
    audio.play().catch(err => console.log('Audio play failed:', err))
  } catch (error) {
    console.log('Failed to play sound:', error)
  }
}

const onPercentageClick = (percentage: number) => {
  if (!availableBalance.value || !currentPrice.value) return
  
  const maxAmount = availableBalance.value * (percentage / 100)
  const quantity = maxAmount / currentPrice.value
  
  orderQuantity.value = quantity.toFixed(6)
  orderAmount.value = maxAmount.toFixed(2)
}

const onQuantityChange = (value: string) => {
  orderQuantity.value = value
  if (value && currentPrice.value) {
    const amount = parseFloat(value) * currentPrice.value
    orderAmount.value = amount.toFixed(2)
  }
}

const onAmountChange = (value: string) => {
  orderAmount.value = value
  if (value && currentPrice.value) {
    const quantity = parseFloat(value) / currentPrice.value
    orderQuantity.value = quantity.toFixed(6)
  }
}

const onPriceChange = (value: string) => {
  orderPrice.value = value
  if (orderQuantity.value && value) {
    const amount = parseFloat(orderQuantity.value) * parseFloat(value)
    orderAmount.value = amount.toFixed(2)
  }
}

const cancelOrder = async (orderId: string) => {
  try {
    await tradingStore.cancelOrder(orderId)
    
    // 成功提示
    confirmDialog.value = {
      visible: true,
      title: '订单取消成功',
      message: `订单 ${orderId} 已成功取消`,
      type: 'success',
      loading: false,
      onConfirm: () => {
        confirmDialog.value.visible = false
      },
      onCancel: () => {
        confirmDialog.value.visible = false
      }
    }
    
    await tradingStore.refreshOrders()
  } catch (error: any) {
    console.error('Cancel order failed:', error)
    
    // 错误提示
    confirmDialog.value = {
      visible: true,
      title: '取消订单失败',
      message: error.message || '未知错误',
      type: 'error',
      loading: false,
      onConfirm: () => {
        confirmDialog.value.visible = false
      },
      onCancel: () => {
        confirmDialog.value.visible = false
      }
    }
  }
}

// 监听器
watch(() => userStore.isAuthenticated, async (isAuth) => {
  if (isAuth) {
    await loadTradingData()
  }
})

watch(selectedSymbol, () => {
  orderPrice.value = currentPrice.value.toString()
})

// 生命周期
onMounted(async () => {
  console.log('Trading dashboard mounted')
  
  await loadRealMarketData()
  
  if (userStore.isAuthenticated) {
    await loadTradingData()
  }
  
  await connectWebSocket()
  
  // 定期刷新数据
  setInterval(loadRealMarketData, 1000)
})

onUnmounted(() => {
  console.log('Trading dashboard unmounted')
  disconnectWebSocket()
})
</script>

<style lang="scss" scoped>
.binance-style-dashboard {
  height: 100vh;
  display: flex;
  flex-direction: column;
  background: #0b0e11;
  color: #eaecef;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  font-size: 12px;
}

.top-navigation {
  height: 60px;
  background: #1e2329;
  border-bottom: 1px solid #2b3139;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  
  .nav-left {
    flex: 1;
    min-width: 0;
  }
  
  .nav-right {
    flex-shrink: 0;
    margin-left: 16px;
  }
}

.main-trading-layout {
  flex: 1;
  display: flex;
  overflow: hidden;
}

.trading-sidebar {
  width: 320px;
  background: #1e2329;
  border-left: 1px solid #2b3139;
  display: flex;
  flex-direction: column;
}

@media (max-width: 1200px) {
  .trading-sidebar {
    width: 280px;
  }
}
</style>