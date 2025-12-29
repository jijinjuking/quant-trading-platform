import { ref } from 'vue'

interface ConfirmOptions {
  title?: string
  message: string
  type?: 'info' | 'warning' | 'error' | 'success'
  confirmText?: string
  cancelText?: string
}

const visible = ref(false)
const options = ref<ConfirmOptions>({
  message: ''
})
const loading = ref(false)
let resolvePromise: ((value: boolean) => void) | null = null

export function useConfirm() {
  const confirm = (opts: ConfirmOptions): Promise<boolean> => {
    options.value = {
      title: '确认操作',
      type: 'warning',
      confirmText: '确定',
      cancelText: '取消',
      ...opts
    }
    visible.value = true
    loading.value = false

    return new Promise((resolve) => {
      resolvePromise = resolve
    })
  }

  const handleConfirm = () => {
    if (resolvePromise) {
      resolvePromise(true)
      resolvePromise = null
    }
    visible.value = false
  }

  const handleCancel = () => {
    if (resolvePromise) {
      resolvePromise(false)
      resolvePromise = null
    }
    visible.value = false
  }

  const setLoading = (value: boolean) => {
    loading.value = value
  }

  return {
    visible,
    options,
    loading,
    confirm,
    handleConfirm,
    handleCancel,
    setLoading
  }
}

// 全局单例
const globalConfirm = useConfirm()

export function showConfirm(opts: ConfirmOptions): Promise<boolean> {
  return globalConfirm.confirm(opts)
}

export function getGlobalConfirm() {
  return globalConfirm
}
