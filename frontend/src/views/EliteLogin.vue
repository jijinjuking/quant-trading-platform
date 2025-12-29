<template>
  <div class="min-h-screen bg-gradient-to-br from-gray-900 via-blue-900 to-indigo-900 flex items-center justify-center p-4">
    
    <!-- 主容器 -->
    <div class="w-full max-w-6xl mx-auto grid lg:grid-cols-2 gap-8 items-center">
      
      <!-- 左侧 - 品牌展示 -->
      <div class="hidden lg:block space-y-8">
        
        <!-- Logo区域 -->
        <div class="text-center lg:text-left">
          <div class="inline-flex items-center space-x-4 mb-6">
            <div class="w-16 h-16 bg-gradient-to-tr from-blue-400 to-purple-500 rounded-2xl flex items-center justify-center shadow-2xl">
              <svg class="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 24 24">
                <path d="M3 13h8V3H9v6H3v4zm0 8h6v-6H3v6zm10 0h8V11h-2v6h-6v4zm5-16V3h-6v6h4V7h2z"/>
              </svg>
            </div>
            <div>
              <h1 class="text-3xl font-bold text-white">QuantNexus</h1>
              <p class="text-blue-300">Professional Trading Platform</p>
            </div>
          </div>
          
          <h2 class="text-4xl lg:text-5xl font-bold text-white leading-tight mb-4">
            机构级量化交易
            <span class="bg-gradient-to-r from-blue-400 to-purple-400 bg-clip-text text-transparent">
              基础设施
            </span>
          </h2>
          
          <p class="text-xl text-gray-300 mb-8 leading-relaxed">
            为专业交易者打造的超低延迟执行引擎，提供毫秒级响应和企业级安全保障
          </p>
        </div>

        <!-- 特性网格 -->
        <div class="grid grid-cols-2 gap-4">
          <div class="bg-white/10 backdrop-blur-sm rounded-xl p-6 border border-white/20">
            <div class="w-12 h-12 bg-blue-500/20 rounded-lg flex items-center justify-center mb-4">
              <svg class="w-6 h-6 text-blue-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
              </svg>
            </div>
            <h3 class="text-white font-semibold mb-2">超低延迟</h3>
            <p class="text-gray-400 text-sm">平均延迟 &lt; 1ms</p>
          </div>
          
          <div class="bg-white/10 backdrop-blur-sm rounded-xl p-6 border border-white/20">
            <div class="w-12 h-12 bg-green-500/20 rounded-lg flex items-center justify-center mb-4">
              <svg class="w-6 h-6 text-green-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
              </svg>
            </div>
            <h3 class="text-white font-semibold mb-2">安全保障</h3>
            <p class="text-gray-400 text-sm">银行级加密</p>
          </div>
          
          <div class="bg-white/10 backdrop-blur-sm rounded-xl p-6 border border-white/20">
            <div class="w-12 h-12 bg-purple-500/20 rounded-lg flex items-center justify-center mb-4">
              <svg class="w-6 h-6 text-purple-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
              </svg>
            </div>
            <h3 class="text-white font-semibold mb-2">实时数据</h3>
            <p class="text-gray-400 text-sm">全球市场覆盖</p>
          </div>
          
          <div class="bg-white/10 backdrop-blur-sm rounded-xl p-6 border border-white/20">
            <div class="w-12 h-12 bg-orange-500/20 rounded-lg flex items-center justify-center mb-4">
              <svg class="w-6 h-6 text-orange-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.663 17h4.673M12 3v1m6.364 1.636l-.707.707M21 12h-1M4 12H3m3.343-5.657l-.707-.707m2.828 9.9a5 5 0 117.072 0l-.548.547A3.374 3.374 0 0014 18.469V19a2 2 0 11-4 0v-.531c0-.895-.356-1.754-.988-2.386l-.548-.547z"/>
              </svg>
            </div>
            <h3 class="text-white font-semibold mb-2">AI 算法</h3>
            <p class="text-gray-400 text-sm">智能策略优化</p>
          </div>
        </div>

        <!-- 实时数据 -->
        <div class="bg-black/30 backdrop-blur-sm rounded-xl p-6 border border-white/20">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-white font-semibold">实时市场数据</h3>
            <div class="flex items-center space-x-2">
              <div class="w-2 h-2 bg-green-400 rounded-full animate-pulse"></div>
              <span class="text-green-400 text-sm font-mono">LIVE</span>
            </div>
          </div>
          <div class="grid grid-cols-3 gap-4 text-center">
            <div>
              <p class="text-gray-400 text-sm">BTC/USD</p>
              <p class="text-white font-mono text-lg">{{ btcPrice.toLocaleString() }}</p>
              <p class="text-green-400 text-sm">+2.45%</p>
            </div>
            <div>
              <p class="text-gray-400 text-sm">ETH/USD</p>
              <p class="text-white font-mono text-lg">{{ ethPrice.toLocaleString() }}</p>
              <p class="text-red-400 text-sm">-1.23%</p>
            </div>
            <div>
              <p class="text-gray-400 text-sm">延迟</p>
              <p class="text-white font-mono text-lg">{{ latency }}ms</p>
              <p class="text-blue-400 text-sm">优秀</p>
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧 - 登录表单 -->
      <div class="w-full max-w-md mx-auto lg:mx-0">
        
        <!-- 移动端Logo -->
        <div class="lg:hidden text-center mb-8">
          <div class="inline-flex items-center space-x-3 mb-4">
            <div class="w-12 h-12 bg-gradient-to-tr from-blue-400 to-purple-500 rounded-xl flex items-center justify-center">
              <svg class="w-6 h-6 text-white" fill="currentColor" viewBox="0 0 24 24">
                <path d="M3 13h8V3H9v6H3v4zm0 8h6v-6H3v6zm10 0h8V11h-2v6h-6v4zm5-16V3h-6v6h4V7h2z"/>
              </svg>
            </div>
            <div>
              <h1 class="text-2xl font-bold text-white">QuantNexus</h1>
              <p class="text-blue-300 text-sm">Professional Trading</p>
            </div>
          </div>
        </div>

        <!-- 登录卡片 -->
        <div class="bg-white/10 backdrop-blur-xl rounded-2xl border border-white/20 shadow-2xl p-8">
          
          <!-- 表单标题 -->
          <div class="text-center mb-8">
            <h2 class="text-2xl font-bold text-white mb-2">欢迎回来</h2>
            <p class="text-gray-300">登录您的交易账户</p>
          </div>

          <!-- 登录表单 -->
          <form @submit.prevent="handleLogin" class="space-y-6">
            
            <!-- 邮箱 -->
            <div>
              <label class="block text-sm font-medium text-gray-300 mb-2">邮箱地址</label>
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"/>
                  </svg>
                </div>
                <input
                  v-model="form.email"
                  type="email"
                  required
                  class="block w-full pl-10 pr-3 py-3 bg-white/5 border border-white/20 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                  placeholder="输入您的邮箱"
                />
              </div>
            </div>

            <!-- 密码 -->
            <div>
              <label class="block text-sm font-medium text-gray-300 mb-2">密码</label>
              <div class="relative">
                <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
                  <svg class="h-5 w-5 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
                  </svg>
                </div>
                <input
                  v-model="form.password"
                  :type="showPassword ? 'text' : 'password'"
                  required
                  class="block w-full pl-10 pr-10 py-3 bg-white/5 border border-white/20 rounded-xl text-white placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all"
                  placeholder="输入您的密码"
                />
                <button
                  type="button"
                  @click="showPassword = !showPassword"
                  class="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-400 hover:text-white"
                >
                  <svg v-if="showPassword" class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21"/>
                  </svg>
                  <svg v-else class="h-5 w-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/>
                  </svg>
                </button>
              </div>
            </div>

            <!-- 记住我 -->
            <div class="flex items-center justify-between">
              <label class="flex items-center">
                <input
                  v-model="form.remember"
                  type="checkbox"
                  class="w-4 h-4 text-blue-500 bg-white/10 border-white/20 rounded focus:ring-blue-500"
                />
                <span class="ml-2 text-sm text-gray-300">记住我</span>
              </label>
              <a href="#" class="text-sm text-blue-400 hover:text-blue-300">忘记密码？</a>
            </div>

            <!-- 登录按钮 -->
            <button
              type="submit"
              :disabled="isLoading"
              class="w-full bg-gradient-to-r from-blue-500 to-purple-600 hover:from-blue-600 hover:to-purple-700 text-white font-semibold py-3 px-4 rounded-xl transition-all duration-200 transform hover:scale-105 disabled:opacity-50 disabled:cursor-not-allowed shadow-lg"
            >
              <span v-if="isLoading" class="flex items-center justify-center">
                <svg class="animate-spin -ml-1 mr-3 h-5 w-5 text-white" fill="none" viewBox="0 0 24 24">
                  <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                  <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                </svg>
                登录中...
              </span>
              <span v-else>登录交易终端</span>
            </button>
          </form>

          <!-- 分割线 -->
          <div class="my-6 flex items-center">
            <div class="flex-1 border-t border-white/20"></div>
            <span class="px-4 text-sm text-gray-400">或</span>
            <div class="flex-1 border-t border-white/20"></div>
          </div>

          <!-- 注册链接 -->
          <div class="text-center">
            <p class="text-gray-300">
              还没有账户？
              <button class="text-blue-400 hover:text-blue-300 font-medium ml-1">
                立即注册
              </button>
            </p>
          </div>

          <!-- 安全提示 -->
          <div class="mt-6 p-4 bg-blue-500/10 border border-blue-500/20 rounded-xl">
            <div class="flex items-start space-x-3">
              <svg class="w-5 h-5 text-blue-400 mt-0.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"/>
              </svg>
              <div>
                <p class="text-sm text-blue-300 font-medium">安全保障</p>
                <p class="text-xs text-blue-200/80 mt-1">
                  采用银行级 AES-256 加密技术保护您的账户安全
                </p>
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
import { useUserStore } from '@/stores/user'

const router = useRouter()
const userStore = useUserStore()

// 表单数据
const form = reactive({
  email: '',
  password: '',
  remember: false
})

const showPassword = ref(false)
const isLoading = ref(false)

// 实时数据
const btcPrice = ref(43250)
const ethPrice = ref(2680)
const latency = ref(0.8)

let priceTimer: NodeJS.Timeout | null = null

// 更新价格数据
const updatePrices = () => {
  btcPrice.value += (Math.random() - 0.5) * 100
  ethPrice.value += (Math.random() - 0.5) * 20
  latency.value = Number((0.5 + Math.random() * 1).toFixed(1))
}

// 处理登录
const handleLogin = async () => {
  try {
    isLoading.value = true
    
    // 模拟登录
    await new Promise(resolve => setTimeout(resolve, 1500))
    
    const success = await userStore.login({
      email: form.email,
      password: form.password
    })

    if (success) {
      ElNotification({
        title: '登录成功',
        message: '欢迎回来！正在跳转...',
        type: 'success',
        duration: 2000
      })
      
      setTimeout(() => {
        router.push('/trading')
      }, 800)
    }
  } catch (error) {
    ElNotification({
      title: '登录失败',
      message: '邮箱或密码错误',
      type: 'error',
      duration: 3000
    })
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  priceTimer = setInterval(updatePrices, 3000)
})

onUnmounted(() => {
  if (priceTimer) {
    clearInterval(priceTimer)
  }
})
</script>

<style scoped>
/* 自定义动画 */
@keyframes float {
  0%, 100% { transform: translateY(0px); }
  50% { transform: translateY(-10px); }
}

.animate-float {
  animation: float 3s ease-in-out infinite;
}
</style>