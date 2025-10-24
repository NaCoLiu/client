# 全局对话框系统

这是一个基于React Context和Radix UI AlertDialog构建的全局对话框系统，提供了多种使用方式来显示确认框、提示框和删除确认框。

## 功能特性

- 🎯 **多种使用方式**：支持Hook、工具类和Promise三种使用方式
- 🎨 **自定义配置**：支持自定义标题、描述、按钮文本和回调函数
- ⚡ **异步支持**：支持异步确认操作，自动显示加载状态
- 🎭 **多种样式**：支持默认和危险（删除）两种样式
- 📱 **响应式设计**：自适应不同屏幕尺寸
- 🔧 **TypeScript支持**：完整的类型定义

## 安装和配置

### 1. 在应用根部添加Provider

```tsx
import { GlobalDialogProvider } from "@/components/global-dialog";

function App() {
  return (
    <GlobalDialogProvider>
      {/* 你的应用内容 */}
    </GlobalDialogProvider>
  );
}
```

### 2. 导入必要的类型和工具

```tsx
import { useGlobalDialog, useConfirm, useAlert } from "@/components/global-dialog";
import { DialogUtils, dialog } from "@/lib/dialog-utils";
```

## 使用方式

### 方式一：Hook方式（推荐用于组件内）

```tsx
function MyComponent() {
  const { showDialog } = useGlobalDialog();
  const confirm = useConfirm();
  const alert = useAlert();
  const destructiveConfirm = useDestructiveConfirm();

  // 自定义对话框
  const handleCustomDialog = () => {
    showDialog({
      title: "自定义标题",
      description: "自定义描述内容",
      confirmText: "确定",
      cancelText: "取消",
      variant: "default", // 或 "destructive"
      onConfirm: async () => {
        // 异步操作
        await doSomething();
      },
      onCancel: () => {
        // 取消操作
      }
    });
  };

  // 快速确认框
  const handleConfirm = () => {
    confirm("确认操作", "您确定要执行此操作吗？", () => {
      console.log("用户确认了");
    });
  };

  // 快速提示框
  const handleAlert = () => {
    alert("提示", "操作完成！");
  };

  // 删除确认框
  const handleDelete = () => {
    destructiveConfirm("删除确认", "此操作不可撤销", async () => {
      await deleteItem();
    });
  };
}
```

### 方式二：工具类方式（推荐用于工具函数）

```tsx
import { DialogUtils } from "@/lib/dialog-utils";

// 在任何地方使用，无需Hook
function someUtilFunction() {
  // 确认对话框
  DialogUtils.confirm("确认操作", "描述信息", {
    confirmText: "确定",
    cancelText: "取消",
    onConfirm: () => console.log("确认"),
    onCancel: () => console.log("取消")
  });

  // 提示对话框
  DialogUtils.alert("提示", "消息内容");

  // 删除确认
  DialogUtils.confirmDelete("删除确认", "此操作不可撤销", () => {
    // 删除逻辑
  });

  // 预设样式
  DialogUtils.success("成功", "操作成功完成");
  DialogUtils.error("错误", "发生了错误");
  DialogUtils.warning("警告", "请注意");
}
```

### 方式三：Promise方式（推荐用于异步流程）

```tsx
import { dialog } from "@/lib/dialog-utils";

async function handleAsyncOperation() {
  // 返回Promise的确认框
  const confirmed = await dialog.confirm("确认操作", "您确定要继续吗？");
  
  if (confirmed) {
    console.log("用户确认了");
    // 继续执行操作
    
    // 显示结果
    await dialog.alert("成功", "操作已完成");
  } else {
    console.log("用户取消了");
  }
}

async function handleDelete() {
  const confirmed = await dialog.confirmDelete("删除确认", "此操作不可撤销");
  
  if (confirmed) {
    await deleteItem();
    await dialog.alert("成功", "删除完成");
  }
}
```

## API 参考

### DialogConfig 接口

```tsx
interface DialogConfig {
  title: string;                          // 对话框标题
  description?: string;                   // 对话框描述
  confirmText?: string;                   // 确认按钮文本
  cancelText?: string;                    // 取消按钮文本
  variant?: 'default' | 'destructive';   // 样式变体
  onConfirm?: () => void | Promise<void>; // 确认回调
  onCancel?: () => void;                  // 取消回调
}
```

### Hook API

- `useGlobalDialog()` - 获取对话框上下文
- `useConfirm()` - 快速确认对话框Hook
- `useAlert()` - 快速提示对话框Hook
- `useDestructiveConfirm()` - 快速删除确认Hook

### 工具类API

- `DialogUtils.confirm()` - 确认对话框
- `DialogUtils.alert()` - 提示对话框
- `DialogUtils.confirmDelete()` - 删除确认对话框
- `DialogUtils.success()` - 成功提示
- `DialogUtils.error()` - 错误提示
- `DialogUtils.warning()` - 警告提示

### Promise API

- `dialog.confirm()` - 返回Promise<boolean>的确认框
- `dialog.alert()` - 返回Promise<void>的提示框
- `dialog.confirmDelete()` - 返回Promise<boolean>的删除确认框

## 最佳实践

1. **组件内使用Hook方式**：在React组件内部推荐使用Hook方式
2. **工具函数使用工具类**：在工具函数或非组件代码中使用DialogUtils
3. **异步流程使用Promise方式**：在需要根据用户选择决定后续流程的场景中使用Promise方式
4. **合理使用variant**：删除操作使用destructive样式，普通操作使用default样式
5. **提供清晰的描述**：为用户提供足够的上下文信息来做出决定

## 注意事项

- GlobalDialogProvider必须在组件树的顶层使用
- DialogUtils需要在Provider初始化后才能使用
- 同时只能显示一个对话框
- 异步操作会自动显示加载状态，确认按钮会被禁用