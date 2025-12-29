<template>
  <div class="bottom-panel">
    <div class="panel-tabs-bottom">
      <button 
        v-for="tab in bottomTabs"
        :key="tab.key"
        :class="['bottom-tab', { active: activeTab === tab.key }]"
        @click="$emit('tab-change', tab.key)"
      >
        {{ tab.label }}
      </button>
    </div>
    
    <div class="bottom-content">
      <div v-if="activeTab === 'open-orders'" class="orders-table">
        <div class="table-header">
          <span>时间</span>
          <span>交易对</span>
          <span>类型</span>
          <span>方向</span>
          <span>数量</span>
          <span>价格</span>
          <span>成交</span>
          <span>状态</span>
          <span>操作</span>
        </div>
        <div class="table-body">
          <div v-if="openOrders.length === 0" class="empty-state">
            <span>暂无当前委托</span>
          </div>
          <div 
            v-for="order in openOrders"
            :key="order.id"
            class="order-row"
          >
            <span class="time">{{ formatTime(order.time) }}</span>
            <span class="symbol">{{ order.symbol }}</span>
            <span class="type">{{ order.type }}</span>
            <span class="side" :class="order.side === 'BUY' ? 'positive' : 'negative'">
              {{ order.side === 'BUY' ? '买入' : '卖出' }}
            </span>
            <span class="quantity">{{ formatQuantity(order.quantity) }}</span>
            <span class="price">{{ order.price ? formatPrice(order.price) : '-' }}</span>
            <span class="filled">{{ formatQuantity(order.executedQty) }}</span>
            <span class="status">{{ getOrderStatusText(order.status) }}</span>
            <span class="actions">
              <button 
                class="cancel-btn"
                @click="$emit('cancel-order', order.id)"
                v-if="order.status === 'NEW' || order.status === 'PARTIALLY_FILLED'"
              >
                取消
              </button>
            </span>
          </div>
        </div>
      </div>
      
      <div v-if="activeTab === 'order-history'" class="orders-table">
        <div class="table-header">
          <span>时间</span>
          <span>交易对</span>
          <span>类型</span>
          <span>方向</span>
          <span>数量</span>
          <span>价格</span>
          <span>成交</span>
          <span>状态</span>
        </div>
        <div class="table-body">
          <div v-if="orderHistory.length === 0" class="empty-state">
            <span>暂无历史订单</span>
          </div>
          <div 
            v-for="order in orderHistory"
            :key="order.id"
            class="order-row"
          >
            <span class="time">{{ formatTime(order.time) }}</span>
            <span class="symbol">{{ order.symbol }}</span>
            <span class="type">{{ order.type }}</span>
            <span class="side" :class="order.side === 'BUY' ? 'positive' : 'negative'">
              {{ order.side === 'BUY' ? '买入' : '卖出' }}
            </span>
            <span class="quantity">{{ formatQuantity(order.quantity) }}</span>
            <span class="price">{{ order.price ? formatPrice(order.price) : '-' }}</span>
            <span class="filled">{{ formatQuantity(order.executedQty) }}</span>
            <span class="status">{{ getOrderStatusText(order.status) }}</span>
          </div>
        </div>
      </div>
      
      <div v-if="activeTab === 'trades'" class="trades-table">
        <div class="table-header">
          <span>时间</span>
          <span>价格</span>
          <span>数量</span>
          <span>成交额</span>
        </div>
        <div class="table-body">
          <div 
            v-for="trade in recentTrades"
            :key="trade.id"
            class="trade-row"
          >
            <span class="time">{{ formatTime(trade.time) }}</span>
            <span class="price" :class="trade.side === 'buy' ? 'positive' : 'negative'">
              {{ formatPrice(trade.price) }}
            </span>
            <span class="quantity">{{ formatQuantity(trade.quantity) }}</span>
            <span class="amount">{{ formatPrice(trade.price * trade.quantity) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatPrice, formatQuantity, formatTime } from '@/utils/formatters'

interface Order {
  id: string
  time: number
  symbol: string
  type: string
  side: 'BUY' | 'SELL'
  quantity: number
  price?: number
  executedQty: number
  status: string
}

interface Trade {
  id: number
  time: number
  price: number
  quantity: number
  side: 'buy' | 'sell'
}

interface Props {
  activeTab: string
  openOrders: Order[]
  orderHistory: Order[]
  recentTrades: Trade[]
}

defineProps<Props>()

defineEmits<{
  'tab-change': [tab: string]
  'cancel-order': [orderId: string]
}>()

const bottomTabs = [
  { key: 'open-orders', label: '当前委托' },
  { key: 'order-history', label: '订单历史' },
  { key: 'trades', label: '成交记录' }
]

const getOrderStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'NEW': '新建',
    'PARTIALLY_FILLED': '部分成交',
    'FILLED': '完全成交',
    'CANCELED': '已取消',
    'REJECTED': '已拒绝',
    'EXPIRED': '已过期'
  }
  return statusMap[status] || status
}
</script>

<style lang="scss" scoped>
.bottom-panel {
  height: 200px;
  background: #1e2329;
  border-top: 1px solid #2b3139;
  display: flex;
  flex-direction: column;
  
  .panel-tabs-bottom {
    display: flex;
    background: #2b3139;
    
    .bottom-tab {
      padding: 8px 16px;
      background: transparent;
      border: none;
      color: #848e9c;
      font-size: 12px;
      cursor: pointer;
      transition: all 0.2s;
      
      &.active {
        background: #1e2329;
        color: #eaecef;
        border-bottom: 2px solid #f0b90b;
      }
    }
  }
  
  .bottom-content {
    flex: 1;
    overflow: hidden;
    
    .orders-table, .trades-table {
      height: 100%;
      display: flex;
      flex-direction: column;
      
      .table-header {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(80px, 1fr));
        padding: 8px 16px;
        background: #2b3139;
        font-size: 11px;
        color: #848e9c;
        font-weight: 500;
      }
      
      .table-body {
        flex: 1;
        overflow-y: auto;
        
        .empty-state {
          display: flex;
          justify-content: center;
          align-items: center;
          height: 100px;
          color: #848e9c;
          font-size: 12px;
        }
        
        .trade-row, .order-row {
          display: grid;
          grid-template-columns: repeat(auto-fit, minmax(80px, 1fr));
          padding: 4px 16px;
          font-size: 11px;
          font-family: 'SF Mono', Monaco, monospace;
          border-bottom: 1px solid #2b3139;
          
          .time {
            color: #848e9c;
          }
          
          .price {
            &.positive { color: #02c076; }
            &.negative { color: #f84960; }
          }
          
          .side {
            &.positive { color: #02c076; }
            &.negative { color: #f84960; }
          }
          
          .quantity, .amount, .filled {
            color: #eaecef;
          }
          
          .symbol, .type, .status {
            color: #eaecef;
          }
          
          .actions {
            display: flex;
            gap: 4px;
            
            .cancel-btn {
              padding: 2px 6px;
              background: #f84960;
              border: none;
              border-radius: 2px;
              color: #fff;
              font-size: 10px;
              cursor: pointer;
              transition: background 0.2s;
              
              &:hover {
                background: #e63946;
              }
            }
          }
        }
      }
    }
  }
}

::-webkit-scrollbar {
  width: 6px;
  height: 6px;
}

::-webkit-scrollbar-track {
  background: #2b3139;
}

::-webkit-scrollbar-thumb {
  background: #3c4043;
  border-radius: 3px;
  
  &:hover {
    background: #4a5568;
  }
}
</style>