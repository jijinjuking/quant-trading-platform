<template>
  <div class="settings-page">
    <div class="page-container">
      <div class="page-header">
        <h1>账户设置</h1>
        <p>管理您的账户偏好和安全设置</p>
      </div>
      
      <div class="settings-content">
        <div class="settings-sidebar">
          <div class="sidebar-menu">
            <div 
              v-for="item in menuItems"
              :key="item.key"
              :class="['menu-item', { active: activeTab === item.key }]"
              @click="activeTab = item.key"
            >
              <span>{{ item.label }}</span>
            </div>
          </div>
        </div>

        <div class="settings-main">
          <!-- 通用设置 -->
          <div v-if="activeTab === 'general'" class="tab-content">
            <div class="section-header">
              <h2>通用设置</h2>
            </div>
            <div class="settings-card">
              <div class="setting-item">
                <div class="setting-info">
                  <h3>主题</h3>
                  <p>选择您喜欢的界面主题</p>
                </div>
                <select v-model="settings.theme" class="setting-select">
                  <option value="dark">深色主题</option>
                  <option value="light">浅色主题</option>
                  <option value="auto">跟随系统</option>
                </select>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <h3>语言</h3>
                  <p>选择界面显示语言</p>
                </div>
                <select v-model="settings.language" class="setting-select">
                  <option value="zh-CN">简体中文</option>
                  <option value="en-US">English</option>
                  <option value="ja-JP">日本語</option>
                </select>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <h3>时区</h3>
                  <p>设置您的时区</p>
                </div>
                <select v-model="settings.timezone" class="setting-select">
                  <option value="Asia/Shanghai">北京时间 (UTC+8)</option>
                  <option value="America/New_York">纽约时间 (UTC-5)</option>
                  <option value="Europe/London">伦敦时间 (UTC+0)</option>
                </select>
              </div>
            </div>
          </div>

          <!-- 交易设置 -->
          <div v-if="activeTab === 'trading'" class="tab-content">
            <div class="section-header">
              <h2>交易设置</h2>
            </div>
            <div class="settings-card">
              <div class="setting-item">
                <div class="setting-info">
                  <h3>下单确认</h3>
                  <p>下单前显示确认对话框</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="settings.trading.confirmOrders">
                  <span class="slider"></span>
                </label>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <h3>声音提醒</h3>
                  <p>交易成功时播放提示音</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="settings.trading.soundEnabled">
                  <span class="slider"></span>
                </label>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <h3>默认杠杆</h3>
                  <p>新订单的默认杠杆倍数</p>
                </div>
                <select v-model="settings.trading.defaultLeverage" class="setting-select">
                  <option :value="1">1x</option>
                  <option :value="5">5x</option>
                  <option :value="10">10x</option>
                  <option :value="20">20x</option>
                </select>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <h3>风险等级</h3>
                  <p>设置您的风险承受能力</p>
                </div>
                <select v-model="settings.trading.riskLevel" class="setting-select">
                  <option value="LOW">保守</option>
                  <option value="MEDIUM">平衡</option>
                  <option value="HIGH">激进</option>
                </select>
              </div>
            </div>
          </div>

          <!-- 通知设置 -->
          <div v-if="activeTab === 'notifications'" class="tab-content">
            <div class="section-header">
              <h2>通知设置</h2>
            </div>
            <div class="settings-card">
              <div class="setting-item">
                <div class="setting-info">
                  <h3>邮件通知</h3>
                  <p>接收重要事件的邮件通知</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="settings.notifications.email">
                  <span class="slider"></span>
                </label>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <h3>推送通知</h3>
                  <p>接收浏览器推送通知</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="settings.notifications.push">
                  <span class="slider"></span>
                </label>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <h3>交易通知</h3>
                  <p>订单成交和状态变化通知</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="settings.notifications.trading">
                  <span class="slider"></span>
                </label>
              </div>
              
              <div class="setting-item">
                <div class="setting-info">
                  <h3>市场资讯</h3>
                  <p>接收市场新闻和分析</p>
                </div>
                <label class="toggle-switch">
                  <input type="checkbox" v-model="settings.notifications.news">
                  <span class="slider"></span>
                </label>
              </div>
            </div>
          </div>

          <!-- 安全设置 -->
          <div v-if="activeTab === 'security'" class="tab-content">
            <div class="section-header">
              <h2>安全设置</h2>
            </div>
            <div class="settings-card">
              <div class="security-item">
                <div class="security-info">
                  <h3>修改密码</h3>
                  <p>定期更换密码可以提高账户安全性</p>
                </div>
                <button class="security-btn">修改密码</button>
              </div>
              
              <div class="security-item">
                <div class="security-info">
                  <h3>双因素认证</h3>
                  <p>使用Google Authenticator或短信验证码</p>
                  <span class="security-status enabled">已启用</span>
                </div>
                <button class="security-btn">管理2FA</button>
              </div>
              
              <div class="security-item">
                <div class="security-info">
                  <h3>API密钥管理</h3>
                  <p>创建和管理API访问密钥</p>
                </div>
                <button class="security-btn">管理API</button>
              </div>
              
              <div class="security-item">
                <div class="security-info">
                  <h3>登录历史</h3>
                  <p>查看最近的登录记录</p>
                </div>
                <button class="security-btn">查看历史</button>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <div class="settings-actions">
        <button class="save-btn" @click="saveSettings">保存设置</button>
        <button class="reset-btn" @click="resetSettings">重置</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue'
import { useUserStore } from '@/stores/user'

const userStore = useUserStore()
const activeTab = ref('general')

const menuItems = [
  { key: 'general', label: '通用设置' },
  { key: 'trading', label: '交易设置' },
  { key: 'notifications', label: '通知设置' },
  { key: 'security', label: '安全设置' }
]

const settings = reactive({
  theme: 'dark',
  language: 'zh-CN',
  timezone: 'Asia/Shanghai',
  trading: {
    confirmOrders: true,
    soundEnabled: true,
    defaultLeverage: 10,
    riskLevel: 'MEDIUM'
  },
  notifications: {
    email: true,
    push: true,
    trading: true,
    news: false
  }
})

const saveSettings = () => {
  // 保存设置逻辑
  console.log('Saving settings:', settings)
  alert('设置已保存')
}

const resetSettings = () => {
  // 重置设置逻辑
  if (confirm('确定要重置所有设置吗？')) {
    Object.assign(settings, {
      theme: 'dark',
      language: 'zh-CN',
      timezone: 'Asia/Shanghai',
      trading: {
        confirmOrders: true,
        soundEnabled: true,
        defaultLeverage: 10,
        riskLevel: 'MEDIUM'
      },
      notifications: {
        email: true,
        push: true,
        trading: true,
        news: false
      }
    })
  }
}
</script>

<style lang="scss" scoped>
.settings-page {
  min-height: 100vh;
  background: #0b0e11;
  color: #eaecef;
  padding: 20px;
  
  .page-container {
    max-width: 1200px;
    margin: 0 auto;
  }
  
  .page-header {
    margin-bottom: 32px;
    
    h1 {
      font-size: 28px;
      font-weight: 600;
      margin: 0 0 8px 0;
    }
    
    p {
      color: #848e9c;
      margin: 0;
    }
  }
  
  .settings-content {
    display: flex;
    gap: 24px;
    margin-bottom: 32px;
    
    .settings-sidebar {
      width: 240px;
      flex-shrink: 0;
      
      .sidebar-menu {
        background: #1e2329;
        border-radius: 8px;
        padding: 8px;
        
        .menu-item {
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
        }
      }
    }
    
    .settings-main {
      flex: 1;
      
      .tab-content {
        .section-header {
          margin-bottom: 24px;
          
          h2 {
            font-size: 20px;
            font-weight: 600;
            margin: 0;
          }
        }
        
        .settings-card {
          background: #1e2329;
          border-radius: 8px;
          padding: 24px;
          
          .setting-item {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px 0;
            border-bottom: 1px solid #2b3139;
            
            &:last-child {
              border-bottom: none;
            }
            
            .setting-info {
              flex: 1;
              
              h3 {
                font-size: 16px;
                margin: 0 0 4px 0;
              }
              
              p {
                font-size: 12px;
                color: #848e9c;
                margin: 0;
              }
            }
            
            .setting-select {
              padding: 6px 12px;
              background: #2b3139;
              border: 1px solid #3c4043;
              border-radius: 4px;
              color: #eaecef;
              font-size: 14px;
              min-width: 120px;
            }
            
            .toggle-switch {
              position: relative;
              display: inline-block;
              width: 44px;
              height: 24px;
              
              input {
                opacity: 0;
                width: 0;
                height: 0;
                
                &:checked + .slider {
                  background-color: #f0b90b;
                  
                  &:before {
                    transform: translateX(20px);
                  }
                }
              }
              
              .slider {
                position: absolute;
                cursor: pointer;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background-color: #3c4043;
                transition: .4s;
                border-radius: 24px;
                
                &:before {
                  position: absolute;
                  content: "";
                  height: 18px;
                  width: 18px;
                  left: 3px;
                  bottom: 3px;
                  background-color: white;
                  transition: .4s;
                  border-radius: 50%;
                }
              }
            }
          }
          
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
              flex: 1;
              
              h3 {
                font-size: 16px;
                margin: 0 0 4px 0;
              }
              
              p {
                font-size: 12px;
                color: #848e9c;
                margin: 0;
              }
              
              .security-status {
                display: inline-block;
                margin-top: 4px;
                padding: 2px 8px;
                border-radius: 12px;
                font-size: 11px;
                font-weight: 500;
                
                &.enabled {
                  background: rgba(2, 192, 118, 0.2);
                  color: #02c076;
                }
              }
            }
            
            .security-btn {
              padding: 8px 16px;
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
        }
      }
    }
  }
  
  .settings-actions {
    display: flex;
    gap: 12px;
    justify-content: flex-end;
    
    .save-btn, .reset-btn {
      padding: 10px 20px;
      border: none;
      border-radius: 6px;
      font-size: 14px;
      font-weight: 500;
      cursor: pointer;
      transition: all 0.2s;
    }
    
    .save-btn {
      background: #f0b90b;
      color: #000;
      
      &:hover {
        background: #e6a809;
      }
    }
    
    .reset-btn {
      background: transparent;
      border: 1px solid #3c4043;
      color: #eaecef;
      
      &:hover {
        background: #3c4043;
      }
    }
  }
}
</style>