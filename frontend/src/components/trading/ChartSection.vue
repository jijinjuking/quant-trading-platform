<template>
  <div class="chart-section">
    <div class="chart-toolbar">
      <div class="symbol-info-mini">
        <span class="symbol">{{ symbol }}</span>
        <span class="price" :class="priceChangeClass">{{ formatPrice(price) }}</span>
      </div>
      
      <div class="timeframe-selector">
        <button 
          v-for="tf in timeframes"
          :key="tf.value"
          :class="['tf-btn', { active: selectedInterval === tf.value }]"
          @click="$emit('interval-change', tf.value)"
        >
          {{ tf.label }}
        </button>
      </div>
      
      <div class="chart-tools">
        <div class="realtime-indicator">
          <div class="indicator-dot" :class="connectionStatus"></div>
          <span class="indicator-text">
            {{ getConnectionText(connectionStatus) }}
          </span>
        </div>
        
        <button class="tool-btn" @click="$emit('toggle-fullscreen')" title="全屏">
          <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <path d="M1.5 1a.5.5 0 0 0-.5.5v4a.5.5 0 0 1-1 0v-4A1.5 1.5 0 0 1 1.5 0h4a.5.5 0 0 1 0 1h-4zM10 .5a.5.5 0 0 1 .5-.5h4A1.5 1.5 0 0 1 16 1.5v4a.5.5 0 0 1-1 0v-4a.5.5 0 0 0-.5-.5h-4a.5.5 0 0 1-.5-.5zM.5 10a.5.5 0 0 1 .5.5v4a.5.5 0 0 0 .5.5h4a.5.5 0 0 1 0 1h-4A1.5 1.5 0 0 1 0 14.5v-4a.5.5 0 0 1 .5-.5zm15 0a.5.5 0 0 1 .5.5v4a1.5 1.5 0 0 1-1.5 1.5h-4a.5.5 0 0 1 0-1h4a.5.5 0 0 0 .5-.5v-4a.5.5 0 0 1 .5-.5z"/>
          </svg>
        </button>
      </div>
    </div>
    
    <div class="chart-container">
      <TradingViewChart
        :symbol="symbol"
        :interval="selectedInterval"
        theme="dark"
        height="100%"
        @ready="$emit('chart-ready', $event)"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import TradingViewChart from '@/components/charts/TradingViewChart.vue'
import { formatPrice } from '@/utils/formatters'

interface Props {
  symbol: string
  price: number
  priceChange: number
  selectedInterval: string
  connectionStatus: 'connected' | 'connecting' | 'disconnected'
}

const props = defineProps<Props>()

defineEmits<{
  'interval-change': [interval: string]
  'toggle-fullscreen': []
  'chart-ready': [chart: any]
}>()

const timeframes = [
  { label: '1m', value: '1m' },
  { label: '5m', value: '5m' },
  { label: '15m', value: '15m' },
  { label: '30m', value: '30m' },
  { label: '1H', value: '1h' },
  { label: '4H', value: '4h' },
  { label: '1D', value: '1d' }
]

const priceChangeClass = computed(() => {
  return props.priceChange >= 0 ? 'positive' : 'negative'
})

const getConnectionText = (status: string) => {
  const statusMap = {
    'connected': '实时',
    'connecting': '连接中',
    'disconnected': '离线'
  }
  return statusMap[status as keyof typeof statusMap] || '离线'
}
</script>

<style lang="scss" scoped>
.chart-section {
  flex: 1;
  background: #1e2329;
  display: flex;
  flex-direction: column;
  min-width: 0;
  
  .chart-toolbar {
    height: 40px;
    background: #2b3139;
    border-bottom: 1px solid #3c4043;
    display: flex;
    align-items: center;
    padding: 0 12px;
    gap: 16px;
    
    .symbol-info-mini {
      display: flex;
      align-items: center;
      gap: 8px;
      
      .symbol {
        font-size: 14px;
        font-weight: 600;
        color: #eaecef;
      }
      
      .price {
        font-size: 14px;
        font-weight: 600;
        font-family: 'SF Mono', Monaco, monospace;
        
        &.positive { color: #02c076; }
        &.negative { color: #f84960; }
      }
    }
    
    .timeframe-selector {
      display: flex;
      gap: 1px;
      
      .tf-btn {
        padding: 4px 8px;
        background: transparent;
        border: 1px solid #3c4043;
        color: #848e9c;
        font-size: 11px;
        cursor: pointer;
        transition: all 0.2s;
        
        &:first-child { border-radius: 2px 0 0 2px; }
        &:last-child { border-radius: 0 2px 2px 0; }
        
        &.active {
          background: #f0b90b;
          border-color: #f0b90b;
          color: #000;
        }
        
        &:hover:not(.active) {
          background: #3c4043;
          color: #eaecef;
        }
      }
    }
    
    .chart-tools {
      margin-left: auto;
      display: flex;
      align-items: center;
      gap: 12px;
      
      .realtime-indicator {
        display: flex;
        align-items: center;
        gap: 6px;
        padding: 4px 8px;
        background: rgba(43, 49, 57, 0.5);
        border-radius: 12px;
        
        .indicator-dot {
          width: 8px;
          height: 8px;
          border-radius: 50%;
          
          &.connected {
            background: #02c076;
            box-shadow: 0 0 6px rgba(2, 192, 118, 0.6);
            animation: pulse 2s infinite;
          }
          
          &.connecting {
            background: #f0b90b;
            animation: pulse 1s infinite;
          }
          
          &.disconnected {
            background: #848e9c;
          }
        }
        
        .indicator-text {
          font-size: 11px;
          color: #848e9c;
          font-weight: 500;
        }
      }
      
      .tool-btn {
        padding: 6px;
        background: transparent;
        border: 1px solid #3c4043;
        border-radius: 2px;
        color: #848e9c;
        cursor: pointer;
        transition: all 0.2s;
        
        &:hover {
          background: #3c4043;
          color: #eaecef;
        }
      }
    }
  }
  
  .chart-container {
    flex: 1;
    background: #1e2329;
  }
}

@keyframes pulse {
  0%, 100% { opacity: 1; }
  50% { opacity: 0.5; }
}
</style>