import { useState, useEffect, useCallback } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface LoginInfo {
  password?: string;
  expiration_timestamp?: number;
  login_time?: number;
}

export interface LoginStatus {
  is_logged_in: boolean;
  user_info?: LoginInfo | null;
}

/**
 * 登录状态管理 Hook
 * 与 Rust 端的状态管理同步
 */
export function useLoginState() {
  const [loginStatus, setLoginStatus] = useState<LoginStatus>({
    is_logged_in: false,
    user_info: null,
  });
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // 从 Rust 端获取登录状态
  const fetchLoginStatus = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const status = await invoke<LoginStatus>("get_login_status");
      setLoginStatus(status);
    } catch (err) {
      setError(err instanceof Error ? err.message : "获取登录状态失败");
      console.error("获取登录状态失败:", err);
    } finally {
      setLoading(false);
    }
  }, []);

  // 设置登录状态
  const setLoginState = useCallback(
    async (isLoggedIn: boolean, userInfo?: LoginInfo | null) => {
      try {
        setError(null);
        await invoke("set_login_status", {
          isLoggedIn: isLoggedIn,
          userInfo: userInfo,
        });

        // 更新本地状态
        setLoginStatus({
          is_logged_in: isLoggedIn,
          user_info: userInfo,
        });
      } catch (err) {
        setError(err instanceof Error ? err.message : "设置登录状态失败");
        throw err;
      }
    },
    []
  );

  // 退出登录
  const logout = useCallback(async () => {
    try {
      setError(null);
      await invoke("logout");

      // 更新本地状态
      setLoginStatus({
        is_logged_in: false,
        user_info: null,
      });

      // 重新获取 Rust 端状态确保同步
      await fetchLoginStatus();
    } catch (err) {
      setError(err instanceof Error ? err.message : "退出登录失败");
      throw err;
    }
  }, [fetchLoginStatus]);

  // 检查登录状态是否有效（检查过期时间）
  const isLoginValid = useCallback(() => {
    if (
      !loginStatus.is_logged_in ||
      !loginStatus.user_info?.expiration_timestamp
    ) {
      return false;
    }

    const now = Date.now() / 1000; // 转换为秒
    return now < loginStatus.user_info.expiration_timestamp;
  }, [loginStatus]);

  // 初始化时获取登录状态
  useEffect(() => {
    const initializeLoginState = async () => {
      await fetchLoginStatus();
    };

    initializeLoginState();
  }, []); // 移除依赖项，只在组件首次挂载时运行

  // 定期检查登录状态（每分钟检查一次）
  useEffect(() => {
    const interval = setInterval(() => {
      if (loginStatus.is_logged_in && !isLoginValid()) {
        // 如果登录状态过期，自动退出登录
        logout();
      }
    }, 60000); // 每分钟检查一次

    return () => clearInterval(interval);
  }, [loginStatus.is_logged_in, isLoginValid, logout]);

  return {
    loginStatus,
    loading,
    error,
    isLoggedIn: loginStatus.is_logged_in,
    userInfo: loginStatus.user_info,
    isLoginValid,
    fetchLoginStatus,
    setLoginState,
    logout,
  };
}
