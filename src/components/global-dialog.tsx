import {
  createContext,
  useState,
  ReactNode,
  useEffect,
  useCallback,
  useMemo,
} from "react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import type { DialogConfig, DialogContextType } from "@/types/dialog";
import { DialogUtils } from "@/lib/dialog-utils";

// 创建上下文
const DialogContext = createContext<DialogContextType | null>(null);

// 导出Context供外部使用
export { DialogContext };

// 全局对话框Provider组件
export function GlobalDialogProvider({ children }: { children: ReactNode }) {
  const [isOpen, setIsOpen] = useState(false);
  const [config, setConfig] = useState<DialogConfig | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  const showDialog = useCallback((dialogConfig: DialogConfig) => {
    setConfig(dialogConfig);
    setIsOpen(true);
  }, []);

  // 初始化DialogUtils
  useEffect(() => {
    DialogUtils.setShowDialog(showDialog);
  }, [showDialog]);

  const showConfirm = useCallback(
    (
      title: string,
      description?: string,
      onConfirm?: () => void | Promise<void>
    ) => {
      showDialog({
        title,
        description,
        confirmText: "确认",
        cancelText: "取消",
        onConfirm,
      });
    },
    [showDialog]
  );

  const showAlert = useCallback(
    (title: string, description?: string) => {
      showDialog({
        title,
        description,
        confirmText: "确定",
      });
    },
    [showDialog]
  );

  const showDestructive = useCallback(
    (
      title: string,
      description?: string,
      onConfirm?: () => void | Promise<void>
    ) => {
      showDialog({
        title,
        description,
        confirmText: "删除",
        cancelText: "取消",
        variant: "destructive",
        onConfirm,
      });
    },
    [showDialog]
  );

  const handleConfirm = useCallback(async () => {
    if (config?.onConfirm) {
      setIsLoading(true);
      try {
        await config.onConfirm();
      } catch (error) {
        console.error("Dialog confirm error:", error);
      } finally {
        setIsLoading(false);
      }
    }
    setIsOpen(false);
    setConfig(null);
  }, [config]);

  const handleCancel = useCallback(() => {
    if (config?.onCancel) {
      config.onCancel();
    }
    setIsOpen(false);
    setConfig(null);
  }, [config]);

  const handleOpenChange = useCallback(
    (open: boolean) => {
      if (!open) {
        handleCancel();
      }
    },
    [handleCancel]
  );

  const contextValue = useMemo(
    () => ({
      showDialog,
      showConfirm,
      showAlert,
      showDestructive,
    }),
    [showDialog, showConfirm, showAlert, showDestructive]
  );

  return (
    <DialogContext.Provider value={contextValue}>
      {children}

      <AlertDialog open={isOpen} onOpenChange={handleOpenChange}>
        <AlertDialogContent className="backdrop-blur-3xl bg-black/30!">
          <AlertDialogHeader>
            <AlertDialogTitle>{config?.title}</AlertDialogTitle>
            {config?.description && (
              <AlertDialogDescription>
                {config.description}
              </AlertDialogDescription>
            )}
          </AlertDialogHeader>

          <AlertDialogFooter>
            {config?.cancelText && (
              <AlertDialogCancel onClick={handleCancel} disabled={isLoading}>
                {config.cancelText}
              </AlertDialogCancel>
            )}
            {config?.confirmText && (
              <AlertDialogAction
                onClick={handleConfirm}
                disabled={isLoading}
                className={
                  config.variant === "destructive"
                    ? "cursor-pointer bg-destructive text-destructive-foreground hover:bg-destructive/90"
                    : "cursor-pointer"
                }
              >
                {isLoading ? "处理中..." : config.confirmText}
              </AlertDialogAction>
            )}
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </DialogContext.Provider>
  );
}

// 注意：hooks已移动到 src/hooks/use-dialog.ts 文件中
// 这样可以避免Fast Refresh问题
