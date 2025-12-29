<template>
  <div class="strategies-page">
    <div class="page-container">
      <div class="page-header">
        <h1>策略管理</h1>
        <p>创建和管理您的交易策略</p>
        <button class="create-btn">创建策略</button>
      </div>
      
      <div class="strategies-content">
        <div class="strategies-grid">
          <div 
            v-for="strategy in strategies"
            :key="strategy.id"
            class="strategy-card"
          >
            <div class="card-header">
              <div class="strategy-info">
                <h3>{{ strategy.name }}</h3>
                <span class="strategy-type" :class="getTypeClass(strategy.type)">
                  {{ getTypeText(strategy.type) }}
                </span>
              </div>
              <div class="strategy-status" :class="getStatusClass(strategy.status)">
                {{ getStatusText(strategy.status) }}
              </div>
            </div>
            
            <div class="card-content">
              <p class="description">{{ strategy.description }}</p>
              
              <div class="strategy-symbols">
                <span class="symbols-label">交易对:</span>
                <div class="symbols-list">
                  <span 
                    v-for="symbol in strategy.symbols"
                    :key="symbol"
                    class="symbol-tag"
                  >
                    {{ symbol }}
                  </span>
                </div>
              </div>
              
              <div class="performance-metrics">
                <div class="metric">
                  <span class="metric-label">总收益</span>
                  <span class="metric-value positive">{{ strategy.performance.totalReturn }}%</span>
                </div>
                <div class="metric">
                  <span class="metric-label">胜率</span>
                  <span class="metric-value">{{ strategy.performance.winRate }}%</span>
                </div>
                <div class="metric">
                  <span class="metric-label">最大回撤</span>
                  <span class="metric-value negative">{{ strategy.performance.maxDrawdown }}%</span>
                </div>
                <div class="metric">
                  <span class="metric-label">夏普比率</span>
                  <span class="metric-value">{{ strategy.performance.sharpeRatio }}</span>
                </div>
              </div>
            </div>
            
            <div class="card-actions">
              <button class="action-btn edit-btn">编辑</button>
              <button 
                class="action-btn"
                :class="strategy.status === 'ACTIVE' ? 'pause-btn' : 'start-btn'"
                @click="toggleStrategy(strategy)"
              >
                {{ strategy.status === 'ACTIVE' ? '暂停' : '启动' }}
              </button>
              <button class="action-btn delete-btn">删除</button>
            </div>
          </div>
          
          <!-- 创建新策略卡片 -->
          <div class="strategy-card create-card">
            <div class="create-content">
              <div class="create-icon">+</div>
              <h3>创建新策略</h3>
              <p>使用AI助手或手动创建交易策略</p>
              <button class="create-strategy-btn">开始创建</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

// 模拟策略数据
const strategies = ref([
  {
    id: 'strategy_1',
    name: 'BTC网格策略',
    description: '基于BTC价格波动的网格交易策略，适合震荡行情',
    type: 'AI',
    status: 'ACTIVE',
    symbols: ['BTCUSDT'],
    performance: {
      totalReturn: 15.6,
      winRate: 68.5,
      maxDrawdown: -8.2,
      sharpeRatio: 1.45
    }
  },
  {
    id: 'strategy_2',
    name: 'ETH趋势跟踪',
    description: '基于技术指标的ETH趋势跟踪策略，捕捉中长期趋势',
    type: 'MANUAL',
    status: 'ACTIVE',
    symbols: ['ETHUSDT'],
    performance: {
      totalReturn: 23.4,
      winRate: 72.1,
      maxDrawdown: -12.5,
      sharpeRatio: 1.78
    }
  },
  {
    id: 'strategy_3',
    name: '多币种套利',
    description: '跨交易对套利策略，利用价差获取稳定收益',
    type: 'COPY',
    status: 'PAUSED',
    symbols: ['BTCUSDT', 'ETHUSDT', 'BNBUSDT'],
    performance: {
      totalReturn: 8.9,
      winRate: 85.2,
      maxDrawdown: -3.1,
      sharpeRatio: 2.15
    }
  }
])

const getTypeText = (type: string) => {
  const typeMap = {
    'AI': 'AI策略',
    'MANUAL': '手动策略',
    'COPY': '跟单策略'
  }
  return typeMap[type as keyof typeof typeMap] || type
}

const getTypeClass = (type: string) => {
  return {
    'type-ai': type === 'AI',
    'type-manual': type === 'MANUAL',
    'type-copy': type === 'COPY'
  }
}

const getStatusText = (status: string) => {
  const statusMap = {
    'ACTIVE': '运行中',
    'PAUSED': '已暂停',
    'INACTIVE': '未激活'
  }
  return statusMap[status as keyof typeof statusMap] || status
}

const getStatusClass = (status: string) => {
  return {
    'status-active': status === 'ACTIVE',
    'status-paused': status === 'PAUSED',
    'status-inactive': status === 'INACTIVE'
  }
}

const toggleStrategy = (strategy: any) => {
  if (strategy.status === 'ACTIVE') {
    strategy.status = 'PAUSED'
  } else {
    strategy.status = 'ACTIVE'
  }
}
</script>

<style lang="scss" scoped>
.strategies-page {
  min-height: 100vh;
  background: #0b0e11;
  color: #eaecef;
  padding: 20px;
  
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .page-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    margin-bottom: 32px;
    
    div {
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
    
    .create-btn {
      padding: 10px 20px;
      background: #f0b90b;
      border: none;
      border-radius: 6px;
      color: #000;
      font-size: 14px;
      font-weight: 600;
      cursor: pointer;
      
      &:hover {
        background: #e6a809;
      }
    }
  }
  
  .strategies-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
    gap: 24px;
    
    .strategy-card {
      background: #1e2329;
      border-radius: 8px;
      padding: 20px;
      border: 1px solid #2b3139;
      transition: all 0.2s;
      
      &:hover {
        border-color: #f0b90b;
      }
      
      .card-header {
        display: flex;
        justify-content: space-between;
        align-items: flex-start;
        margin-bottom: 16px;
        
        .strategy-info {
          h3 {
            font-size: 18px;
            font-weight: 600;
            margin: 0 0 8px 0;
          }
          
          .strategy-type {
            padding: 2px 8px;
            border-radius: 12px;
            font-size: 11px;
            font-weight: 500;
            
            &.type-ai {
              background: rgba(240, 185, 11, 0.2);
              color: #f0b90b;
            }
            
            &.type-manual {
              background: rgba(2, 192, 118, 0.2);
              color: #02c076;
            }
            
            &.type-copy {
              background: rgba(132, 142, 156, 0.2);
              color: #848e9c;
            }
          }
        }
        
        .strategy-status {
          padding: 4px 8px;
          border-radius: 4px;
          font-size: 11px;
          font-weight: 500;
          
          &.status-active {
            background: rgba(2, 192, 118, 0.2);
            color: #02c076;
          }
          
          &.status-paused {
            background: rgba(248, 73, 96, 0.2);
            color: #f84960;
          }
          
          &.status-inactive {
            background: rgba(132, 142, 156, 0.2);
            color: #848e9c;
          }
        }
      }
      
      .card-content {
        .description {
          font-size: 14px;
          color: #848e9c;
          line-height: 1.5;
          margin: 0 0 16px 0;
        }
        
        .strategy-symbols {
          display: flex;
          align-items: center;
          gap: 8px;
          margin-bottom: 16px;
          
          .symbols-label {
            font-size: 12px;
            color: #848e9c;
          }
          
          .symbols-list {
            display: flex;
            gap: 4px;
            
            .symbol-tag {
              padding: 2px 6px;
              background: #2b3139;
              border-radius: 4px;
              font-size: 11px;
              color: #eaecef;
            }
          }
        }
        
        .performance-metrics {
          display: grid;
          grid-template-columns: 1fr 1fr;
          gap: 12px;
          
          .metric {
            display: flex;
            justify-content: space-between;
            
            .metric-label {
              font-size: 12px;
              color: #848e9c;
            }
            
            .metric-value {
              font-size: 12px;
              font-weight: 600;
              
              &.positive {
                color: #02c076;
              }
              
              &.negative {
                color: #f84960;
              }
            }
          }
        }
      }
      
      .card-actions {
        display: flex;
        gap: 8px;
        margin-top: 16px;
        
        .action-btn {
          flex: 1;
          padding: 6px 12px;
          border: none;
          border-radius: 4px;
          font-size: 12px;
          font-weight: 500;
          cursor: pointer;
          transition: all 0.2s;
          
          &.edit-btn {
            background: transparent;
            border: 1px solid #3c4043;
            color: #eaecef;
            
            &:hover {
              background: #3c4043;
            }
          }
          
          &.start-btn {
            background: #02c076;
            color: #fff;
            
            &:hover {
              background: #00a866;
            }
          }
          
          &.pause-btn {
            background: #f0b90b;
            color: #000;
            
            &:hover {
              background: #e6a809;
            }
          }
          
          &.delete-btn {
            background: transparent;
            border: 1px solid #f84960;
            color: #f84960;
            
            &:hover {
              background: rgba(248, 73, 96, 0.1);
            }
          }
        }
      }
      
      &.create-card {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 300px;
        border: 2px dashed #3c4043;
        background: transparent;
        
        &:hover {
          border-color: #f0b90b;
          background: rgba(240, 185, 11, 0.05);
        }
        
        .create-content {
          text-align: center;
          
          .create-icon {
            font-size: 48px;
            color: #848e9c;
            margin-bottom: 16px;
          }
          
          h3 {
            font-size: 18px;
            margin: 0 0 8px 0;
          }
          
          p {
            font-size: 14px;
            color: #848e9c;
            margin: 0 0 16px 0;
          }
          
          .create-strategy-btn {
            padding: 8px 16px;
            background: #f0b90b;
            border: none;
            border-radius: 4px;
            color: #000;
            font-size: 14px;
            font-weight: 500;
            cursor: pointer;
            
            &:hover {
              background: #e6a809;
            }
          }
        }
      }
    }
  }
}
</style>