import { getCurrentWindow } from "@tauri-apps/api/window";
import { X, Minus, LogOut } from "lucide-react";
import { useNavigate } from "react-router";
import { useLoginState } from "@/hooks/use-login-state";


export const WindowsControl = ({}: {}) => {
  const handleMinimize = () => {
    getCurrentWindow().minimize();
  };

  const handleClose = () => {
    getCurrentWindow().close();
  };

  const { isLoggedIn, logout } = useLoginState();
  const navigate = useNavigate();

  // 处理退出登录
  const handleLogout = async () => {
    console.log("退出前 isLoggedIn:", isLoggedIn);
    try {
      await logout();
      console.log("退出登录完成，导航到登录页");
      navigate("/login");
    } catch (error) {
      console.error("退出登录失败:", error);
      navigate("/login");
    }
  };

  return (
    <div
      data-tauri-drag-region
      className={`w-full h-8 flex justify-end border-b relative`}
    >
      <div className="flex items-center">
        {isLoggedIn && (
          <div
            className="flex justify-center px-2 items-center cursor-pointer"
            onClick={handleLogout}
            title="退出登录"
          >
            <LogOut className="w-4 h-4" />
          </div>
        )}
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
