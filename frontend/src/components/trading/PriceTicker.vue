<template>
  <div class="price-ticker-bar">
    <div class="main-symbol-info">
      <span class="symbol-name">{{ symbol }}</span>
      <span class="current-price" :class="priceChangeClass">{{ formatPrice(price) }}</span>
      <span class="price-change-percent" :class="priceChangeClass">{{ formatPriceChange(priceChange) }}</span>
    </div>
    
    <div class="market-stats-row">
      <div class="stat-item">
        <span class="stat-label">24h变化</span>
        <span class="stat-value" :class="priceChangeClass">{{ formatPriceChange(priceChange) }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">24h高</span>
        <span class="stat-value">{{ formatPrice(high24h) }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">24h低</span>
        <span class="stat-value">{{ formatPrice(low24h) }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">24h量({{ symbol.replace('USDT', '') }})</span>
        <span class="stat-value">{{ formatVolume(volume24h) }}</span>
      </div>
      <div class="stat-item">
        <span class="stat-label">24h量(USDT)</span>
        <span class="stat-value">{{ formatVolume(quoteVolume24h) }}</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatPrice, formatPriceChange, formatVolume } from '@/utils/formatters'

interface Props {
  symbol: string
  price: number
  priceChange: number
  high24h: number
  low24h: number
  volume24h: number
  quoteVolume24h: number
}

const props = defineProps<Props>()

const priceChangeClass = computed(() => {
  return props.priceChange >= 0 ? 'positive' : 'negative'
})
</script>

<style lang="scss" scoped>
.price-ticker-bar {
  display: flex;
  align-items: center;
  gap: 32px;
  height: 100%;
  
  .main-symbol-info {
    display: flex;
    align-items: center;
    gap: 12px;
    
    .symbol-name {
      font-size: 16px;
      font-weight: 600;
      color: #eaecef;
    }
    
    .current-price {
      font-size: 20px;
      font-weight: 600;
      font-family: 'SF Mono', Monaco, monospace;
      
      &.positive { color: #02c076; }
      &.negative { color: #f84960; }
    }
    
    .price-change-percent {
      font-size: 14px;
      font-weight: 500;
      padding: 2px 6px;
      border-radius: 2px;
      
      &.positive { 
        color: #02c076; 
        background: rgba(2, 192, 118, 0.1);
      }
      &.negative { 
        color: #f84960; 
        background: rgba(248, 73, 96, 0.1);
      }
    }
  }
  
  .market-stats-row {
    display: flex;
    gap: 24px;
    
    .stat-item {
      display: flex;
      flex-direction: column;
      gap: 2px;
      
      .stat-label {
        font-size: 11px;
        color: #848e9c;
      }
      
      .stat-value {
        font-size: 12px;
        font-weight: 500;
        color: #eaecef;
        font-family: 'SF Mono', Monaco, monospace;
        
        &.positive { color: #02c076; }
        &.negative { color: #f84960; }
      }
    }
  }
}
</style>