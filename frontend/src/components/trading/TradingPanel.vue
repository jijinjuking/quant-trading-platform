<template>
  <div class="trading-panel-section">
    <div class="panel-tabs">
      <button class="tab-btn active">现货</button>
      <button class="tab-btn">杠杆</button>
    </div>
    
    <div class="trading-form">
      <div class="order-type-tabs">
        <button 
          :class="['order-type-btn', { active: orderType === 'LIMIT' }]"
          @click="$emit('order-type-change', 'LIMIT')"
        >
          限价
        </button>
        <button 
          :class="['order-type-btn', { active: orderType === 'MARKET' }]"
          @click="$emit('order-type-change', 'MARKET')"
        >
          市价
        </button>
      </div>
      
      <div class="balance-info">
        <div class="balance-row">
          <span>可用</span>
          <span class="balance-amount">{{ formatBalance(balance) }} USDT</span>
        </div>
        <div class="balance-row" v-if="totalPnL !== 0">
          <span>未实现盈亏</span>
          <span class="balance-amount" :class="totalPnL >= 0 ? 'positive' : 'negative'">
            {{ formatBalance(totalPnL) }} USDT
          </span>
        </div>
      </div>
      
      <div class="order-inputs">
        <div class="input-group" v-if="orderType === 'LIMIT'">
          <label>价格</label>
          <input 
            type="text" 
            :value="orderPrice" 
            :placeholder="formatPrice(currentPrice)"
            class="price-input"
            @input="$emit('price-change', ($event.target as HTMLInputElement).value)"
          />
          <span class="input-unit">USDT</span>
        </div>
        
        <div class="input-group">
          <label>数量</label>
          <input 
            type="text" 
            :value="orderQuantity"
            placeholder="0" 
            class="quantity-input"
            @input="$emit('quantity-change', ($event.target as HTMLInputElement).value)"
          />
          <span class="input-unit">{{ symbol.replace('USDT', '') }}</span>
        </div>
        
        <div class="percentage-buttons">
          <button 
            v-for="pct in [25, 50, 75, 100]"
            :key="pct"
            class="pct-btn" 
            @click="$emit('percentage-click', pct)"
          >
            {{ pct }}%
          </button>
        </div>
        
        <div class="input-group">
          <label>金额</label>
          <input 
            type="text" 
            :value="orderAmount"
            placeholder="0" 
            class="amount-input"
            @input="$emit('amount-change', ($event.target as HTMLInputElement).value)"
          />
          <span class="input-unit">USDT</span>
        </div>
      </div>
      
      <div class="order-buttons">
        <button 
          class="buy-btn" 
          @click="$emit('buy-click')"
          :disabled="isSubmittingOrder || !canTrade"
        >
          {{ isSubmittingOrder ? '提交中...' : `买入 ${symbol.replace('USDT', '')}` }}
        </button>
        <button 
          class="sell-btn" 
          @click="$emit('sell-click')"
          :disabled="isSubmittingOrder || !canTrade"
        >
          {{ isSubmittingOrder ? '提交中...' : `卖出 ${symbol.replace('USDT', '')}` }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatPrice, formatBalance } from '@/utils/formatters'

interface Props {
  symbol: string
  currentPrice: number
  orderType: 'LIMIT' | 'MARKET'
  orderPrice: string
  orderQuantity: string
  orderAmount: string
  balance: number
  totalPnL: number
  isSubmittingOrder: boolean
  canTrade: boolean
}

defineProps<Props>()

defineEmits<{
  'order-type-change': [type: 'LIMIT' | 'MARKET']
  'price-change': [price: string]
  'quantity-change': [quantity: string]
  'amount-change': [amount: string]
  'percentage-click': [percentage: number]
  'buy-click': []
  'sell-click': []
}>()
</script>

<style lang="scss" scoped>
.trading-panel-section {
  border-bottom: 1px solid #2b3139;
  
  .panel-tabs {
    display: flex;
    background: #2b3139;
    
    .tab-btn {
      flex: 1;
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
      }
    }
  }
  
  .trading-form {
    padding: 16px;
    
    .order-type-tabs {
      display: flex;
      gap: 1px;
      margin-bottom: 16px;
      
      .order-type-btn {
        flex: 1;
        padding: 6px 12px;
        background: #2b3139;
        border: none;
        color: #848e9c;
        font-size: 12px;
        cursor: pointer;
        
        &:first-child { border-radius: 2px 0 0 2px; }
        &:last-child { border-radius: 0 2px 2px 0; }
        
        &.active {
          background: #f0b90b;
          color: #000;
        }
      }
    }
    
    .balance-info {
      margin-bottom: 16px;
      
      .balance-row {
        display: flex;
        justify-content: space-between;
        font-size: 12px;
        color: #848e9c;
        
        .balance-amount {
          color: #eaecef;
          font-family: 'SF Mono', Monaco, monospace;
          
          &.positive { color: #02c076; }
          &.negative { color: #f84960; }
        }
      }
    }
    
    .order-inputs {
      .input-group {
        margin-bottom: 12px;
        position: relative;
        
        label {
          display: block;
          font-size: 11px;
          color: #848e9c;
          margin-bottom: 4px;
        }
        
        input {
          width: 100%;
          padding: 8px 40px 8px 8px;
          background: #2b3139;
          border: 1px solid #3c4043;
          border-radius: 2px;
          color: #eaecef;
          font-size: 12px;
          font-family: 'SF Mono', Monaco, monospace;
          
          &:focus {
            outline: none;
            border-color: #f0b90b;
          }
        }
        
        .input-unit {
          position: absolute;
          right: 8px;
          top: 50%;
          transform: translateY(-50%);
          font-size: 11px;
          color: #848e9c;
          pointer-events: none;
        }
      }
      
      .percentage-buttons {
        display: flex;
        gap: 4px;
        margin-bottom: 12px;
        
        .pct-btn {
          flex: 1;
          padding: 4px 8px;
          background: #2b3139;
          border: 1px solid #3c4043;
          border-radius: 2px;
          color: #848e9c;
          font-size: 11px;
          cursor: pointer;
          transition: all 0.2s;
          
          &:hover {
            background: #3c4043;
            color: #eaecef;
          }
        }
      }
    }
    
    .order-buttons {
      display: flex;
      gap: 8px;
      
      .buy-btn, .sell-btn {
        flex: 1;
        padding: 10px 16px;
        border: none;
        border-radius: 2px;
        font-size: 12px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
      }
      
      .buy-btn {
        background: #02c076;
        color: #fff;
        
        &:hover:not(:disabled) {
          background: #00a866;
        }
        
        &:disabled {
          background: #4a5568;
          color: #848e9c;
          cursor: not-allowed;
        }
      }
      
      .sell-btn {
        background: #f84960;
        color: #fff;
        
        &:hover:not(:disabled) {
          background: #e63946;
        }
        
        &:disabled {
          background: #4a5568;
          color: #848e9c;
          cursor: not-allowed;
        }
      }
    }
  }
}
</style>