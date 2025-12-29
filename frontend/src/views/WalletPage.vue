<template>
  <div class="wallet-page">
    <div class="page-container">
      <div class="page-header">
        <h1>我的钱包</h1>
        <p>管理您的数字资产</p>
      </div>
      
      <div class="wallet-content">
        <div class="balance-overview">
          <div class="balance-card">
            <h2>总资产 (USDT)</h2>
            <div class="balance-amount">{{ formatBalance(totalBalance) }}</div>
            <div class="balance-change positive">+2.45%</div>
          </div>
          
          <div class="balance-breakdown">
            <div class="breakdown-item">
              <span class="label">可用余额</span>
              <span class="value">{{ formatBalance(availableBalance) }}</span>
            </div>
            <div class="breakdown-item">
              <span class="label">冻结资金</span>
              <span class="value">{{ formatBalance(frozenBalance) }}</span>
            </div>
            <div class="breakdown-item">
              <span class="label">未实现盈亏</span>
              <span class="value positive">{{ formatBalance(unrealizedPnL) }}</span>
            </div>
          </div>
        </div>
        
        <div class="assets-list">
          <h3>资产列表</h3>
          <div class="assets-table">
            <div class="table-header">
              <span>币种</span>
              <span>可用</span>
              <span>冻结</span>
              <span>估值(USDT)</span>
            </div>
            <div class="table-body">
              <div v-for="asset in assets" :key="asset.symbol" class="asset-row">
                <span class="symbol">{{ asset.symbol }}</span>
                <span class="available">{{ asset.available }}</span>
                <span class="frozen">{{ asset.frozen }}</span>
                <span class="value">{{ formatBalance(asset.value) }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

// 模拟数据
const totalBalance = ref(12450.67)
const availableBalance = ref(10000.00)
const frozenBalance = ref(2200.00)
const unrealizedPnL = ref(250.67)

const assets = ref([
  { symbol: 'USDT', available: 8500.00, frozen: 1500.00, value: 10000.00 },
  { symbol: 'BTC', available: 0.05, frozen: 0.01, value: 2200.00 },
  { symbol: 'ETH', available: 0.8, frozen: 0.1, value: 250.67 }
])

const formatBalance = (balance: number) => {
  return balance.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  })
}
</script>

<style lang="scss" scoped>
.wallet-page {
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
  
  .balance-overview {
    background: #1e2329;
    border-radius: 8px;
    padding: 24px;
    margin-bottom: 24px;
    
    .balance-card {
      text-align: center;
      margin-bottom: 24px;
      
      h2 {
        font-size: 16px;
        color: #848e9c;
        margin: 0 0 12px 0;
      }
      
      .balance-amount {
        font-size: 36px;
        font-weight: 600;
        color: #eaecef;
        margin-bottom: 8px;
      }
      
      .balance-change {
        font-size: 14px;
        
        &.positive {
          color: #02c076;
        }
        
        &.negative {
          color: #f84960;
        }
      }
    }
    
    .balance-breakdown {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 16px;
      
      .breakdown-item {
        display: flex;
        justify-content: space-between;
        padding: 12px;
        background: #2b3139;
        border-radius: 6px;
        
        .label {
          color: #848e9c;
          font-size: 14px;
        }
        
        .value {
          color: #eaecef;
          font-weight: 600;
          
          &.positive {
            color: #02c076;
          }
        }
      }
    }
  }
  
  .assets-list {
    background: #1e2329;
    border-radius: 8px;
    padding: 24px;
    
    h3 {
      margin: 0 0 16px 0;
      font-size: 18px;
    }
    
    .assets-table {
      .table-header {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr 1fr;
        padding: 12px 0;
        border-bottom: 1px solid #2b3139;
        font-size: 12px;
        color: #848e9c;
        font-weight: 600;
      }
      
      .table-body {
        .asset-row {
          display: grid;
          grid-template-columns: 1fr 1fr 1fr 1fr;
          padding: 12px 0;
          border-bottom: 1px solid #2b3139;
          
          &:last-child {
            border-bottom: none;
          }
          
          .symbol {
            font-weight: 600;
          }
        }
      }
    }
  }
}
</style>