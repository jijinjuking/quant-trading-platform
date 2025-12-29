<template>
  <div class="login-container">
    <div class="login-card">
      <!-- Logo和标题 -->
      <div class="login-header">
        <div class="logo">
          <div class="logo-placeholder">📈</div>
        </div>
        <h1 class="title">AI量化交易平台</h1>
        <p class="subtitle">专业的量化交易解决方案</p>
      </div>

      <!-- 登录表单 -->
      <div class="login-form">
        <el-tabs v-model="activeTab" class="auth-tabs">
          <!-- 登录标签页 -->
          <el-tab-pane label="登录" name="login">
            <el-form
              ref="loginFormRef"
              :model="loginForm"
              :rules="loginRules"
              @submit.prevent="handleLogin"
            >
              <el-form-item prop="email">
                <el-input
                  v-model="loginForm.email"
                  placeholder="邮箱地址"
                  size="large"
                  prefix-icon="User"
                  :disabled="userStore.isLoading"
                />
              </el-form-item>
              
              <el-form-item prop="password">
                <el-input
                  v-model="loginForm.password"
                  type="password"
                  placeholder="密码"
                  size="large"
                  prefix-icon="Lock"
                  show-password
                  :disabled="userStore.isLoading"
                  @keyup.enter="handleLogin"
                />
              </el-form-item>

              <el-form-item>
                <div class="form-options">
                  <el-checkbox v-model="rememberMe">记住我</el-checkbox>
                  <el-link type="primary" :underline="false">忘记密码？</el-link>
                </div>
              </el-form-item>

              <el-form-item>
                <el-button
                  type="primary"
                  size="large"
                  :loading="userStore.isLoading"
                  @click="handleLogin"
                  class="login-btn"
                >
                  {{ userStore.isLoading ? '登录中...' : '登录' }}
                </el-button>
              </el-form-item>

              <!-- 测试按钮 -->
              <el-form-item>
                <el-button
                  type="success"
                  size="large"
                  plain
                  @click="fillTestAccount"
                  class="test-btn"
                  :disabled="userStore.isLoading"
                >
                  🧪 使用测试账号
                </el-button>
              </el-form-item>
            </el-form>
          </el-tab-pane>

          <!-- 注册标签页 -->
          <el-tab-pane label="注册" name="register">
            <el-form
              ref="registerFormRef"
              :model="registerForm"
              :rules="registerRules"
              @submit.prevent="handleRegister"
            >
              <el-form-item prop="username">
                <el-input
                  v-model="registerForm.username"
                  placeholder="用户名"
                  size="large"
                  prefix-icon="User"
                  :disabled="userStore.isLoading"
                />
              </el-form-item>

              <el-form-item prop="email">
                <el-input
                  v-model="registerForm.email"
                  placeholder="邮箱地址"
                  size="large"
                  prefix-icon="Message"
                  :disabled="userStore.isLoading"
                />
              </el-form-item>
              
              <el-form-item prop="password">
                <el-input
                  v-model="registerForm.password"
                  type="password"
                  placeholder="密码"
                  size="large"
                  prefix-icon="Lock"
                  show-password
                  :disabled="userStore.isLoading"
                />
              </el-form-item>

              <el-form-item prop="confirmPassword">
                <el-input
                  v-model="registerForm.confirmPassword"
                  type="password"
                  placeholder="确认密码"
                  size="large"
                  prefix-icon="Lock"
                  show-password
                  :disabled="userStore.isLoading"
                  @keyup.enter="handleRegister"
                />
              </el-form-item>

              <el-form-item>
                <el-checkbox v-model="agreeTerms" :disabled="userStore.isLoading">
                  我已阅读并同意
                  <el-link type="primary" :underline="false">《用户协议》</el-link>
                  和
                  <el-link type="primary" :underline="false">《隐私政策》</el-link>
                </el-checkbox>
              </el-form-item>

              <el-form-item>
                <el-button
                  type="primary"
                  size="large"
                  :loading="userStore.isLoading"
                  :disabled="!agreeTerms"
                  @click="handleRegister"
                  class="login-btn"
                >
                  {{ userStore.isLoading ? '注册中...' : '注册' }}
                </el-button>
              </el-form-item>
            </el-form>
          </el-tab-pane>
        </el-tabs>
      </div>

      <!-- 错误提示 -->
      <div v-if="userStore.lastError" class="error-message">
        <el-alert
          :title="userStore.lastError"
          type="error"
          :closable="false"
          show-icon
        />
      </div>

      <!-- 测试信息提示 -->
      <div class="test-info">
        <el-alert
          title="测试账号信息"
          type="info"
          :closable="false"
          show-icon
        >
          <template #default>
            <div class="test-accounts">
              <p><strong>管理员账号：</strong></p>
              <p>邮箱: admin@quantnexus.com</p>
              <p>密码: Admin123456</p>
              <br>
              <p><strong>普通用户：</strong></p>
              <p>邮箱: user@quantnexus.com</p>
              <p>密码: User123456</p>
              <br>
              <p><strong>测试交易员：</strong></p>
              <p>邮箱: trader@quantnexus.com</p>
              <p>密码: Trader123456</p>
            </div>
          </template>
        </el-alert>
      </div>

      <!-- 底部信息 -->
      <div class="login-footer">
        <p class="copyright">© 2024 AI量化交易平台. 保留所有权利.</p>
        <div class="features">
          <div class="feature-item">
            <el-icon><TrendCharts /></el-icon>
            <span>专业量化</span>
          </div>
          <div class="feature-item">
            <el-icon><Lock /></el-icon>
            <span>安全可靠</span>
          </div>
          <div class="feature-item">
            <el-icon><Lightning /></el-icon>
            <span>高速执行</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { ElMessage, ElNotification } from 'element-plus'
import { useUserStore } from '@/stores/user'
import type { FormInstance, FormRules } from 'element-plus'

const router = useRouter()
const userStore = useUserStore()

// 表单引用
const loginFormRef = ref<FormInstance>()
const registerFormRef = ref<FormInstance>()

// 活动标签页
const activeTab = ref('login')

// 登录表单
const loginForm = reactive({
  email: '',
  password: ''
})

// 注册表单
const registerForm = reactive({
  username: '',
  email: '',
  password: '',
  confirmPassword: ''
})

// 其他状态
const rememberMe = ref(false)
const agreeTerms = ref(false)

// 表单验证规则
const loginRules: FormRules = {
  email: [
    { required: true, message: '请输入邮箱地址', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 6, message: '密码长度不能少于6位', trigger: 'blur' }
  ]
}

const registerRules: FormRules = {
  username: [
    { required: true, message: '请输入用户名', trigger: 'blur' },
    { min: 3, max: 20, message: '用户名长度为3-20个字符', trigger: 'blur' },
    { pattern: /^[a-zA-Z0-9_]+$/, message: '用户名只能包含字母、数字和下划线', trigger: 'blur' }
  ],
  email: [
    { required: true, message: '请输入邮箱地址', trigger: 'blur' },
    { type: 'email', message: '请输入正确的邮箱格式', trigger: 'blur' }
  ],
  password: [
    { required: true, message: '请输入密码', trigger: 'blur' },
    { min: 8, message: '密码长度不能少于8位', trigger: 'blur' },
    { pattern: /^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/, message: '密码必须包含大小写字母和数字', trigger: 'blur' }
  ],
  confirmPassword: [
    { required: true, message: '请确认密码', trigger: 'blur' },
    {
      validator: (rule, value, callback) => {
        if (value !== registerForm.password) {
          callback(new Error('两次输入的密码不一致'))
        } else {
          callback()
        }
      },
      trigger: 'blur'
    }
  ]
}

// 填充测试账号
const fillTestAccount = () => {
  loginForm.email = 'admin@quantnexus.com'
  loginForm.password = 'Admin123456'
  ElMessage.success('已填充测试账号信息，点击登录即可')
}

// 处理登录
const handleLogin = async () => {
  if (!loginFormRef.value) return

  try {
    const valid = await loginFormRef.value.validate()
    if (!valid) return

    const success = await userStore.login({
      email: loginForm.email,
      password: loginForm.password
    })

    if (success) {
      ElNotification({
        title: '登录成功',
        message: `欢迎回来，${userStore.user?.username}！`,
        type: 'success',
        duration: 3000
      })

      // 跳转到交易页面
      router.push('/trading')
    }
  } catch (error) {
    ElMessage.error('登录失败，请检查用户名和密码')
  }
}

// 处理注册
const handleRegister = async () => {
  if (!registerFormRef.value) return

  try {
    const valid = await registerFormRef.value.validate()
    if (!valid) return

    if (!agreeTerms.value) {
      ElMessage.warning('请先同意用户协议和隐私政策')
      return
    }

    const success = await userStore.register(registerForm)

    if (success) {
      ElNotification({
        title: '注册成功',
        message: `欢迎加入，${userStore.user?.username}！`,
        type: 'success',
        duration: 3000
      })

      // 跳转到交易页面
      router.push('/trading')
    }
  } catch (error) {
    ElMessage.error('注册失败，请检查输入信息')
  }
}

// 组件挂载时检查认证状态
onMounted(async () => {
  // 如果已经登录，直接跳转到交易页面
  if (userStore.isAuthenticated) {
    router.push('/trading')
    return
  }

  // 尝试从本地存储恢复认证状态
  await userStore.checkAuth()
  if (userStore.isAuthenticated) {
    router.push('/trading')
  }
})
</script>

<style scoped>
.login-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.login-card {
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.1);
  padding: 40px;
  width: 100%;
  max-width: 420px;
  position: relative;
  overflow: hidden;
}

.login-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, #f0b90b, #f0b90b);
}

.login-header {
  text-align: center;
  margin-bottom: 32px;
}

.logo {
  margin-bottom: 16px;
}

.logo-placeholder {
  width: 64px;
  height: 64px;
  font-size: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0 auto;
}

.title {
  font-size: 28px;
  font-weight: 700;
  color: #1a1a1a;
  margin: 0 0 8px 0;
}

.subtitle {
  font-size: 16px;
  color: #666;
  margin: 0;
}

.login-form {
  margin-bottom: 24px;
}

.auth-tabs {
  margin-bottom: 24px;
}

.auth-tabs :deep(.el-tabs__header) {
  margin: 0 0 24px 0;
}

.auth-tabs :deep(.el-tabs__nav-wrap::after) {
  display: none;
}

.auth-tabs :deep(.el-tabs__item) {
  font-size: 16px;
  font-weight: 600;
  padding: 0 24px;
}

.auth-tabs :deep(.el-tabs__active-bar) {
  background-color: #f0b90b;
}

.auth-tabs :deep(.el-tabs__item.is-active) {
  color: #f0b90b;
}

.form-options {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.login-btn {
  width: 100%;
  height: 48px;
  font-size: 16px;
  font-weight: 600;
  background: linear-gradient(135deg, #f0b90b, #f0b90b);
  border: none;
}

.login-btn:hover {
  background: linear-gradient(135deg, #d4a309, #d4a309);
}

.test-btn {
  width: 100%;
  height: 44px;
  font-size: 14px;
  font-weight: 500;
  margin-top: 8px;
}

.test-info {
  margin-bottom: 24px;
}

.test-accounts {
  font-size: 13px;
  line-height: 1.4;
}

.test-accounts p {
  margin: 2px 0;
}

.test-accounts strong {
  color: #409eff;
}

.error-message {
  margin-bottom: 24px;
}

.login-footer {
  text-align: center;
  padding-top: 24px;
  border-top: 1px solid #f0f0f0;
}

.copyright {
  font-size: 14px;
  color: #999;
  margin: 0 0 16px 0;
}

.features {
  display: flex;
  justify-content: center;
  gap: 24px;
}

.feature-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 4px;
  font-size: 12px;
  color: #666;
}

.feature-item .el-icon {
  font-size: 20px;
  color: #f0b90b;
}

/* 响应式设计 */
@media (max-width: 480px) {
  .login-container {
    padding: 16px;
  }
  
  .login-card {
    padding: 24px;
  }
  
  .title {
    font-size: 24px;
  }
  
  .features {
    gap: 16px;
  }
}
</style>