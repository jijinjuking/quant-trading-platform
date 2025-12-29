<template>
  <div class="w-full h-full flex flex-col p-6 bg-quant-800/50 backdrop-blur-sm rounded-xl border border-quant-700 shadow-2xl overflow-hidden relative group hover:border-quant-accent/50 transition-colors duration-500">
    <div class="flex items-center justify-between mb-4 z-10">
      <div class="flex items-center space-x-2">
        <div class="p-2 bg-quant-accent/10 rounded-lg">
          <Activity class="w-5 h-5 text-quant-accent" />
        </div>
        <div>
          <h3 class="text-sm font-semibold text-gray-200">BTC/USD 永续合约</h3>
          <p class="text-xs text-gray-500">实时指数</p>
        </div>
      </div>
      <div class="text-right">
        <p class="text-lg font-mono font-bold text-quant-success">
          ${{ currentPrice.toFixed(2) }}
        </p>
        <p class="text-xs font-medium text-quant-success flex items-center justify-end">
          +2.45%
        </p>
      </div>
    </div>

    <div class="flex-1 min-h-[200px] w-full relative z-10">
      <div class="w-full h-full flex items-center justify-center">
        <svg class="w-full h-full" viewBox="0 0 400 200">
          <!-- 简化的SVG图表 -->
          <defs>
            <linearGradient id="chartGradient" x1="0%" y1="0%" x2="0%" y2="100%">
              <stop offset="0%" style="stop-color:#3B82F6;stop-opacity:0.3" />
              <stop offset="100%" style="stop-color:#3B82F6;stop-opacity:0" />
            </linearGradient>
          </defs>
          
          <!-- 网格线 -->
          <g stroke="#1E2433" stroke-width="1" opacity="0.5">
            <line v-for="i in 5" :key="'h' + i" :x1="0" :y1="i * 40" :x2="400" :y2="i * 40" />
            <line v-for="i in 8" :key="'v' + i" :x1="i * 50" :y1="0" :x2="i * 50" :y2="200" />
          </g>
          
          <!-- 价格曲线 -->
          <path 
            :d="chartPath" 
            fill="url(#chartGradient)" 
            stroke="#3B82F6" 
            stroke-width="2"
            class="animate-pulse"
          />
          
          <!-- 当前价格点 -->
          <circle 
            :cx="380" 
            :cy="currentPriceY" 
            r="4" 
            fill="#3B82F6"
            class="animate-pulse"
          >
            <animate attributeName="r" values="4;6;4" dur="2s" repeatCount="indefinite" />
          </circle>
        </svg>
      </div>
    </div>
    
    <!-- Decorative Grid Background -->
    <div class="absolute inset-0 z-0 opacity-10" 
         style="background-image: radial-gradient(#3B82F6 1px, transparent 1px); background-size: 20px 20px;">
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { Activity } from 'lucide-vue-next'

const currentPrice = ref(42850.45)
const priceData = ref<number[]>([])

let dataTimer: NodeJS.Timeout | null = null

// 生成初始价格数据
const generateInitialData = () => {
  const data = []
  let val = 42000
  for (let i = 0; i < 20; i++) {
    const change = Math.random() * 200 - 100
    val += change
    data.push(Math.abs(val))
  }
  return data
}

// 计算SVG路径
const chartPath = computed(() => {
  if (priceData.value.length < 2) return ''
  
  const width = 400
  const height = 200
  const minVal = Math.min(...priceData.value)
  const maxVal = Math.max(...priceData.value)
  const range = maxVal - minVal || 1
  
  let path = ''
  
  priceData.value.forEach((price, index) => {
    const x = (width / (priceData.value.length - 1)) * index
    const y = height - ((price - minVal) / range) * height
    
    if (index === 0) {
      path += `M ${x} ${y}`
    } else {
      path += ` L ${x} ${y}`
    }
  })
  
  // 添加面积填充路径
  const lastX = (width / (priceData.value.length - 1)) * (priceData.value.length - 1)
  path += ` L ${lastX} ${height} L 0 ${height} Z`
  
  return path
})

// 计算当前价格点的Y坐标
const currentPriceY = computed(() => {
  if (priceData.value.length === 0) return 100
  
  const minVal = Math.min(...priceData.value)
  const maxVal = Math.max(...priceData.value)
  const range = maxVal - minVal || 1
  const currentVal = priceData.value[priceData.value.length - 1]
  
  return 200 - ((currentVal - minVal) / range) * 200
})

// 更新数据
const updateData = () => {
  if (priceData.value.length === 0) return
  
  const lastValue = priceData.value[priceData.value.length - 1]
  const change = (Math.random() - 0.48) * 100 // 轻微上涨趋势
  const newValue = lastValue + change
  
  priceData.value.push(newValue)
  
  // 保持数据长度
  if (priceData.value.length > 20) {
    priceData.value.shift()
  }
  
  currentPrice.value = newValue
}

onMounted(() => {
  priceData.value = generateInitialData()
  currentPrice.value = priceData.value[priceData.value.length - 1]
  
  // 定期更新数据
  dataTimer = setInterval(updateData, 2000)
})

onUnmounted(() => {
  if (dataTimer) {
    clearInterval(dataTimer)
  }
})
</script>