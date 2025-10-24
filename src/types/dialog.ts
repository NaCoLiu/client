// 对话框配置接口
export interface DialogConfig {
  title: string
  description?: string
  confirmText?: string
  cancelText?: string
  variant?: 'default' | 'destructive'
  onConfirm?: () => void | Promise<void>
  onCancel?: () => void
}

// 对话框上下文接口
export interface DialogContextType {
  showDialog: (config: DialogConfig) => void
  showConfirm: (title: string, description?: string, onConfirm?: () => void | Promise<void>) => void
  showAlert: (title: string, description?: string) => void
  showDestructive: (title: string, description?: string, onConfirm?: () => void | Promise<void>) => void
}

// 预设对话框类型
export type DialogType = 'confirm' | 'alert' | 'destructive'

// 对话框状态
export interface DialogState {
  isOpen: boolean
  config: DialogConfig | null
  isLoading: boolean
}