<template>
  <div class="market-sidebar">
    <div class="market-header">
      <div class="market-tabs">
        <button 
          v-for="tab in marketTabs"
          :key="tab.key"
          :class="['market-tab', { active: activeTab === tab.key }]"
          @click="$emit('tab-change', tab.key)"
        >
          {{ tab.label }}
        </button>
      </div>
      <div class="market-search">
        <input 
          type="text" 
          placeholder="搜索"
          :value="searchKeyword"
          @input="$emit('search', ($event.target as HTMLInputElement).value)"
          class="search-input"
        />
      </div>
    </div>
    
    <div class="market-table">
      <div class="table-header">
        <div class="col-symbol">交易对</div>
        <div class="col-price">价格</div>
        <div class="col-change">涨跌幅</div>
      </div>
      <div class="table-body">
        <div 
          v-for="symbol in symbols"
          :key="symbol.symbol"
          :class="['symbol-row', { active: selectedSymbol === symbol.symbol }]"
          @click="$emit('symbol-select', symbol.symbol)"
        >
          <div class="col-symbol">
            <span class="symbol-name">{{ symbol.symbol }}</span>
          </div>
          <div class="col-price">{{ formatPrice(symbol.price) }}</div>
          <div class="col-change" :class="symbol.change >= 0 ? 'positive' : 'negative'">
            {{ formatPriceChange(symbol.change) }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { formatPrice, formatPriceChange } from '@/utils/formatters'

interface Symbol {
  symbol: string
  price: number
  change: number
}

interface Props {
  symbols: Symbol[]
  selectedSymbol: string
  activeTab: string
  searchKeyword: string
}

defineProps<Props>()

defineEmits<{
  'tab-change': [tab: string]
  'search': [keyword: string]
  'symbol-select': [symbol: string]
}>()

const marketTabs = [
  { key: 'usdt', label: 'USDT' },
  { key: 'btc', label: 'BTC' },
  { key: 'eth', label: 'ETH' },
  { key: 'hot', label: '热门' }
]
</script>

<style lang="scss" scoped>
.market-sidebar {
  width: 280px;
  background: #1e2329;
  border-right: 1px solid #2b3139;
  display: flex;
  flex-direction: column;
  
  .market-header {
    padding: 12px;
    border-bottom: 1px solid #2b3139;
    
    .market-tabs {
      display: flex;
      gap: 1px;
      margin-bottom: 12px;
      
      .market-tab {
        flex: 1;
        padding: 6px 12px;
        background: #2b3139;
        border: none;
        color: #848e9c;
        font-size: 12px;
        cursor: pointer;
        transition: all 0.2s;
        
        &:first-child { border-radius: 2px 0 0 2px; }
        &:last-child { border-radius: 0 2px 2px 0; }
        
        &.active {
          background: #f0b90b;
          color: #000;
        }
        
        &:hover:not(.active) {
          background: #3c4043;
          color: #eaecef;
        }
      }
    }
    
    .market-search {
      .search-input {
        width: 100%;
        padding: 6px 8px;
        background: #2b3139;
        border: 1px solid #3c4043;
        border-radius: 2px;
        color: #eaecef;
        font-size: 12px;
        
        &::placeholder {
          color: #848e9c;
        }
        
        &:focus {
          outline: none;
          border-color: #f0b90b;
        }
      }
    }
  }
  
  .market-table {
    flex: 1;
    overflow: hidden;
    
    .table-header {
      display: grid;
      grid-template-columns: 1fr 80px 60px;
      padding: 8px 12px;
      background: #2b3139;
      font-size: 11px;
      color: #848e9c;
      font-weight: 500;
      
      .col-symbol { text-align: left; }
      .col-price { text-align: right; }
      .col-change { text-align: right; }
    }
    
    .table-body {
      height: calc(100% - 32px);
      overflow-y: auto;
      
      .symbol-row {
        display: grid;
        grid-template-columns: 1fr 80px 60px;
        padding: 6px 12px;
        cursor: pointer;
        transition: background 0.2s;
        
        &:hover {
          background: #2b3139;
        }
        
        &.active {
          background: rgba(240, 185, 11, 0.1);
        }
        
        .col-symbol {
          .symbol-name {
            font-size: 12px;
            font-weight: 500;
            color: #eaecef;
          }
        }
        
        .col-price {
          text-align: right;
          font-family: 'SF Mono', Monaco, monospace;
          font-size: 12px;
          color: #eaecef;
        }
        
        .col-change {
          text-align: right;
          font-family: 'SF Mono', Monaco, monospace;
          font-size: 12px;
          
          &.positive { color: #02c076; }
          &.negative { color: #f84960; }
        }
      }
    }
  }
}

@media (max-width: 1200px) {
  .market-sidebar {
    width: 240px;
  }
}
</style>