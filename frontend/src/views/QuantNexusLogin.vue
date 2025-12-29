<template>
  <div class="min-h-screen w-full flex bg-quant-900 overflow-hidden font-sans selection:bg-quant-accent/30 selection:text-white">
    
    <!-- Left Side - Login/Register Form -->
    <div class="w-full lg:w-[480px] flex flex-col justify-between p-8 lg:p-12 relative z-20 bg-quant-900 border-r border-quant-800 shadow-2xl">
      <!-- Logo Header -->
      <div class="flex items-center space-x-2 animate-slideInLeft">
        <div class="w-8 h-8 bg-gradient-to-tr from-quant-accent to-blue-400 rounded-lg flex items-center justify-center shadow-lg shadow-blue-900/20">
          <Terminal class="w-5 h-5 text-white" />
        </div>
        <span class="text-xl font-bold tracking-tight text-white">QuantNexus</span>
      </div>

      <!-- Form Container -->
      <div class="my-auto">
        <Transition name="slide-fade" mode="out-in">
          <RegisterForm 
            v-if="isRegistering" 
            @login-click="isRegistering = false"
            @register-success="handleRegisterSuccess"
          />
          <LoginForm 
            v-else 
            @register-click="isRegistering = true"
            @login-success="handleLoginSuccess"
          />
        </Transition>
      </div>

      <!-- Footer -->
      <div class="flex items-center justify-between text-xs text-gray-600 font-mono animate-slideInLeft" style="animation-delay: 0.3s">
        <span>v2.4.0-stable</span>
        <div class="flex items-center space-x-4">
          <span class="flex items-center hover:text-gray-400 cursor-pointer transition-colors">
            <Globe class="w-3 h-3 mr-1" /> CN
          </span>
          <span class="flex items-center hover:text-gray-400 cursor-pointer transition-colors">
            <Zap class="w-3 h-3 mr-1" /> 
            <span class="relative">
              系统状态: 正常
              <span class="absolute -top-1 -right-1 w-2 h-2 bg-green-500 rounded-full animate-pulse"></span>
            </span>
          </span>
        </div>
      </div>
    </div>

    <!-- Right Side - Visuals -->
    <div class="hidden lg:flex flex-1 relative bg-quant-900 flex-col items-center justify-center p-12 overflow-hidden">
      
      <!-- Abstract Background Effects -->
      <div class="absolute inset-0 bg-[linear-gradient(rgba(11,14,20,0)_0%,rgba(11,14,20,0.8)_100%),radial-gradient(circle_at_50%_0%,rgba(59,130,246,0.15)_0%,rgba(11,14,20,0)_50%)]" />
      
      <!-- Animated Background Particles -->
      <div class="absolute inset-0">
        <div 
          v-for="(particle, index) in backgroundParticles" 
          :key="index"
          class="absolute w-1 h-1 bg-quant-accent/30 rounded-full animate-float-bg"
          :style="{ 
            left: particle.x + '%', 
            top: particle.y + '%',
            animationDelay: particle.delay + 's',
            animationDuration: particle.duration + 's'
          }"
        />
      </div>
      
      <!-- Globe Visual -->
      <GlobeVisual />
      
      <!-- Floating Widgets Container -->
      <div class="relative z-10 w-full max-w-4xl h-[600px] grid grid-cols-2 grid-rows-2 gap-6 p-6">
          
          <!-- Main Chart Widget -->
          <div class="col-span-2 row-span-1 animate-float animate-slideInRight" style="animation-delay: 0.2s">
            <MarketChart />
          </div>

          <!-- Stats Widget 1 - System Latency -->
          <div class="col-span-1 row-span-1 bg-quant-800/40 backdrop-blur-md rounded-xl p-6 border border-quant-700 hover:border-quant-accent/30 transition-all duration-300 group animate-float animate-slideInRight" style="animation-delay: 0.4s">
            <div class="flex items-start justify-between mb-4">
              <div class="p-2 bg-purple-500/10 rounded-lg group-hover:bg-purple-500/20 transition-colors">
                <Cpu class="w-6 h-6 text-purple-400" />
              </div>
              <span class="px-2 py-1 rounded-full bg-green-500/10 text-green-400 text-xs font-mono animate-pulse">
                +12.5%
              </span>
            </div>
            <h4 class="text-gray-400 text-sm font-medium">系统延迟 (Latency)</h4>
            <p class="text-2xl font-mono text-white mt-1">{{ systemLatency }}<span class="text-sm text-gray-500 ml-1">ms</span></p>
            <div class="mt-4 h-1.5 w-full bg-quant-700 rounded-full overflow-hidden">
              <div 
                class="h-full bg-gradient-to-r from-purple-500 to-indigo-500 transition-all duration-1000 ease-out" 
                :style="{ width: latencyPercentage + '%' }" 
              />
            </div>
            <div class="mt-2 flex justify-between text-xs text-gray-500">
              <span>优秀</span>
              <span>{{ latencyPercentage }}%</span>
            </div>
          </div>

          <!-- Stats Widget 2 - Trading Volume -->
          <div class="col-span-1 row-span-1 bg-quant-800/40 backdrop-blur-md rounded-xl p-6 border border-quant-700 hover:border-quant-accent/30 transition-all duration-300 group animate-float animate-slideInRight" style="animation-delay: 0.6s">
             <div class="flex items-start justify-between mb-4">
              <div class="p-2 bg-emerald-500/10 rounded-lg group-hover:bg-emerald-500/20 transition-colors">
                <BarChart3 class="w-6 h-6 text-emerald-400" />
              </div>
               <span class="px-2 py-1 rounded-full bg-blue-500/10 text-blue-400 text-xs font-mono">
                活跃中
              </span>
            </div>
            <h4 class="text-gray-400 text-sm font-medium">24H 交易量</h4>
            <p class="text-2xl font-mono text-white mt-1">${{ tradingVolume }}<span class="text-sm text-gray-500 ml-1">M</span></p>
             <div class="mt-4 flex space-x-1">
               <div 
                 v-for="(bar, i) in volumeBars" 
                 :key="i" 
                 :class="`h-1.5 flex-1 rounded-full transition-all duration-300 ${bar ? 'bg-emerald-500' : 'bg-quant-700'}`"
                 :style="{ transitionDelay: i * 50 + 'ms' }"
               />
             </div>
             <div class="mt-2 flex justify-between text-xs text-gray-500">
               <span>实时</span>
               <span>{{ Math.round(volumeBars.filter(Boolean).length / volumeBars.length * 100) }}% 活跃</span>
             </div>
          </div>

      </div>

      <!-- Bottom Text -->
      <div class="absolute bottom-12 text-center z-10 animate-slideInUp" style="animation-delay: 0.8s">
        <h3 class="text-white font-semibold text-lg mb-2">机构级交易基础设施</h3>
        <p class="text-gray-400 text-sm max-w-md mx-auto leading-relaxed">
          体验专为高频交易打造的超低延迟执行引擎与高级分析工具。
        </p>
        <div class="flex items-center justify-center mt-4 space-x-6 text-xs text-gray-500">
          <div class="flex items-center">
            <div class="w-2 h-2 bg-green-500 rounded-full mr-2 animate-pulse"></div>
            <span>实时数据</span>
          </div>
          <div class="flex items-center">
            <div class="w-2 h-2 bg-blue-500 rounded-full mr-2 animate-pulse" style="animation-delay: 0.5s"></div>
            <span>安全连接</span>
          </div>
          <div class="flex items-center">
            <div class="w-2 h-2 bg-purple-500 rounded-full mr-2 animate-pulse" style="animation-delay: 1s"></div>
            <span>低延迟</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElNotification } from 'element-plus'
import { Terminal, Globe, Zap, Cpu, BarChart3 } from 'lucide-vue-next'
import LoginForm from '@/components/auth/LoginForm.vue'
import RegisterForm from '@/components/auth/RegisterForm.vue'
import MarketChart from '@/components/auth/MarketChart.vue'
import GlobeVisual from '@/components/auth/GlobeVisual.vue'

const router = useRouter()

// 状态管理
const isRegistering = ref(false)

// 实时数据模拟
const systemLatency = ref(14.2)
const tradingVolume = ref(42.8)
const latencyPercentage = ref(70)
const volumeBars = ref(Array.from({ length: 12 }, (_, i) => i < 8))

// 背景粒子
const backgroundParticles = ref<Array<{ x: number, y: number, delay: number, duration: number }>>([])

// 定时器
let statsTimer: NodeJS.Timeout | null = null

// 生成背景粒子
const generateBackgroundParticles = () => {
  const particles = []
  for (let i = 0; i < 30; i++) {
    particles.push({
      x: Math.random() * 100,
      y: Math.random() * 100,
      delay: Math.random() * 10,
      duration: 8 + Math.random() * 12
    })
  }
  backgroundParticles.value = particles
}

// 模拟实时数据更新
const updateStats = () => {
  // 系统延迟在 10-20ms 之间波动
  systemLatency.value = Number((10 + Math.random() * 10).toFixed(1))
  latencyPercentage.value = Math.max(50, Math.min(95, 100 - systemLatency.value * 4))
  
  // 交易量在 40-50M 之间波动
  tradingVolume.value = Number((40 + Math.random() * 10).toFixed(1))
  
  // 随机更新音量条
  volumeBars.value = volumeBars.value.map(() => Math.random() > 0.3)
}

// 处理登录成功
const handleLoginSuccess = (userData: any) => {
  ElNotification({
    title: '登录成功',
    message: `欢迎回来，${userData.username}！`,
    type: 'success',
    duration: 3000,
    position: 'top-right'
  })
  
  // 跳转到交易页面
  router.push('/trading')
}

// 处理注册成功
const handleRegisterSuccess = (userData: any) => {
  ElNotification({
    title: '注册成功',
    message: `欢迎加入，${userData.username}！`,
    type: 'success',
    duration: 3000,
    position: 'top-right'
  })
  
  // 跳转到交易页面
  router.push('/trading')
}

onMounted(() => {
  // 生成背景粒子
  generateBackgroundParticles()
  
  // 启动实时数据更新
  statsTimer = setInterval(updateStats, 2000)
})

onUnmounted(() => {
  if (statsTimer) {
    clearInterval(statsTimer)
  }
})
</script>

<style scoped>
/* 页面切换动画 */
.slide-fade-enter-active,
.slide-fade-leave-active {
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.slide-fade-enter-from {
  opacity: 0;
  transform: translateX(30px);
}

.slide-fade-leave-to {
  opacity: 0;
  transform: translateX(-30px);
}

/* 自定义滚动条 */
:deep(::-webkit-scrollbar) {
  width: 8px;
}

:deep(::-webkit-scrollbar-track) {
  background: #0B0E14;
}

:deep(::-webkit-scrollbar-thumb) {
  background: #1E2433;
  border-radius: 4px;
}

:deep(::-webkit-scrollbar-thumb:hover) {
  background: #3B82F6;
}

/* 浮动动画 */
@keyframes float {
  0%, 100% { 
    transform: translateY(0); 
  }
  50% { 
    transform: translateY(-20px); 
  }
}

.animate-float {
  animation: float 6s ease-in-out infinite;
}

/* 背景粒子浮动 */
@keyframes float-bg {
  0%, 100% { 
    transform: translateY(0px) translateX(0px);
    opacity: 0.3;
  }
  33% { 
    transform: translateY(-30px) translateX(10px);
    opacity: 0.7;
  }
  66% { 
    transform: translateY(-10px) translateX(-15px);
    opacity: 0.5;
  }
}

.animate-float-bg {
  animation: float-bg 15s ease-in-out infinite;
}

/* 入场动画 */
@keyframes slideInLeft {
  0% { 
    opacity: 0; 
    transform: translateX(-50px); 
  }
  100% { 
    opacity: 1; 
    transform: translateX(0); 
  }
}

@keyframes slideInRight {
  0% { 
    opacity: 0; 
    transform: translateX(50px); 
  }
  100% { 
    opacity: 1; 
    transform: translateX(0); 
  }
}

@keyframes slideInUp {
  0% { 
    opacity: 0; 
    transform: translateY(30px); 
  }
  100% { 
    opacity: 1; 
    transform: translateY(0); 
  }
}

.animate-slideInLeft {
  animation: slideInLeft 0.8s ease-out;
}

.animate-slideInRight {
  animation: slideInRight 0.8s ease-out;
}

.animate-slideInUp {
  animation: slideInUp 0.8s ease-out;
}

/* 渐入动画 */
@keyframes fadeIn {
  0% { 
    opacity: 0; 
    transform: translateY(20px); 
  }
  100% { 
    opacity: 1; 
    transform: translateY(0); 
  }
}

.animate-fadeIn {
  animation: fadeIn 0.5s ease-out;
}
</style>