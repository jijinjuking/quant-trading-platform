<template>
  <div class="profile-page">
    <div class="profile-container">
      <div class="profile-header">
        <h1>个人中心</h1>
        <p>管理您的账户信息和偏好设置</p>
      </div>

      <div class="profile-content">
        <div class="profile-sidebar">
          <div class="sidebar-menu">
            <div 
              v-for="item in menuItems"
              :key="item.key"
              :class="['menu-item', { active: activeTab === item.key }]"
              @click="activeTab = item.key"
            >
              <component :is="item.icon" class="menu-icon" />
              <span>{{ item.label }}</span>
            </div>
          </div>
        </div>

        <div class="profile-main">
          <!-- 基本信息 -->
          <div v-if="activeTab === 'basic'" class="tab-content">
            <div class="section-header">
              <h2>基本信息</h2>
            </div>
            <div class="user-info-card">
              <div class="avatar-section">
                <img 
                  :src="userStore.user?.avatar || defaultAvatar" 
                  :alt="userStore.user?.username"
                  class="user-avatar"
                />
                <button class="change-avatar-btn">更换头像</button>
              </div>
              <div class="info-section">
                <div class="info-item">
                  <label>用户名</label>
                  <span>{{ userStore.user?.username }}</span>
                </div>
                <div class="info-item">
                  <label>邮箱</label>
                  <span>{{ userStore.user?.email }}</span>
                </div>
                <div class="info-item">
                  <label>用户等级</label>
                  <span class="user-role" :class="getRoleClass(userStore.user?.role)">
                    {{ getRoleText(userStore.user?.role) }}
                  </span>
                </div>
                <div class="info-item">
                  <label>注册时间</label>
                  <span>{{ formatDate(userStore.user?.createdAt) }}</span>
                </div>
                <div class="info-item">
                  <label>最后登录</label>
                  <span>{{ formatDate(userStore.user?.lastLoginAt) }}</span>
                </div>
              </div>
            </div>
          </div>

          <!-- KYC认证 -->
          <div v-if="activeTab === 'kyc'" class="tab-content">
            <div class="section-header">
              <h2>KYC认证</h2>
            </div>
            <div class="kyc-status-card">
              <div class="status-header">
                <div class="status-info">
                  <span class="status-label">认证状态</span>
                  <span class="status-value" :class="getKycStatusClass(userStore.user?.kyc.status)">
                    {{ getKycStatusText(userStore.user?.kyc.status) }}
                  </span>
                </div>
                <div class="level-info">
                  <span class="level-label">认证等级</span>
                  <span class="level-value">Level {{ userStore.user?.kyc.level }}</span>
                </div>
              </div>
              <div class="limits-info">
                <h3>当前限额</h3>
                <div class="limits-grid">
                  <div class="limit-item">
                    <span class="limit-label">日提现限额</span>
                    <span class="limit-value">{{ formatCurrency(userStore.user?.kyc.limits.dailyWithdraw) }}</span>
                  </div>
                  <div class="limit-item">
                    <span class="limit-label">月提现限额</span>
                    <span class="limit-value">{{ formatCurrency(userStore.user?.kyc.limits.monthlyWithdraw) }}</span>
                  </div>
                  <div class="limit-item">
                    <span class="limit-label">最大杠杆</span>
                    <span class="limit-value">{{ userStore.user?.kyc.limits.maxLeverage }}x</span>
                  </div>
                </div>
              </div>
            </div>
          </div>

          <!-- 安全设置 -->
          <div v-if="activeTab === 'security'" class="tab-content">
            <div class="section-header">
              <h2>安全设置</h2>
            </div>
            <div class="security-card">
              <div class="security-item">
                <div class="security-info">
                  <h3>登录密码</h3>
                  <p>定期更换密码可以提高账户安全性</p>
                </div>
                <button class="security-btn">修改密码</button>
              </div>
              <div class="security-item">
                <div class="security-info">
                  <h3>双因素认证</h3>
                  <p>使用Google Authenticator或短信验证码</p>
                </div>
                <button class="security-btn">设置2FA</button>
              </div>
              <div class="security-item">
                <div class="security-info">
                  <h3>API密钥</h3>
                  <p>管理您的API访问密钥</p>
                </div>
                <button class="security-btn">管理API</button>
              </div>
            </div>
          </div>

          <!-- 偏好设置 -->
          <div v-if="activeTab === 'preferences'" class="tab-content">
            <div class="section-header">
              <h2>偏好设置</h2>
            </div>
            <div class="preferences-card">
              <div class="pref-section">
                <h3>界面设置</h3>
                <div class="pref-item">
                  <label>主题</label>
                  <select v-model="preferences.theme">
                    <option value="dark">深色主题</option>
                    <option value="light">浅色主题</option>
                    <option value="auto">跟随系统</option>
                  </select>
                </div>
                <div class="pref-item">
                  <label>语言</label>
                  <select v-model="preferences.language">
                    <option value="zh-CN">简体中文</option>
                    <option value="en-US">English</option>
                    <option value="ja-JP">日本語</option>
                  </select>
                </div>
              </div>
              
              <div class="pref-section">
                <h3>交易设置</h3>
                <div class="pref-item">
                  <label>
                    <input 
                      type="checkbox" 
                      v-model="preferences.trading.confirmOrders"
                    />
                    下单前确认
                  </label>
                </div>
                <div class="pref-item">
                  <label>
                    <input 
                      type="checkbox" 
                      v-model="preferences.trading.soundEnabled"
                    />
                    启用声音提醒
                  </label>
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
import { ref, reactive } from 'vue'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()

// 响应式数据
const activeTab = ref('basic')
const defaultAvatar = 'https://api.dicebear.com/7.x/avataaars/svg?seed=default'

// 偏好设置
const preferences = reactive({
  theme: userStore.user?.preferences.theme || 'dark',
  language: userStore.user?.preferences.language || 'zh-CN',
  trading: {
    confirmOrders: userStore.user?.preferences.trading.confirmOrders || true,
    soundEnabled: userStore.user?.preferences.trading.soundEnabled || true
  }
})

// 菜单项
const menuItems = [
  { key: 'basic', label: '基本信息', icon: 'UserIcon' },
  { key: 'kyc', label: 'KYC认证', icon: 'ShieldIcon' },
  { key: 'security', label: '安全设置', icon: 'LockIcon' },
  { key: 'preferences', label: '偏好设置', icon: 'SettingsIcon' }
]

// 方法
const getRoleText = (role?: string) => {
  const roleMap = {
    'ADMIN': '管理员',
    'VIP': 'VIP用户',
    'USER': '普通用户'
  }
  return roleMap[role as keyof typeof roleMap] || '用户'
}

const getRoleClass = (role?: string) => {
  return {
    'role-admin': role === 'ADMIN',
    'role-vip': role === 'VIP',
    'role-user': role === 'USER'
  }
}

const getKycStatusText = (status?: string) => {
  const statusMap = {
    'APPROVED': '已认证',
    'PENDING': '审核中',
    'REJECTED': '被拒绝',
    'NONE': '未认证'
  }
  return statusMap[status as keyof typeof statusMap] || '未认证'
}

const getKycStatusClass = (status?: string) => {
  return {
    'status-approved': status === 'APPROVED',
    'status-pending': status === 'PENDING',
    'status-rejected': status === 'REJECTED',
    'status-none': status === 'NONE'
  }
}

const formatDate = (timestamp?: number) => {
  if (!timestamp) return '-'
  return new Date(timestamp).toLocaleDateString('zh-CN')
}

const formatCurrency = (amount?: number) => {
  if (!amount) return '0'
  return amount.toLocaleString('en-US') + ' USDT'
}
</script>

<style lang="scss" scoped>
.profile-page {
  min-height: 100vh;
  background: #0b0e11;
  color: #eaecef;
  
  .profile-container {
    max-width: 1200px;
    margin: 0 auto;
    padding: 20px;
  }
  
  .profile-header {
    margin-bottom: 32px;
    
    h1 {
      font-size: 28px;
      font-weight: 600;
      margin: 0 0 8px 0;
      color: #eaecef;
    }
    
    p {
      font-size: 14px;
      color: #848e9c;
      margin: 0;
    }
  }
  
  .profile-content {
    display: flex;
    gap: 24px;
    
    .profile-sidebar {
      width: 240px;
      flex-shrink: 0;
      
      .sidebar-menu {
        background: #1e2329;
        border-radius: 8px;
        padding: 8px;
        
        .menu-item {
          display: flex;
          align-items: center;
          gap: 12px;
          padding: 12px 16px;
          border-radius: 6px;
          cursor: pointer;
          transition: all 0.2s;
          font-size: 14px;
          color: #848e9c;
          
          &:hover {
            background: #2b3139;
            color: #eaecef;
          }
          
          &.active {
            background: #f0b90b;
            color: #000;
          }
          
          .menu-icon {
            width: 16px;
            height: 16px;
          }
        }
      }
    }
    
    .profile-main {
      flex: 1;
      
      .tab-content {
        .section-header {
          margin-bottom: 24px;
          
          h2 {
            font-size: 20px;
            font-weight: 600;
            margin: 0;
            color: #eaecef;
          }
        }
        
        .user-info-card, .kyc-status-card, .security-card, .preferences-card {
          background: #1e2329;
          border-radius: 8px;
          padding: 24px;
        }
        
        .user-info-card {
          display: flex;
          gap: 24px;
          
          .avatar-section {
            display: flex;
            flex-direction: column;
            align-items: center;
            gap: 12px;
            
            .user-avatar {
              width: 80px;
              height: 80px;
              border-radius: 50%;
              border: 3px solid #f0b90b;
            }
            
            .change-avatar-btn {
              padding: 6px 12px;
              background: transparent;
              border: 1px solid #3c4043;
              border-radius: 4px;
              color: #eaecef;
              font-size: 12px;
              cursor: pointer;
              
              &:hover {
                background: #3c4043;
              }
            }
          }
          
          .info-section {
            flex: 1;
            display: flex;
            flex-direction: column;
            gap: 16px;
            
            .info-item {
              display: flex;
              justify-content: space-between;
              align-items: center;
              
              label {
                font-size: 14px;
                color: #848e9c;
                font-weight: 500;
              }
              
              span {
                font-size: 14px;
                color: #eaecef;
                
                &.user-role {
                  padding: 2px 8px;
                  border-radius: 12px;
                  font-size: 12px;
                  font-weight: 500;
                  
                  &.role-admin {
                    background: rgba(248, 73, 96, 0.2);
                    color: #f84960;
                  }
                  
                  &.role-vip {
                    background: rgba(240, 185, 11, 0.2);
                    color: #f0b90b;
                  }
                  
                  &.role-user {
                    background: rgba(132, 142, 156, 0.2);
                    color: #848e9c;
                  }
                }
              }
            }
          }
        }
        
        .kyc-status-card {
          .status-header {
            display: flex;
            justify-content: space-between;
            margin-bottom: 24px;
            
            .status-info, .level-info {
              display: flex;
              flex-direction: column;
              gap: 4px;
              
              .status-label, .level-label {
                font-size: 12px;
                color: #848e9c;
              }
              
              .status-value, .level-value {
                font-size: 16px;
                font-weight: 600;
                
                &.status-approved {
                  color: #02c076;
                }
                
                &.status-pending {
                  color: #f0b90b;
                }
                
                &.status-rejected {
                  color: #f84960;
                }
                
                &.status-none {
                  color: #848e9c;
                }
              }
            }
          }
          
          .limits-info {
            h3 {
              font-size: 16px;
              margin: 0 0 16px 0;
              color: #eaecef;
            }
            
            .limits-grid {
              display: grid;
              grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
              gap: 16px;
              
              .limit-item {
                display: flex;
                justify-content: space-between;
                padding: 12px;
                background: #2b3139;
                border-radius: 6px;
                
                .limit-label {
                  font-size: 12px;
                  color: #848e9c;
                }
                
                .limit-value {
                  font-size: 14px;
                  font-weight: 600;
                  color: #eaecef;
                }
              }
            }
          }
        }
        
        .security-card {
          .security-item {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px 0;
            border-bottom: 1px solid #2b3139;
            
            &:last-child {
              border-bottom: none;
            }
            
            .security-info {
              h3 {
                font-size: 16px;
                margin: 0 0 4px 0;
                color: #eaecef;
              }
              
              p {
                font-size: 12px;
                color: #848e9c;
                margin: 0;
              }
            }
            
            .security-btn {
              padding: 8px 16px;
              background: #f0b90b;
              border: none;
              border-radius: 4px;
              color: #000;
              font-size: 12px;
              font-weight: 500;
              cursor: pointer;
              
              &:hover {
                background: #e6a809;
              }
            }
          }
        }
        
        .preferences-card {
          .pref-section {
            margin-bottom: 24px;
            
            &:last-child {
              margin-bottom: 0;
            }
            
            h3 {
              font-size: 16px;
              margin: 0 0 16px 0;
              color: #eaecef;
            }
            
            .pref-item {
              display: flex;
              justify-content: space-between;
              align-items: center;
              margin-bottom: 12px;
              
              &:last-child {
                margin-bottom: 0;
              }
              
              label {
                font-size: 14px;
                color: #eaecef;
                display: flex;
                align-items: center;
                gap: 8px;
                
                input[type="checkbox"] {
                  width: 16px;
                  height: 16px;
                }
              }
              
              select {
                padding: 6px 12px;
                background: #2b3139;
                border: 1px solid #3c4043;
                border-radius: 4px;
                color: #eaecef;
                font-size: 14px;
              }
            }
          }
        }
      }
    }
  }
}

@media (max-width: 768px) {
  .profile-content {
    flex-direction: column;
    
    .profile-sidebar {
      width: 100%;
      
      .sidebar-menu {
        display: flex;
        overflow-x: auto;
        
        .menu-item {
          white-space: nowrap;
        }
      }
    }
  }
}
</style>