import { createRouter, createWebHistory } from 'vue-router'
import { useUserStore } from '@/stores/user'
import TradingDashboard from '@/views/TradingDashboard.vue'
import LoginPage from '@/views/LoginPage.vue'
import QuantNexusLogin from '@/views/QuantNexusLogin.vue'
import TestPage from '@/views/TestPage.vue'
import SimpleTest from '@/views/SimpleTest.vue'

const routes = [
  {
    path: '/',
    redirect: '/login'
  },
  {
    path: '/login',
    name: 'Login',
    component: LoginPage,
    meta: { requiresAuth: false }
  },
  {
    path: '/login-old',
    name: 'LoginOld',
    component: LoginPage,
    meta: { requiresAuth: false }
  },
  {
    path: '/trading',
    name: 'Trading',
    component: TradingDashboard,
    meta: { requiresAuth: true }
  },
  {
    path: '/test',
    name: 'TestPage',
    component: TestPage,
    meta: { requiresAuth: false }
  },
  {
    path: '/simple',
    name: 'SimpleTest',
    component: SimpleTest,
    meta: { requiresAuth: false }
  },
  {
    path: '/chinese-test',
    name: 'ChineseTest',
    component: () => import('@/views/ChineseTest.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/component-test',
    name: 'ComponentTest',
    component: () => import('@/views/ComponentTest.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/login-quant',
    name: 'QuantLogin',
    component: QuantNexusLogin,
    meta: { requiresAuth: false }
  },
  {
    path: '/login-pro',
    name: 'ProfessionalLogin',
    component: () => import('@/views/ProfessionalLogin.vue'),
    meta: { requiresAuth: false }
  },
  {
    path: '/profile',
    name: 'Profile',
    component: () => import('@/views/ProfilePage.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/wallet',
    name: 'Wallet',
    component: () => import('@/views/WalletPage.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/orders',
    name: 'Orders',
    component: () => import('@/views/OrdersPage.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/strategies',
    name: 'Strategies',
    component: () => import('@/views/StrategiesPage.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('@/views/SettingsPage.vue'),
    meta: { requiresAuth: true }
  },
  {
    path: '/api-management',
    name: 'APIManagement',
    component: () => import('@/views/APIManagement.vue'),
    meta: { requiresAuth: true }
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

// 路由守卫 - 检查认证状态
router.beforeEach(async (to, from, next) => {
  const userStore = useUserStore()
  
  console.log(`🔍 Route guard: ${to.path}, current auth: ${userStore.isAuthenticated}`)
  
  // 检查路由是否需要认证
  const requiresAuth = to.meta.requiresAuth === true
  
  if (requiresAuth) {
    // 如果需要认证，先尝试恢复认证状态
    if (!userStore.isAuthenticated) {
      console.log('🔄 Trying to restore auth state...')
      await userStore.checkAuth()
    }
    
    // 如果仍未认证，重定向到登录页
    if (!userStore.isAuthenticated) {
      console.log('🔒 Route requires auth, redirecting to login')
      next('/login')
      return
    }
  }
  
  // 如果已认证且访问登录页，重定向到交易页
  if (userStore.isAuthenticated && (to.path === '/login' || to.path === '/')) {
    console.log('✅ User authenticated, redirecting to trading')
    next('/trading')
    return
  }
  
  console.log(`🚀 Navigating to ${to.path}`)
  next()
})

export default router