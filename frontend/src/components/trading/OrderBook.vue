<template>
  <div class="orderbook-section">
    <div class="orderbook-header">
      <span>订单簿</span>
      <select class="precision-select">
        <option>0.01</option>
        <option>0.1</option>
        <option>1</option>
      </select>
    </div>
    
    <div class="orderbook-content">
      <div class="orderbook-table">
        <div class="orderbook-header-row">
          <span>价格(USDT)</span>
          <span>数量({{ symbol.replace('USDT', '') }})</span>
          <span>累计</span>
        </div>
        
        <!-- 卖单 -->
        <div class="asks-section">
          <div 
            v-for="(ask, index) in asks"
            :key="'ask-' + index"
            class="orderbook-row ask-row"
            @click="$emit('price-click', ask[0])"
          >
            <span class="price ask-price">{{ formatPrice(ask[0]) }}</span>
            <span class="quantity">{{ formatQuantity(ask[1]) }}</span>
            <span class="total">{{ formatQuantity(ask[2]) }}</span>
          </div>
        </div>
        
        <!-- 当前价格 -->
        <div class="current-price-row">
          <span class="current-price-label" :class="priceChangeClass">
            {{ formatPrice(currentPrice) }}
          </span>
          <span class="price-change-mini" :class="priceChangeClass">
            {{ formatPriceChange(priceChange) }}
          </span>
        </div>
        
        <!-- 买单 -->
        <div class="bids-section">
          <div 
            v-for="(bid, index) in bids"
            :key="'bid-' + index"
            class="orderbook-row bid-row"
            @click="$emit('price-click', bid[0])"
          >
            <span class="price bid-price">{{ formatPrice(bid[0]) }}</span>
            <span class="quantity">{{ formatQuantity(bid[1]) }}</span>
            <span class="total">{{ formatQuantity(bid[2]) }}</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { formatPrice, formatQuantity, formatPriceChange } from '@/utils/formatters'

interface Props {
  symbol: string
  currentPrice: number
  priceChange: number
  bids: number[][]
  asks: number[][]
}

const props = defineProps<Props>()

defineEmits<{
  'price-click': [price: number]
}>()

const priceChangeClass = computed(() => {
  return props.priceChange >= 0 ? 'positive' : 'negative'
})
</script>

<style lang="scss" scoped>
.orderbook-section {
  flex: 1;
  display: flex;
  flex-direction: column;
  
  .orderbook-header {
    padding: 12px 16px;
    background: #2b3139;
    display: flex;
    justify-content: space-between;
    align-items: center;
    font-size: 12px;
    font-weight: 500;
    color: #eaecef;
    
    .precision-select {
      background: #3c4043;
      border: 1px solid #4a5568;
      border-radius: 2px;
      color: #eaecef;
      font-size: 11px;
      padding: 2px 6px;
    }
  }
  
  .orderbook-content {
    flex: 1;
    overflow: hidden;
    
    .orderbook-table {
      height: 100%;
      display: flex;
      flex-direction: column;
      
      .orderbook-header-row {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;
        padding: 8px 16px;
        background: #2b3139;
        font-size: 11px;
        color: #848e9c;
        text-align: right;
        
        span:first-child {
          text-align: left;
        }
      }
      
      .asks-section, .bids-section {
        .orderbook-row {
          display: grid;
          grid-template-columns: 1fr 1fr 1fr;
          padding: 2px 16px;
          font-size: 11px;
          font-family: 'SF Mono', Monaco, monospace;
          cursor: pointer;
          transition: background 0.2s;
          
          &:hover {
            background: #2b3139;
          }
          
          .price {
            text-align: left;
            
            &.ask-price { color: #f84960; }
            &.bid-price { color: #02c076; }
          }
          
          .quantity, .total {
            text-align: right;
            color: #eaecef;
          }
        }
      }
      
      .current-price-row {
        padding: 8px 16px;
        background: #2b3139;
        display: flex;
        justify-content: space-between;
        align-items: center;
        border-top: 1px solid #3c4043;
        border-bottom: 1px solid #3c4043;
        
        .current-price-label {
          font-size: 14px;
          font-weight: 600;
          font-family: 'SF Mono', Monaco, monospace;
          
          &.positive { color: #02c076; }
          &.negative { color: #f84960; }
        }
        
        .price-change-mini {
          font-size: 11px;
          
          &.positive { color: #02c076; }
          &.negative { color: #f84960; }
        }
      }
    }
  }
}
</style>