import { getCurrentWindow } from "@tauri-apps/api/window";
import { X, Minus } from "lucide-react";

export const WindowsControl = () => {
  const handleMinimize = () => {
    getCurrentWindow().minimize();
  };

  const handleClose = () => {
    getCurrentWindow().close();
  };
  return (
    <div
      data-tauri-drag-region
      className="border-b flex items-center justify-end bg-black/10 backdrop-blur-xl relative"
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
