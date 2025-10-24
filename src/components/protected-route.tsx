import { useEffect } from 'react';
import { useNavigate } from 'react-router';
import { useLoginState } from '@/hooks/use-login-state';

interface ProtectedRouteProps {
  children: React.ReactNode;
}

/**
 * 路由保护组件
 * 检查用户是否已登录，如果未登录则重定向到登录页面
 */
export function ProtectedRoute({ children }: ProtectedRouteProps) {
  const { isLoggedIn, isLoginValid, loading } = useLoginState();
  const navigate = useNavigate();

  useEffect(() => {
    // 等待登录状态加载完成
    if (loading) return;

    // 如果未登录或登录已过期，重定向到登录页面
    if (!isLoggedIn || !isLoginValid()) {
      navigate('/login', { replace: true });
    }
  }, [isLoggedIn, isLoginValid, loading, navigate]);

  // 如果正在加载或未登录，显示加载状态或空内容
  if (loading || !isLoggedIn || !isLoginValid()) {
    return (
      <div className="w-full h-full flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-white mx-auto mb-4"></div>
          <p className="text-white/70">验证登录状态...</p>
        </div>
      </div>
    );
  }

  // 如果已登录且未过期，显示子组件
  return <>{children}</>;
}