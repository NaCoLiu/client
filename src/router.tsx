import { createBrowserRouter, Navigate } from "react-router";
import { LoginForm } from "./pages/login";
import Home from "./pages/home/home";
import { WindowsFrame } from "./Windows";
import { ProtectedRoute } from "./components/protected-route";

import { SettingsPage } from "./pages/home/settings";

import type { ExtendedRouteObject } from "./types/router";
import { Settings } from "lucide-react";

const routes: ExtendedRouteObject[] = [
  {
    path: "/",
    element: <Navigate to="/login" replace />,
  },
  {
    path: "/login",
    element: <LoginForm />,
  },
  {
    path: "/home",
    element: (
      <ProtectedRoute>
        <Home />
      </ProtectedRoute>
    ),
    children: [
      {
        index: true,
        element: <Navigate to="/home/settings" replace />,
      },
      {
        path: "settings",
        element: <SettingsPage />,
        icon: <Settings />,
      },
    ],
  },
  {
    path: "/windows",
    element: (
      <ProtectedRoute>
        <WindowsFrame />
      </ProtectedRoute>
    ),
  },
];

export const router = createBrowserRouter(routes);

// 导出路由配置以便其他组件访问icon等自定义字段
export { routes };
