import { getCurrentWindow } from "@tauri-apps/api/window";
import { X, Minus } from "lucide-react";

export const WindowsControl = ({
  // 兼容性props 内部菜单页面使用fixed 定位
  fixed,
}: {
  fixed?: boolean;
}) => {
  const handleMinimize = () => {
    getCurrentWindow().minimize();
  };

  const handleClose = () => {
    getCurrentWindow().close();
  };
  return (
    <div
      data-tauri-drag-region
      className={`w-full h-8 flex justify-end border-b ${fixed ? "fixed top-0 left-0" : ""}`}
    >
      <div className="flex items-center">
        <button
          onClick={handleMinimize}
          className="p-1.5 hover:bg-gray-200/25 cursor-pointer "
        >
          <Minus className="w-5 h-5" />
        </button>
        <button
          onClick={handleClose}
          className="hover:bg-red-500! p-1.5 cursor-pointer"
        >
          <X className="w-5 h-5" />
        </button>
      </div>
    </div>
  );
};
