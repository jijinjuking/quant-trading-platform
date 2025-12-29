<template>
  <div class="w-full max-w-md mx-auto animate-fadeIn">
    <div class="text-center mb-8">
      <div class="inline-flex items-center justify-center w-12 h-12 rounded-xl bg-quant-accent/10 text-quant-accent mb-4 ring-1 ring-quant-accent/20 shadow-lg shadow-blue-900/20">
          <UserCheck class="w-6 h-6" />
      </div>
      <h2 class="text-3xl font-bold tracking-tight text-white mb-2">申请开户</h2>
      <p class="text-gray-400">创建您的 QuantNexus 量化交易账户</p>
    </div>

    <form @submit.prevent="handleSubmit" class="space-y-5">
      
      <!-- 用户名输入 -->
      <div class="group relative">
        <label class="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
          用户名
        </label>
        <div class="relative">
          <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <User class="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
          </div>
          <input
            v-model="formData.username"
            type="text"
            required
            class="block w-full pl-10 pr-3 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm hover:border-quant-600"
            placeholder="输入用户名"
          />
        </div>
      </div>

      <!-- 邮箱输入 -->
      <div class="group relative">
        <label class="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
          电子邮箱
        </label>
        <div class="relative">
          <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Mail class="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
          </div>
          <input
            v-model="formData.email"
            type="email"
            required
            class="block w-full pl-10 pr-3 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm hover:border-quant-600"
            placeholder="name@company.com"
          />
        </div>
      </div>

      <!-- 手机号输入 -->
      <div class="group relative">
        <label class="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
          手机号码
        </label>
        <div class="relative">
          <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Phone class="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
          </div>
          <input
            v-model="formData.phone"
            type="tel"
            required
            class="block w-full pl-10 pr-3 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm hover:border-quant-600"
            placeholder="138 0000 0000"
          />
        </div>
      </div>

      <!-- 验证码输入 -->
      <div class="group relative">
        <label class="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
          短信验证码
        </label>
        <div class="flex space-x-3">
          <div class="relative flex-1">
            <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
              <MessageSquare class="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
            </div>
            <input
              v-model="formData.code"
              type="text"
              required
              maxlength="6"
              class="block w-full pl-10 pr-3 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm hover:border-quant-600"
              placeholder="000000"
            />
          </div>
          <button
            type="button"
            @click="handleSendCode"
            :disabled="countdown > 0 || !formData.phone"
            :class="[
              'px-4 py-3 rounded-lg text-sm font-medium transition-all duration-200 min-w-[120px] border border-quant-700',
              countdown > 0 || !formData.phone
                ? 'bg-quant-800 text-gray-500 cursor-not-allowed'
                : 'bg-quant-800 hover:bg-quant-700 text-quant-accent hover:text-blue-400 hover:border-quant-accent/50 hover:shadow-md'
            ]"
          >
            {{ countdown > 0 ? `${countdown}s 后重发` : '获取验证码' }}
          </button>
        </div>
      </div>

      <!-- 密码输入 -->
      <div class="group relative">
        <label class="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
          设置密码
        </label>
        <div class="relative">
          <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Lock class="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
          </div>
          <input
            v-model="formData.password"
            :type="showPassword ? 'text' : 'password'"
            required
            class="block w-full pl-10 pr-10 py-3 border border-quant-700 rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 focus:ring-quant-accent/50 focus:border-quant-accent transition-all duration-200 sm:text-sm backdrop-blur-sm hover:border-quant-600"
            placeholder="8-16位字符，包含字母和数字"
          />
          <button
            type="button"
            @click="showPassword = !showPassword"
            class="absolute inset-y-0 right-0 pr-3 flex items-center text-gray-500 hover:text-gray-300 focus:outline-none transition-colors"
          >
            <EyeOff v-if="showPassword" class="h-5 w-5" />
            <Eye v-else class="h-5 w-5" />
          </button>
        </div>
      </div>

      <!-- 确认密码输入 -->
      <div class="group relative">
        <label class="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
          确认密码
        </label>
        <div class="relative">
          <div class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none">
            <Lock class="h-5 w-5 text-gray-500 group-focus-within:text-quant-accent transition-colors duration-200" />
          </div>
          <input
            v-model="formData.confirmPassword"
            :type="showPassword ? 'text' : 'password'"
            required
            :class="[
              'block w-full pl-10 pr-3 py-3 border rounded-lg leading-5 bg-quant-800/50 text-gray-100 placeholder-gray-600 focus:outline-none focus:ring-2 transition-all duration-200 sm:text-sm backdrop-blur-sm hover:border-quant-600',
              passwordMatch ? 'border-quant-700 focus:ring-quant-accent/50 focus:border-quant-accent' : 'border-red-500/50 focus:ring-red-500/50 focus:border-red-500'
            ]"
            placeholder="再次输入密码"
          />
        </div>
        <p v-if="formData.confirmPassword && !passwordMatch" class="mt-1 text-xs text-red-400">
          密码不匹配
        </p>
      </div>

      <!-- 注册按钮 -->
      <button
        type="submit"
        :disabled="isLoading || isSuccess || !passwordMatch"
        :class="[
          'w-full flex justify-center items-center py-3 px-4 mt-6 border border-transparent rounded-lg text-sm font-semibold text-white transition-all duration-300 transform focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-quant-900 focus:ring-quant-accent',
          isSuccess
            ? 'bg-quant-success hover:bg-emerald-600 shadow-lg shadow-emerald-900/20'
            : isLoading || !passwordMatch
            ? 'bg-quant-accent/50 cursor-not-allowed'
            : 'bg-quant-accent hover:bg-blue-600 hover:shadow-lg hover:shadow-blue-900/20 hover:scale-[1.02] active:scale-[0.98]'
        ]"
      >
        <Loader2 v-if="isLoading" class="w-5 h-5 animate-spin" />
        <Check v-else-if="isSuccess" class="w-5 h-5 mr-2" />
        <span v-if="isSuccess">注册成功，跳转中...</span>
        <span v-else-if="isLoading">创建账户中...</span>
        <span v-else class="flex items-center">
          立即注册 <ArrowRight class="ml-2 w-4 h-4 transition-transform group-hover:translate-x-1" />
        </span>
      </button>
    </form>

    <!-- 底部链接 -->
    <div class="mt-8 pt-6 border-t border-quant-800 text-center">
      <p class="text-xs text-gray-500">
        已有账号？ 
        <button 
          @click="$emit('login-click')"
          class="ml-1 text-quant-accent hover:text-blue-400 font-medium transition-colors focus:outline-none underline-offset-2 hover:underline"
        >
          立即登录
        </button>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed, onUnmounted } from 'vue'
import { Mail, Lock, ArrowRight, Loader2, Check, Phone, MessageSquare, Eye, EyeOff, User, UserCheck } from 'lucide-vue-next'
import { useUserStore } from '@/stores/user'

// 事件定义
const emit = defineEmits<{
  'login-click': []
  'register-success': [userData: any]
}>()

const userStore = useUserStore()

// 表单数据
const formData = reactive({
  username: '',
  email: '',
  phone: '',
  code: '',
  password: '',
  confirmPassword: ''
})

// 状态管理
const showPassword = ref(false)
const isLoading = ref(false)
const isSuccess = ref(false)
const countdown = ref(0)

// 定时器
let countdownTimer: NodeJS.Timeout | null = null

// 计算属性
const passwordMatch = computed(() => {
  if (!formData.confirmPassword) return true
  return formData.password === formData.confirmPassword
})

// 发送验证码
const handleSendCode = () => {
  if (countdown.value === 0 && formData.phone) {
    // 模拟发送验证码
    countdown.value = 60
    
    countdownTimer = setInterval(() => {
      countdown.value--
      if (countdown.value <= 0) {
        clearInterval(countdownTimer!)
        countdownTimer = null
      }
    }, 1000)
  }
}

// 处理注册提交
const handleSubmit = async () => {
  // 验证密码匹配
  if (!passwordMatch.value) {
    return
  }

  try {
    isLoading.value = true
    
    // 模拟API调用延迟
    await new Promise(resolve => setTimeout(resolve, 2500))
    
    // 调用用户store的注册方法
    const success = await userStore.register({
      username: formData.username,
      email: formData.email,
      password: formData.password,
      confirmPassword: formData.confirmPassword
    })

    if (success) {
      isSuccess.value = true
      
      // 延迟一下显示成功状态
      setTimeout(() => {
        emit('register-success', userStore.user)
      }, 1500)
    }
  } catch (error) {
    console.error('Registration failed:', error)
  } finally {
    isLoading.value = false
  }
}

// 清理定时器
onUnmounted(() => {
  if (countdownTimer) {
    clearInterval(countdownTimer)
  }
})
</script>

<style scoped>
/* 渐入动画 */
.animate-fadeIn {
  animation: fadeIn 0.5s ease-out;
}

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

/* 输入框聚焦效果 */
.group:focus-within .group-focus-within\:text-quant-accent {
  color: #3B82F6;
}

/* 按钮悬停组效果 */
button:hover .group-hover\:translate-x-1 {
  transform: translateX(0.25rem);
}
</style>