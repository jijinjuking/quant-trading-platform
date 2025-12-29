<template>
  <div class="orders-page">
    <div class="page-container">
      <div class="page-header">
        <h1>订单管理</h1>
        <p>查看和管理您的交易订单</p>
      </div>
      
      <div class="orders-content">
        <div class="orders-tabs">
          <button 
            v-for="tab in tabs"
            :key="tab.key"
            :class="['tab-btn', { active: activeTab === tab.key }]"
            @click="activeTab = tab.key"
          >
            {{ tab.label }}
          </button>
        </div>
        
        <div class="orders-table">
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
            <div v-if="filteredOrders.length === 0" class="empty-state">
              <span>暂无订单数据</span>
            </div>
            <div 
              v-for="order in filteredOrders"
              :key="order.id"
              class="order-row"
            >
              <span class="time">{{ formatTime(order.time) }}</span>
              <span class="symbol">{{ order.symbol }}</span>
              <span class="type">{{ order.type }}</span>
              <span class="side" :class="order.side === 'BUY' ? 'positive' : 'negative'">
                {{ order.side === 'BUY' ? '买入' : '卖出' }}
              </span>
              <span class="quantity">{{ order.quantity }}</span>
              <span class="price">{{ order.price || '-' }}</span>
              <span class="filled">{{ order.filled }}</span>
              <span class="status" :class="getStatusClass(order.status)">
                {{ getStatusText(order.status) }}
              </span>
              <span class="actions">
                <button 
                  v-if="order.status === 'NEW' || order.status === 'PARTIALLY_FILLED'"
                  class="cancel-btn"
                  @click="cancelOrder(order.id)"
                >
                  取消
                </button>
                <span v-else>-</span>
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

const activeTab = ref('all')

const tabs = [
  { key: 'all', label: '全部订单' },
  { key: 'open', label: '当前委托' },
  { key: 'filled', label: '已成交' },
  { key: 'canceled', label: '已取消' }
]

// 模拟订单数据
const orders = ref([
  {
    id: '1',
    time: Date.now() - 3600000,
    symbol: 'BTCUSDT',
    type: 'LIMIT',
    side: 'BUY',
    quantity: '0.1',
    price: '50000',
    filled: '0.05',
    status: 'PARTIALLY_FILLED'
  },
  {
    id: '2',
    time: Date.now() - 7200000,
    symbol: 'ETHUSDT',
    type: 'MARKET',
    side: 'SELL',
    quantity: '1.0',
    price: null,
    filled: '1.0',
    status: 'FILLED'
  },
  {
    id: '3',
    time: Date.now() - 10800000,
    symbol: 'BNBUSDT',
    type: 'LIMIT',
    side: 'BUY',
    quantity: '10',
    price: '300',
    filled: '0',
    status: 'CANCELED'
  }
])

const filteredOrders = computed(() => {
  if (activeTab.value === 'all') return orders.value
  
  const statusMap = {
    'open': ['NEW', 'PARTIALLY_FILLED'],
    'filled': ['FILLED'],
    'canceled': ['CANCELED', 'REJECTED']
  }
  
  const targetStatuses = statusMap[activeTab.value as keyof typeof statusMap] || []
  return orders.value.filter(order => targetStatuses.includes(order.status))
})

const formatTime = (timestamp: number) => {
  return new Date(timestamp).toLocaleString('zh-CN')
}

const getStatusText = (status: string) => {
  const statusMap = {
    'NEW': '新建',
    'PARTIALLY_FILLED': '部分成交',
    'FILLED': '完全成交',
    'CANCELED': '已取消',
    'REJECTED': '已拒绝'
  }
  return statusMap[status as keyof typeof statusMap] || status
}

const getStatusClass = (status: string) => {
  return {
    'status-new': status === 'NEW',
    'status-partial': status === 'PARTIALLY_FILLED',
    'status-filled': status === 'FILLED',
    'status-canceled': status === 'CANCELED' || status === 'REJECTED'
  }
}

const cancelOrder = (orderId: string) => {
  const order = orders.value.find(o => o.id === orderId)
  if (order) {
    order.status = 'CANCELED'
  }
}
</script>

<style lang="scss" scoped>
.orders-page {
  min-height: 100vh;
  background: #0b0e11;
  color: #eaecef;
  padding: 20px;
  
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .page-header {
    margin-bottom: 32px;
    
    h1 {
      font-size: 28px;
      font-weight: 600;
      margin: 0 0 8px 0;
    }
    
    p {
      color: #848e9c;
      margin: 0;
    }
  }
  
  .orders-content {
    background: #1e2329;
    border-radius: 8px;
    
    .orders-tabs {
      display: flex;
      border-bottom: 1px solid #2b3139;
      
      .tab-btn {
        padding: 16px 24px;
        background: transparent;
        border: none;
        color: #848e9c;
        font-size: 14px;
        cursor: pointer;
        transition: all 0.2s;
        
        &:hover {
          color: #eaecef;
        }
        
        &.active {
          color: #f0b90b;
          border-bottom: 2px solid #f0b90b;
        }
      }
    }
    
    .orders-table {
      .table-header {
        display: grid;
        grid-template-columns: 120px 100px 80px 80px 100px 100px 100px 80px 80px;
        padding: 16px 24px;
        background: #2b3139;
        font-size: 12px;
        color: #848e9c;
        font-weight: 600;
      }
      
      .table-body {
        .empty-state {
          display: flex;
          justify-content: center;
          align-items: center;
          height: 200px;
          color: #848e9c;
        }
        
        .order-row {
          display: grid;
          grid-template-columns: 120px 100px 80px 80px 100px 100px 100px 80px 80px;
          padding: 12px 24px;
          border-bottom: 1px solid #2b3139;
          font-size: 12px;
          
          &:last-child {
            border-bottom: none;
          }
          
          .side {
            &.positive {
              color: #02c076;
            }
            
            &.negative {
              color: #f84960;
            }
          }
          
          .status {
            &.status-new {
              color: #f0b90b;
            }
            
            &.status-partial {
              color: #02c076;
            }
            
            &.status-filled {
              color: #02c076;
            }
            
            &.status-canceled {
              color: #848e9c;
            }
          }
          
          .cancel-btn {
            padding: 2px 8px;
            background: #f84960;
            border: none;
            border-radius: 2px;
            color: #fff;
            font-size: 10px;
            cursor: pointer;
            
            &:hover {
              background: #e63946;
            }
          }
        }
      }
    }
  }
}
</style>