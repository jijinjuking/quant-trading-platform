<template>
  <div class="min-h-screen w-full relative overflow-hidden bg-gradient-to-br from-slate-900 via-blue-900 to-slate-900">
    
    <!-- 动态背景 -->
    <div class="absolute inset-0">
      <!-- 网格背景 -->
      <div class="absolute inset-0 bg-[linear-gradient(rgba(59,130,246,0.03)_1px,transparent_1px),linear-gradient(90deg,rgba(59,130,246,0.03)_1px,transparent_1px)] bg-[size:50px_50px]"></div>
      
      <!-- 动态光效 -->
      <div class="absolute top-0 left-1/4 w-96 h-96 bg-blue-500/10 rounded-full blur-3xl animate-pulse"></div>
      <div class="absolute bottom-0 right-1/4 w-96 h-96 bg-purple-500/10 rounded-full blur-3xl animate-pulse" style="animation-delay: 2s"></div>
      <div class="absolute top-1/2 left-1/2 w-64 h-64 bg-cyan-500/5 rounded-full blur-2xl animate-pulse" style="animation-delay: 4s"></div>
      
      <!-- 浮动粒子 -->
      <div class="absolute inset-0">
        <div 
          v-for="(particle, index) in particles" 
          :key="index"
          class="absolute w-1 h-1 bg-blue-400/30 rounded-full animate-float-particle"
          :style="{ 
            left: particle.x + '%', 
            top: particle.y + '%',
            animationDelay: particle.delay + 's',
            animationDuration: particle.duration + 's'
          }"
        />
      </div>
    </div>

    <!-- 主要内容 -->
    <div class="relative z-10 min-h-screen flex">
      
      <!-- 左侧 - 品牌展示区 -->
      <div class="hidden lg:flex lg:w-1/2 xl:w-3/5 flex-col justify-between p-12 relative">
        
        <!-- 顶部Logo -->
        <div class="flex items-center space-x-3">
          <div class="relative">
            <div class="w-12 h-12 bg-gradient-to-tr from-blue-400 via-blue-500 to-purple-600 rounded-xl flex items-center justify-center shadow-2xl shadow-blue-500/25">
              <ChartIcon class="w-7 h-7 text-white" />
            </div>
            <div class="absolute -top-1 -right-1 w-4 h-4 bg-green-400 rounded-full animate-pulse"></div>
          </div>
          <div>
            <h1 class="text-2xl font-bold text-white tracking-tight">QuantNexus</h1>
            <p class="text-sm text-blue-300/80 font-medium">Professional Trading Platform</p>
          </div>
        </div>

        <!-- 中央展示区 -->
        <div class="flex-1 flex flex-col justify-center space-y-12">
          
          <!-- 主标题 -->
          <div class="space-y-6">
            <h2 class="text-5xl xl:text-6xl font-bold text-white leading-tight">
              机构级
              <span class="bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
                量化交易
              </span>
              <br />基础设施
            </h2>
            <p class="text-xl text-slate-300 leading-relaxed max-w-lg">
              为专业交易者打造的超低延迟执行引擎，提供毫秒级响应和企业级安全保障
            </p>
          </div>

          <!-- 特性展示 -->
          <div class="grid grid-cols-2 gap-6">
            <div class="group p-6 bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10 hover:border-blue-400/30 transition-all duration-300">
              <div class="flex items-center space-x-3 mb-3">
                <div class="w-10 h-10 bg-blue-500/20 rounded-lg flex items-center justify-center group-hover:bg-blue-500/30 transition-colors">
                  <Zap class="w-5 h-5 text-blue-400" />
                </div>
                <h3 class="text-white font-semibold">超低延迟</h3>
              </div>
              <p class="text-slate-400 text-sm">平均延迟 &lt; 1ms</p>
              <div class="mt-3 flex items-center space-x-2">
                <div class="flex-1 h-1 bg-slate-700 rounded-full overflow-hidden">
                  <div class="h-full bg-gradient-to-r from-green-400 to-blue-400 w-4/5 animate-pulse"></div>
                </div>
                <span class="text-xs text-green-400 font-mono">99.9%</span>
              </div>
            </div>

            <div class="group p-6 bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10 hover:border-purple-400/30 transition-all duration-300">
              <div class="flex items-center space-x-3 mb-3">
                <div class="w-10 h-10 bg-purple-500/20 rounded-lg flex items-center justify-center group-hover:bg-purple-500/30 transition-colors">
                  <Shield class="w-5 h-5 text-purple-400" />
                </div>
                <h3 class="text-white font-semibold">安全保障</h3>
              </div>
              <p class="text-slate-400 text-sm">银行级加密</p>
              <div class="mt-3 flex items-center space-x-2">
                <div class="flex-1 h-1 bg-slate-700 rounded-full overflow-hidden">
                  <div class="h-full bg-gradient-to-r from-purple-400 to-pink-400 w-full animate-pulse"></div>
                </div>
                <span class="text-xs text-purple-400 font-mono">AES-256</span>
              </div>
            </div>

            <div class="group p-6 bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10 hover:border-emerald-400/30 transition-all duration-300">
              <div class="flex items-center space-x-3 mb-3">
                <div class="w-10 h-10 bg-emerald-500/20 rounded-lg flex items-center justify-center group-hover:bg-emerald-500/30 transition-colors">
                  <BarChart3 class="w-5 h-5 text-emerald-400" />
                </div>
                <h3 class="text-white font-semibold">实时数据</h3>
              </div>
              <p class="text-slate-400 text-sm">全球市场覆盖</p>
              <div class="mt-3 flex items-center space-x-2">
                <div class="flex-1 h-1 bg-slate-700 rounded-full overflow-hidden">
                  <div class="h-full bg-gradient-to-r from-emerald-400 to-cyan-400 w-5/6 animate-pulse"></div>
                </div>
                <span class="text-xs text-emerald-400 font-mono">24/7</span>
              </div>
            </div>

            <div class="group p-6 bg-white/5 backdrop-blur-sm rounded-2xl border border-white/10 hover:border-orange-400/30 transition-all duration-300">
              <div class="flex items-center space-x-3 mb-3">
                <div class="w-10 h-10 bg-orange-500/20 rounded-lg flex items-center justify-center group-hover:bg-orange-500/30 transition-colors">
                  <Cpu class="w-5 h-5 text-orange-400" />
                </div>
                <h3 class="text-white font-semibold">AI 算法</h3>
              </div>
              <p class="text-slate-400 text-sm">智能策略优化</p>
              <div class="mt-3 flex items-center space-x-2">
                <div class="flex-1 h-1 bg-slate-700 rounded-full overflow-hidden">
                  <div class="h-full bg-gradient-to-r from-orange-400 to-red-400 w-3/4 animate-pulse"></div>
                </div>
                <span class="text-xs text-orange-400 font-mono">ML</span>
              </div>
            </div>
          </div>

          <!-- 实时数据展示 -->
          <div class="bg-black/20 backdrop-blur-sm rounded-2xl border border-white/10 p-6">
            <div class="flex items-center justify-between mb-4">
              <h3 class="text-white font-semibold flex items-center">
                <Activity class="w-5 h-5 mr-2 text-green-400" />
                实时市场数据
              </h3>
              <div class="flex items-center space-x-2">
                <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
                <span class="text-xs text-green-400 font-mono">LIVE</span>
              </div>
            </div>
            <div class="grid grid-cols-3 gap-4">
              <div class="text-center">
                <p class="text-xs text-slate-400 mb-1">BTC/USD</p>
                <p class="text-lg font-mono text-white">{{ btcPrice.toLocaleString() }}</p>
                <p class="text-xs text-green-400">+2.45%</p>
              </div>
              <div class="text-center">
                <p class="text-xs text-slate-400 mb-1">ETH/USD</p>
                <p class="text-lg font-mono text-white">{{ ethPrice.toLocaleString() }}</p>
                <p class="text-xs text-red-400">-1.23%</p>
              </div>
              <div class="text-center">
                <p class="text-xs text-slate-400 mb-1">延迟</p>
                <p class="text-lg font-mono text-white">{{ latency }}ms</p>
                <p class="text-xs text-blue-400">优秀</p>
              </div>
            </div>
          </div>
        </div>

        <!-- 底部信息 -->
        <div class="flex items-center justify-between text-sm text-slate-400">
          <div class="flex items-center space-x-6">
            <span class="flex items-center">
              <Globe class="w-4 h-4 mr-1" /> 全球服务
            </span>
            <span class="flex items-center">
              <ClockIcon class="w-4 h-4 mr-1" /> 24/7 支持
            </span>
          </div>
          <span>© 2024 QuantNexus. 企业级量化交易平台</span>
        </div>
      </div>

      <!-- 右侧 - 登录表单区 -->
      <div class="w-full lg:w-1/2 xl:w-2/5 flex items-center justify-center p-8 lg:p-12">
        <div class="w-full max-w-md">
          
          <!-- 移动端Logo -->
          <div class="lg:hidden flex items-center justify-center mb-8">
            <div class="w-16 h-16 bg-gradient-to-tr from-blue-400 via-blue-500 to-purple-600 rounded-2xl flex items-center justify-center shadow-2xl shadow-blue-500/25">
              <ChartIcon class="w-8 h-8 text-white" />
            </div>
          </div>

          <!-- 登录表单卡片 -->
          <div class="bg-white/10 backdrop-blur-xl rounded-3xl border border-white/20 shadow-2xl shadow-black/20 p-8 lg:p-10">
            
            <!-- 表单头部 -->
            <div class="text-center mb-8">
              <h2 class="text-3xl font-bold text-white mb-2">欢迎回来</h2>
              <p class="text-slate-300">登录您的 QuantNexus 交易账户</p>
            </div>

            <!-- 登录表单 -->
            <form @submit.prevent="handleLogin" class="space-y-6">
              
              <!-- 邮箱输入 -->
              <div class="space-y-2">
                <label class="block text-sm font-medium text-slate-300">邮箱地址</label>
                <div class="relative">
                  <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                    <Mail class="h-5 w-5 text-slate-400" />
                  </div>
                  <input
                    v-model="loginForm.email"
                    type="email"
                    required
                    class="block w-full pl-12 pr-4 py-4 bg-white/5 border border-white/20 rounded-2xl text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-400/50 focus:border-blue-400/50 transition-all duration-200 backdrop-blur-sm"
                    placeholder="输入您的邮箱地址"
                  />
                </div>
              </div>

              <!-- 密码输入 -->
              <div class="space-y-2">
                <label class="block text-sm font-medium text-slate-300">密码</label>
                <div class="relative">
                  <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none">
                    <Lock class="h-5 w-5 text-slate-400" />
                  </div>
                  <input
                    v-model="loginForm.password"
                    :type="showPassword ? 'text' : 'password'"
                    required
                    class="block w-full pl-12 pr-12 py-4 bg-white/5 border border-white/20 rounded-2xl text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-blue-400/50 focus:border-blue-400/50 transition-all duration-200 backdrop-blur-sm"
                    placeholder="输入您的密码"
                  />
                  <button
                    type="button"
                    @click="showPassword = !showPassword"
                    class="absolute inset-y-0 right-0 pr-4 flex items-center text-slate-400 hover:text-white transition-colors"
                  >
                    <EyeOff v-if="showPassword" class="h-5 w-5" />
                    <Eye v-else class="h-5 w-5" />
                  </button>
                </div>
              </div>

              <!-- 记住我和忘记密码 -->
              <div class="flex items-center justify-between">
                <label class="flex items-center">
                  <input
                    v-model="loginForm.remember"
                    type="checkbox"
                    class="w-4 h-4 text-blue-400 bg-white/10 border-white/20 rounded focus:ring-blue-400/50 focus:ring-2"
                  />
                  <span class="ml-2 text-sm text-slate-300">记住我</span>
                </label>
                <a href="#" class="text-sm text-blue-400 hover:text-blue-300 transition-colors">
                  忘记密码？
                </a>
              </div>

              <!-- 登录按钮 -->
              <button
                type="submit"
                :disabled="isLoading"
                class="w-full bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white font-semibold py-4 px-6 rounded-2xl transition-all duration-300 transform hover:scale-[1.02] active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed shadow-lg shadow-blue-500/25"
              >
                <span v-if="isLoading" class="flex items-center justify-center">
                  <Loader2 class="w-5 h-5 animate-spin mr-2" />
                  登录中...
                </span>
                <span v-else class="flex items-center justify-center">
                  登录交易终端
                  <ArrowRight class="w-5 h-5 ml-2" />
                </span>
              </button>
            </form>

            <!-- 分割线 -->
            <div class="my-8 flex items-center">
              <div class="flex-1 border-t border-white/20"></div>
              <span class="px-4 text-sm text-slate-400">或</span>
              <div class="flex-1 border-t border-white/20"></div>
            </div>

            <!-- 注册链接 -->
            <div class="text-center">
              <p class="text-slate-300">
                还没有账户？
                <button 
                  @click="showRegister = true"
                  class="text-blue-400 hover:text-blue-300 font-medium transition-colors ml-1"
                >
                  立即注册
                </button>
              </p>
            </div>

            <!-- 安全提示 -->
            <div class="mt-6 p-4 bg-blue-500/10 border border-blue-500/20 rounded-xl">
              <div class="flex items-start space-x-3">
                <ShieldCheck class="w-5 h-5 text-blue-400 mt-0.5 flex-shrink-0" />
                <div>
                  <p class="text-sm text-blue-300 font-medium">安全提示</p>
                  <p class="text-xs text-blue-200/80 mt-1">
                    我们使用银行级 AES-256 加密技术保护您的账户安全
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElNotification } from 'element-plus'
import { 
  BarChart3 as ChartIcon, Mail, Lock, Eye, EyeOff, ArrowRight, Loader2, 
  ShieldCheck, Zap, Shield, BarChart3, Cpu, Activity, 
  Globe, Zap as ClockIcon 
} from 'lucide-vue-next'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const userStore = useUserStore()

// 表单状态
const loginForm = reactive({
  email: '',
  password: '',
  remember: false
})

const showPassword = ref(false)
const isLoading = ref(false)
const showRegister = ref(false)

// 实时数据
const btcPrice = ref(43250)
const ethPrice = ref(2680)
const latency = ref(0.8)

// 背景粒子
const particles = ref<Array<{ x: number, y: number, delay: number, duration: number }>>([])

// 定时器
let priceTimer: NodeJS.Timeout | null = null

// 生成背景粒子
const generateParticles = () => {
  const newParticles = []
  for (let i = 0; i < 50; i++) {
    newParticles.push({
      x: Math.random() * 100,
      y: Math.random() * 100,
      delay: Math.random() * 10,
      duration: 15 + Math.random() * 20
    })
  }
  particles.value = newParticles
}

// 更新实时数据
const updatePrices = () => {
  btcPrice.value += (Math.random() - 0.5) * 100
  ethPrice.value += (Math.random() - 0.5) * 20
  latency.value = Number((0.5 + Math.random() * 1).toFixed(1))
}

// 处理登录
const handleLogin = async () => {
  try {
    isLoading.value = true
    
    // 模拟登录延迟
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    const success = await userStore.login({
      email: loginForm.email,
      password: loginForm.password
    })

    if (success) {
      ElNotification({
        title: '登录成功',
        message: '欢迎回来！正在跳转到交易终端...',
        type: 'success',
        duration: 3000,
        position: 'top-right'
      })
      
      setTimeout(() => {
        router.push('/trading')
      }, 1000)
    }
  } catch (error) {
    ElNotification({
      title: '登录失败',
      message: '邮箱或密码错误，请重试',
      type: 'error',
      duration: 3000,
      position: 'top-right'
    })
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  generateParticles()
  priceTimer = setInterval(updatePrices, 3000)
})

onUnmounted(() => {
  if (priceTimer) {
    clearInterval(priceTimer)
  }
})
</script>

<style scoped>
/* 浮动粒子动画 */
@keyframes float-particle {
  0%, 100% { 
    transform: translateY(0px) translateX(0px) scale(0.5);
    opacity: 0.2;
  }
  25% { 
    transform: translateY(-30px) translateX(10px) scale(0.8);
    opacity: 0.6;
  }
  50% { 
    transform: translateY(-60px) translateX(-5px) scale(1);
    opacity: 0.8;
  }
  75% { 
    transform: translateY(-30px) translateX(-15px) scale(0.8);
    opacity: 0.6;
  }
}

.animate-float-particle {
  animation: float-particle 20s ease-in-out infinite;
}

/* 自定义滚动条 */
::-webkit-scrollbar {
  width: 8px;
}

::-webkit-scrollbar-track {
  background: rgba(15, 23, 42, 0.1);
}

::-webkit-scrollbar-thumb {
  background: rgba(59, 130, 246, 0.3);
  border-radius: 4px;
}

::-webkit-scrollbar-thumb:hover {
  background: rgba(59, 130, 246, 0.5);
}
</style>