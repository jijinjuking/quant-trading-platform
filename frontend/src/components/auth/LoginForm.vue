<template>
  <div class="w-full max-w-md mx-auto animate-fadeIn">
    <div class="text-center mb-10">
      <div class="inline-flex items-center justify-center w-12 h-12 rounded-xl bg-quant-accent/10 text-quant-accent mb-4 ring-1 ring-quant-accent/20 shadow-lg shadow-blue-900/20">
          <ShieldCheck class="w-6 h-6" />
      </div>
      <h2 class="text-3xl font-bold tracking-tight text-white mb-2">欢迎回来</h2>
      <p class="text-gray-400">安全接入 QuantNexus 交易终端</p>
    </div>

    <form @submit.prevent="handleSubmit" class="space-y-6">
      <div class="space-y-4">
        <!-- 邮箱输入 -->
        <div class="group relative">
          <label class="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
            工作邮箱
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
              placeholder="trader@quantnexus.com"
            />
          </div>
        </div>

        <!-- 密码输入 -->
        <div class="group relative">
          <label class="block text-xs font-medium text-gray-500 mb-1 ml-1 uppercase tracking-wider">
            密码
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
              placeholder="••••••••••••"
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
      </div>

      <!-- 记住我和忘记密码 -->
      <div class="flex items-center justify-between">
        <div class="flex items-center">
          <input
            v-model="rememberMe"
            id="remember-me"
            name="remember-me"
            type="checkbox"
            class="h-4 w-4 rounded border-quant-700 bg-quant-800 text-quant-accent focus:ring-quant-accent/50 focus:ring-offset-0 transition-colors cursor-pointer"
          />
          <label for="remember-me" class="ml-2 block text-sm text-gray-400 cursor-pointer select-none hover:text-gray-300 transition-colors">
            保持登录
          </label>
        </div>

        <div class="text-sm">
          <a href="#" class="font-medium text-quant-accent hover:text-blue-400 transition-colors">
            忘记密码？
          </a>
        </div>
      </div>

      <!-- 登录按钮 -->
      <button
        type="submit"
        :disabled="isLoading || isSuccess"
        :class="[
          'w-full flex justify-center items-center py-3 px-4 border border-transparent rounded-lg text-sm font-semibold text-white transition-all duration-300 transform focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-offset-quant-900 focus:ring-quant-accent',
          isSuccess
            ? 'bg-quant-success hover:bg-emerald-600 shadow-lg shadow-emerald-900/20'
            : isLoading
            ? 'bg-quant-accent/70 cursor-not-allowed'
            : 'bg-quant-accent hover:bg-blue-600 hover:shadow-lg hover:shadow-blue-900/20 hover:scale-[1.02] active:scale-[0.98]'
        ]"
      >
        <Loader2 v-if="isLoading" class="w-5 h-5 animate-spin" />
        <Check v-else-if="isSuccess" class="w-5 h-5 mr-2" />
        <span v-if="isSuccess">验证成功</span>
        <span v-else-if="isLoading">验证中...</span>
        <span v-else class="flex items-center">
          登录终端 <ArrowRight class="ml-2 w-4 h-4 transition-transform group-hover:translate-x-1" />
        </span>
      </button>
    </form>

    <!-- 底部链接 -->
    <div class="mt-8 pt-6 border-t border-quant-800">
      <p class="text-center text-xs text-gray-500 leading-relaxed">
        <span class="flex items-center justify-center mb-2">
          <Shield class="w-3 h-3 mr-1" />
          采用企业级端对端加密技术保护
        </span>
        还没有账号？ 
        <button 
          @click="$emit('register-click')" 
          class="ml-1 text-quant-accent hover:text-blue-400 font-medium transition-colors focus:outline-none underline-offset-2 hover:underline"
        >
          申请开通
        </button>
      </p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { Mail, Lock, ArrowRight, Eye, EyeOff, ShieldCheck, Loader2, Check, Shield } from 'lucide-vue-next'
import { useUserStore } from '@/stores/user'

// 事件定义
const emit = defineEmits<{
  'register-click': []
  'login-success': [userData: any]
}>()

const userStore = useUserStore()

// 表单数据
const formData = reactive({
  email: '',
  password: ''
})

// 状态管理
const showPassword = ref(false)
const rememberMe = ref(false)
const isLoading = ref(false)
const isSuccess = ref(false)

// 处理登录提交
const handleSubmit = async () => {
  try {
    isLoading.value = true
    
    // 模拟API调用延迟
    await new Promise(resolve => setTimeout(resolve, 2000))
    
    // 调用用户store的登录方法
    const success = await userStore.login({
      email: formData.email,
      password: formData.password
    })

    if (success) {
      isSuccess.value = true
      
      // 延迟一下显示成功状态
      setTimeout(() => {
        emit('login-success', userStore.user)
      }, 1000)
    }
  } catch (error) {
    console.error('Login failed:', error)
  } finally {
    isLoading.value = false
  }
}
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