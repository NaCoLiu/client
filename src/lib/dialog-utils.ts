import type { DialogConfig } from '@/types/dialog'

// 对话框工具函数
export class DialogUtils {
  private static showDialog: ((config: DialogConfig) => void) | null = null

  // 设置全局对话框函数（由Provider调用）
  static setShowDialog(showDialogFn: (config: DialogConfig) => void) {
    this.showDialog = showDialogFn
  }

  // 显示确认对话框
  static confirm(
    title: string,
    description?: string,
    options?: {
      confirmText?: string
      cancelText?: string
      onConfirm?: () => void | Promise<void>
      onCancel?: () => void
    }
  ) {
    if (!this.showDialog) {
      console.error('DialogUtils: showDialog not initialized')
      return
    }

    this.showDialog({
      title,
      description,
      confirmText: options?.confirmText || '确认',
      cancelText: options?.cancelText || '取消',
      onConfirm: options?.onConfirm,
      onCancel: options?.onCancel,
    })
  }

  // 显示提示对话框
  static alert(title: string, description?: string, confirmText?: string) {
    if (!this.showDialog) {
      console.error('DialogUtils: showDialog not initialized')
      return
    }

    this.showDialog({
      title,
      description,
      confirmText: confirmText || '确定',
    })
  }

  // 显示删除确认对话框
  static confirmDelete(
    title: string,
    description?: string,
    onConfirm?: () => void | Promise<void>
  ) {
    if (!this.showDialog) {
      console.error('DialogUtils: showDialog not initialized')
      return
    }

    this.showDialog({
      title,
      description,
      confirmText: '删除',
      cancelText: '取消',
      variant: 'destructive',
      onConfirm,
    })
  }

  // 显示成功提示
  static success(title: string, description?: string) {
    this.alert(title, description, '好的')
  }

  // 显示错误提示
  static error(title: string, description?: string) {
    this.alert(title || '错误', description, '我知道了')
  }

  // 显示警告提示
  static warning(title: string, description?: string) {
    this.alert(title, description, '我明白了')
  }
}

// Promise版本的对话框函数
export const dialog = {
  // 返回Promise的确认对话框
  confirm: (title: string, description?: string): Promise<boolean> => {
    return new Promise((resolve) => {
      DialogUtils.confirm(title, description, {
        onConfirm: () => resolve(true),
        onCancel: () => resolve(false),
      })
    })
  },

  // 返回Promise的提示对话框
  alert: (title: string, description?: string): Promise<void> => {
    return new Promise((resolve) => {
      DialogUtils.alert(title, description)
      // 对于alert，我们立即resolve，因为它只有一个按钮
      resolve()
    })
  },

  // 返回Promise的删除确认对话框
  confirmDelete: (title: string, description?: string): Promise<boolean> => {
    return new Promise((resolve) => {
      DialogUtils.confirmDelete(title, description, () => resolve(true))
      // 如果用户取消，resolve(false)在onCancel中处理
    })
  },
}