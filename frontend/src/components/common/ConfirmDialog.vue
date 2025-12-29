<template>
  <Teleport to="body">
    <Transition name="dialog-fade">
      <div v-if="visible" class="confirm-dialog-overlay" @click="handleCancel">
        <div class="confirm-dialog" @click.stop>
          <!-- 图标 -->
          <div class="dialog-icon" :class="iconClass">
            <svg v-if="type === 'warning'" viewBox="0 0 24 24" fill="none">
              <path d="M12 9v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <svg v-else-if="type === 'error'" viewBox="0 0 24 24" fill="none">
              <path d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <svg v-else-if="type === 'success'" viewBox="0 0 24 24" fill="none">
              <path d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
            <svg v-else viewBox="0 0 24 24" fill="none">
              <path d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>

          <!-- 标题 -->
          <h3 class="dialog-title">{{ title }}</h3>

          <!-- 内容 -->
          <div class="dialog-content">
            <p v-if="typeof message === 'string'" class="dialog-message">{{ message }}</p>
            <div v-else class="dialog-message" v-html="message"></div>
          </div>

          <!-- 按钮 -->
          <div class="dialog-actions">
            <button 
              class="dialog-btn dialog-btn-cancel" 
              @click="handleCancel"
              :disabled="loading"
            >
              {{ cancelText }}
            </button>
            <button 
              class="dialog-btn dialog-btn-confirm" 
              :class="confirmClass"
              @click="handleConfirm"
              :disabled="loading"
            >
              <span v-if="loading" class="loading-spinner"></span>
              {{ loading ? '处理中...' : confirmText }}
            </button>
          </div>
        </div>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'

interface Props {
  visible: boolean
  title?: string
  message: string
  type?: 'info' | 'warning' | 'error' | 'success'
  confirmText?: string
  cancelText?: string
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  title: '确认操作',
  type: 'warning',
  confirmText: '确定',
  cancelText: '取消',
  loading: false
})

const emit = defineEmits<{
  confirm: []
  cancel: []
  'update:visible': [value: boolean]
}>()

const iconClass = computed(() => {
  return {
    'icon-warning': props.type === 'warning',
    'icon-error': props.type === 'error',
    'icon-success': props.type === 'success',
    'icon-info': props.type === 'info'
  }
})

const confirmClass = computed(() => {
  return {
    'btn-warning': props.type === 'warning',
    'btn-error': props.type === 'error',
    'btn-success': props.type === 'success',
    'btn-info': props.type === 'info'
  }
})

const handleConfirm = () => {
  if (!props.loading) {
    emit('confirm')
  }
}

const handleCancel = () => {
  if (!props.loading) {
    emit('cancel')
    emit('update:visible', false)
  }
}
</script>

<style scoped lang="scss">
.confirm-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.6);
  backdrop-filter: blur(4px);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
  padding: 20px;
}

.confirm-dialog {
  background: linear-gradient(135deg, #1e2329 0%, #2b3139 100%);
  border-radius: 16px;
  padding: 32px;
  max-width: 480px;
  width: 100%;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.5);
  border: 1px solid rgba(255, 255, 255, 0.1);
  animation: dialog-enter 0.3s ease-out;
}

@keyframes dialog-enter {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(-20px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.dialog-icon {
  width: 64px;
  height: 64px;
  margin: 0 auto 24px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  
  svg {
    width: 36px;
    height: 36px;
  }
  
  &.icon-warning {
    background: rgba(234, 179, 8, 0.15);
    color: #eab308;
  }
  
  &.icon-error {
    background: rgba(239, 68, 68, 0.15);
    color: #ef4444;
  }
  
  &.icon-success {
    background: rgba(34, 197, 94, 0.15);
    color: #22c55e;
  }
  
  &.icon-info {
    background: rgba(59, 130, 246, 0.15);
    color: #3b82f6;
  }
}

.dialog-title {
  font-size: 24px;
  font-weight: 600;
  color: #eaecef;
  text-align: center;
  margin-bottom: 16px;
}

.dialog-content {
  margin-bottom: 32px;
}

.dialog-message {
  font-size: 15px;
  line-height: 1.6;
  color: #b7bdc6;
  text-align: center;
  white-space: pre-line;
}

.dialog-actions {
  display: flex;
  gap: 12px;
}

.dialog-btn {
  flex: 1;
  height: 48px;
  border-radius: 8px;
  font-size: 15px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s;
  border: none;
  outline: none;
  
  &:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }
  
  &:not(:disabled):hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }
  
  &:not(:disabled):active {
    transform: translateY(0);
  }
}

.dialog-btn-cancel {
  background: rgba(255, 255, 255, 0.05);
  color: #b7bdc6;
  border: 1px solid rgba(255, 255, 255, 0.1);
  
  &:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.08);
    color: #eaecef;
  }
}

.dialog-btn-confirm {
  color: white;
  position: relative;
  
  &.btn-warning {
    background: linear-gradient(135deg, #eab308 0%, #ca8a04 100%);
    box-shadow: 0 4px 12px rgba(234, 179, 8, 0.3);
  }
  
  &.btn-error {
    background: linear-gradient(135deg, #ef4444 0%, #dc2626 100%);
    box-shadow: 0 4px 12px rgba(239, 68, 68, 0.3);
  }
  
  &.btn-success {
    background: linear-gradient(135deg, #22c55e 0%, #16a34a 100%);
    box-shadow: 0 4px 12px rgba(34, 197, 94, 0.3);
  }
  
  &.btn-info {
    background: linear-gradient(135deg, #3b82f6 0%, #2563eb 100%);
    box-shadow: 0 4px 12px rgba(59, 130, 246, 0.3);
  }
}

.loading-spinner {
  display: inline-block;
  width: 16px;
  height: 16px;
  border: 2px solid rgba(255, 255, 255, 0.3);
  border-top-color: white;
  border-radius: 50%;
  animation: spin 0.6s linear infinite;
  margin-right: 8px;
  vertical-align: middle;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.3s;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
}
</style>
