<template>
  <div class="user-header">
    <div class="user-info" v-if="userStore.isAuthenticated">
      <!-- 用户头像和基本信息 -->
      <div class="user-avatar-section" @click="toggleUserMenu">
        <img 
          :src="userStore.user?.avatar || defaultAvatar" 
          :alt="userStore.user?.username"
          class="user-avatar"
        />
        <div class="user-details">
          <div class="username">{{ userStore.user?.username }}</div>
          <div class="user-role" :class="roleClass">
            {{ getRoleText(userStore.user?.role) }}
          </div>
        </div>
        <div class="dropdown-arrow" :class="{ active: showUserMenu }">
          <svg width="12" height="12" viewBox="0 0 12 12" fill="currentColor">
            <path d="M6 8L2 4h8L6 8z"/>
          </svg>
        </div>
      </div>

      <!-- 用户菜单下拉框 -->
      <div v-if="showUserMenu" class="user-menu" @click.stop>
        <div class="menu-header">
          <div class="user-balance">
            <span class="balance-label">账户余额</span>
            <span class="balance-amount">{{ formatBalance(accountBalance) }} USDT</span>
          </div>
          <div class="kyc-status" :class="kycStatusClass">
            <span class="status-dot"></span>
            {{ getKycStatusText(userStore.user?.kyc.status) }}
          </div>
        </div>

        <div class="menu-divider"></div>

        <div class="menu-items">
          <div class="menu-item" @click="goToProfile">
            <svg class="menu-icon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zm2-3a2 2 0 1 1-4 0 2 2 0 0 1 4 0zm4 8c0 1-1 1-1 1H3s-1 0-1-1 1-4 6-4 6 3 6 4zm-1-.004c-.001-.246-.154-.986-.832-1.664C11.516 10.68 10.289 10 8 10c-2.29 0-3.516.68-4.168 1.332-.678.678-.83 1.418-.832 1.664h10z"/>
            </svg>
            <span>个人中心</span>
          </div>

          <div class="menu-item" @click="goToWallet">
            <svg class="menu-icon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M1 3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1H1zm7 8a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"/>
              <path d="M0 5a1 1 0 0 1 1-1h14a1 1 0 0 1 1 1v8a1 1 0 0 1-1 1H1a1 1 0 0 1-1-1V5zm3 0a2 2 0 0 1-2 2v4a2 2 0 0 1 2 2h10a2 2 0 0 1 2-2V7a2 2 0 0 1-2-2H3z"/>
            </svg>
            <span>我的钱包</span>
          </div>

          <div class="menu-item" @click="goToOrders">
            <svg class="menu-icon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M2.5 1a1 1 0 0 0-1 1v1a1 1 0 0 0 1 1H3v9a2 2 0 0 0 2 2h6a2 2 0 0 0 2-2V4h.5a1 1 0 0 0 1-1V2a1 1 0 0 0-1-1H10a1 1 0 0 0-1-1H7a1 1 0 0 0-1 1H2.5zm3 4a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 .5-.5zM8 5a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-1 0v-7A.5.5 0 0 1 8 5zm3 .5v7a.5.5 0 0 1-1 0v-7a.5.5 0 0 1 1 0z"/>
            </svg>
            <span>订单管理</span>
          </div>

          <div class="menu-item" @click="goToStrategies">
            <svg class="menu-icon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M1.5 1.5A.5.5 0 0 1 2 1h12a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.128.334L10 8.692V13.5a.5.5 0 0 1-.342.474l-3 1A.5.5 0 0 1 6 14.5V8.692L1.628 3.834A.5.5 0 0 1 1.5 3.5v-2z"/>
            </svg>
            <span>策略管理</span>
          </div>

          <div class="menu-item" @click="goToSettings">
            <svg class="menu-icon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path d="M8 4.754a3.246 3.246 0 1 0 0 6.492 3.246 3.246 0 0 0 0-6.492zM5.754 8a2.246 2.246 0 1 1 4.492 0 2.246 2.246 0 0 1-4.492 0z"/>
              <path d="M9.796 1.343c-.527-1.79-3.065-1.79-3.592 0l-.094.319a.873.873 0 0 1-1.255.52l-.292-.16c-1.64-.892-3.433.902-2.54 2.541l.159.292a.873.873 0 0 1-.52 1.255l-.319.094c-1.79.527-1.79 3.065 0 3.592l.319.094a.873.873 0 0 1 .52 1.255l-.16.292c-.892 1.64.901 3.434 2.541 2.54l.292-.159a.873.873 0 0 1 1.255.52l.094.319c.527 1.79 3.065 1.79 3.592 0l.094-.319a.873.873 0 0 1 1.255-.52l.292.16c1.64.893 3.434-.902 2.54-2.541l-.159-.292a.873.873 0 0 1 .52-1.255l.319-.094c1.79-.527 1.79-3.065 0-3.592l-.319-.094a.873.873 0 0 1-.52-1.255l.16-.292c.893-1.64-.902-3.433-2.541-2.54l-.292.159a.873.873 0 0 1-1.255-.52l-.094-.319z"/>
            </svg>
            <span>账户设置</span>
          </div>

          <div class="menu-divider"></div>

          <div class="menu-item logout-item" @click="handleLogout">
            <svg class="menu-icon" width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
              <path fill-rule="evenodd" d="M10 12.5a.5.5 0 0 1-.5.5h-8a.5.5 0 0 1-.5-.5v-9a.5.5 0 0 1 .5-.5h8a.5.5 0 0 1 .5.5v2a.5.5 0 0 0 1 0v-2A1.5 1.5 0 0 0 9.5 2h-8A1.5 1.5 0 0 0 0 3.5v9A1.5 1.5 0 0 0 1.5 14h8a1.5 1.5 0 0 0 1.5-1.5v-2a.5.5 0 0 0-1 0v2z"/>
              <path fill-rule="evenodd" d="M15.854 8.354a.5.5 0 0 0 0-.708l-3-3a.5.5 0 0 0-.708.708L14.293 7.5H5.5a.5.5 0 0 0 0 1h8.793l-2.147 2.146a.5.5 0 0 0 .708.708l3-3z"/>
            </svg>
            <span>退出登录</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 未登录状态 -->
    <div v-else class="login-prompt">
      <button class="login-btn" @click="goToLogin">
        登录
      </button>
      <button class="register-btn" @click="goToRegister">
        注册
      </button>
    </div>

    <!-- 确认退出对话框 -->
    <div v-if="showLogoutConfirm" class="logout-modal" @click="showLogoutConfirm = false">
      <div class="logout-dialog" @click.stop>
        <div class="dialog-header">
          <h3>确认退出</h3>
        </div>
        <div class="dialog-content">
          <p>您确定要退出登录吗？</p>
        </div>
        <div class="dialog-actions">
          <button class="cancel-btn" @click="showLogoutConfirm = false">
            取消
          </button>
          <button class="confirm-btn" @click="confirmLogout">
            确认退出
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useUserStore } from '@/stores/user'

const router = useRouter()
const userStore = useUserStore()

// 响应式数据
const showUserMenu = ref(false)
const showLogoutConfirm = ref(false)
const accountBalance = ref(10000) // 模拟账户余额

// 默认头像
const defaultAvatar = 'https://api.dicebear.com/7.x/avataaars/svg?seed=default'

// 计算属性
const roleClass = computed(() => {
  const role = userStore.user?.role
  return {
    'role-admin': role === 'ADMIN',
    'role-vip': role === 'VIP',
    'role-user': role === 'USER'
  }
})

const kycStatusClass = computed(() => {
  const status = userStore.user?.kyc.status
  return {
    'kyc-approved': status === 'APPROVED',
    'kyc-pending': status === 'PENDING',
    'kyc-rejected': status === 'REJECTED',
    'kyc-none': status === 'NONE'
  }
})

// 方法
const toggleUserMenu = () => {
  showUserMenu.value = !showUserMenu.value
}

const getRoleText = (role?: string) => {
  const roleMap = {
    'ADMIN': '管理员',
    'VIP': 'VIP用户',
    'USER': '普通用户'
  }
  return roleMap[role as keyof typeof roleMap] || '用户'
}

const getKycStatusText = (status?: string) => {
  const statusMap = {
    'APPROVED': 'KYC已认证',
    'PENDING': 'KYC审核中',
    'REJECTED': 'KYC被拒绝',
    'NONE': '未认证'
  }
  return statusMap[status as keyof typeof statusMap] || '未认证'
}

const formatBalance = (balance: number) => {
  return balance.toLocaleString('en-US', {
    minimumFractionDigits: 2,
    maximumFractionDigits: 2
  })
}

// 导航方法
const goToProfile = () => {
  showUserMenu.value = false
  router.push('/profile')
}

const goToWallet = () => {
  showUserMenu.value = false
  router.push('/wallet')
}

const goToOrders = () => {
  showUserMenu.value = false
  router.push('/orders')
}

const goToStrategies = () => {
  showUserMenu.value = false
  router.push('/strategies')
}

const goToSettings = () => {
  showUserMenu.value = false
  router.push('/settings')
}

const goToLogin = () => {
  router.push('/login')
}

const goToRegister = () => {
  router.push('/register')
}

const handleLogout = () => {
  showUserMenu.value = false
  showLogoutConfirm.value = true
}

const confirmLogout = async () => {
  try {
    await userStore.logout()
    showLogoutConfirm.value = false
    router.push('/login')
  } catch (error) {
    console.error('Logout failed:', error)
  }
}

// 点击外部关闭菜单
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as HTMLElement
  if (!target.closest('.user-header')) {
    showUserMenu.value = false
  }
}

onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style lang="scss" scoped>
.user-header {
  position: relative;
  display: flex;
  align-items: center;
  height: 100%;
  
  .user-info {
    position: relative;
    
    .user-avatar-section {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 6px 12px;
      border-radius: 6px;
      cursor: pointer;
      transition: all 0.2s;
      
      &:hover {
        background: rgba(240, 185, 11, 0.1);
      }
      
      .user-avatar {
        width: 32px;
        height: 32px;
        border-radius: 50%;
        border: 2px solid #f0b90b;
        object-fit: cover;
      }
      
      .user-details {
        display: flex;
        flex-direction: column;
        gap: 2px;
        
        .username {
          font-size: 14px;
          font-weight: 600;
          color: #eaecef;
          line-height: 1;
        }
        
        .user-role {
          font-size: 11px;
          font-weight: 500;
          line-height: 1;
          
          &.role-admin {
            color: #f84960;
          }
          
          &.role-vip {
            color: #f0b90b;
          }
          
          &.role-user {
            color: #848e9c;
          }
        }
      }
      
      .dropdown-arrow {
        color: #848e9c;
        transition: transform 0.2s;
        
        &.active {
          transform: rotate(180deg);
        }
      }
    }
    
    .user-menu {
      position: absolute;
      top: 100%;
      right: 0;
      width: 280px;
      background: #1e2329;
      border: 1px solid #2b3139;
      border-radius: 6px;
      box-shadow: 0 8px 24px rgba(0, 0, 0, 0.4);
      z-index: 1000;
      margin-top: 4px;
      
      .menu-header {
        padding: 16px;
        
        .user-balance {
          display: flex;
          justify-content: space-between;
          align-items: center;
          margin-bottom: 8px;
          
          .balance-label {
            font-size: 12px;
            color: #848e9c;
          }
          
          .balance-amount {
            font-size: 14px;
            font-weight: 600;
            color: #02c076;
            font-family: 'SF Mono', Monaco, monospace;
          }
        }
        
        .kyc-status {
          display: flex;
          align-items: center;
          gap: 6px;
          font-size: 12px;
          
          .status-dot {
            width: 8px;
            height: 8px;
            border-radius: 50%;
          }
          
          &.kyc-approved {
            color: #02c076;
            .status-dot {
              background: #02c076;
            }
          }
          
          &.kyc-pending {
            color: #f0b90b;
            .status-dot {
              background: #f0b90b;
            }
          }
          
          &.kyc-rejected {
            color: #f84960;
            .status-dot {
              background: #f84960;
            }
          }
          
          &.kyc-none {
            color: #848e9c;
            .status-dot {
              background: #848e9c;
            }
          }
        }
      }
      
      .menu-divider {
        height: 1px;
        background: #2b3139;
        margin: 0 16px;
      }
      
      .menu-items {
        padding: 8px 0;
        
        .menu-item {
          display: flex;
          align-items: center;
          gap: 12px;
          padding: 10px 16px;
          cursor: pointer;
          transition: background 0.2s;
          font-size: 14px;
          color: #eaecef;
          
          &:hover {
            background: #2b3139;
          }
          
          .menu-icon {
            color: #848e9c;
            flex-shrink: 0;
          }
          
          &.logout-item {
            color: #f84960;
            
            .menu-icon {
              color: #f84960;
            }
            
            &:hover {
              background: rgba(248, 73, 96, 0.1);
            }
          }
        }
      }
    }
  }
  
  .login-prompt {
    display: flex;
    align-items: center;
    gap: 8px;
    
    .login-btn, .register-btn {
      padding: 6px 16px;
      border-radius: 4px;
      font-size: 12px;
      font-weight: 500;
      cursor: pointer;
      transition: all 0.2s;
      border: none;
    }
    
    .login-btn {
      background: transparent;
      color: #eaecef;
      border: 1px solid #3c4043;
      
      &:hover {
        background: #3c4043;
      }
    }
    
    .register-btn {
      background: #f0b90b;
      color: #000;
      
      &:hover {
        background: #e6a809;
      }
    }
  }
}

// 退出确认对话框
.logout-modal {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 2000;
  
  .logout-dialog {
    background: #1e2329;
    border: 1px solid #2b3139;
    border-radius: 8px;
    width: 400px;
    max-width: 90vw;
    
    .dialog-header {
      padding: 20px 20px 0;
      
      h3 {
        margin: 0;
        font-size: 18px;
        font-weight: 600;
        color: #eaecef;
      }
    }
    
    .dialog-content {
      padding: 16px 20px;
      
      p {
        margin: 0;
        font-size: 14px;
        color: #848e9c;
        line-height: 1.5;
      }
    }
    
    .dialog-actions {
      padding: 0 20px 20px;
      display: flex;
      gap: 12px;
      justify-content: flex-end;
      
      .cancel-btn, .confirm-btn {
        padding: 8px 16px;
        border-radius: 4px;
        font-size: 14px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s;
        border: none;
      }
      
      .cancel-btn {
        background: transparent;
        color: #848e9c;
        border: 1px solid #3c4043;
        
        &:hover {
          background: #3c4043;
          color: #eaecef;
        }
      }
      
      .confirm-btn {
        background: #f84960;
        color: #fff;
        
        &:hover {
          background: #e63946;
        }
      }
    }
  }
}

@media (max-width: 768px) {
  .user-header {
    .user-info {
      .user-menu {
        width: 260px;
        right: -20px;
      }
    }
  }
}
</style>