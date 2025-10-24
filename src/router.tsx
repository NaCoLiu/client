import { createHashRouter, Navigate } from "react-router";
import { LoginForm } from "./pages/login";
import Home from "./pages/home/home";
import { WindowsFrame } from "./Windows";
import { LauncherPage } from "./pages/home/launcher";
import { SettingsPage } from "./pages/home/settings";

import type { ExtendedRouteObject } from "./types/router";
import { Play, Settings } from "lucide-react";

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
    element: <Home />,

    children: [
      {
        path: "",
        element: <Navigate to="launcher" replace />,
      },

      {
        path: "launcher",
        element: <LauncherPage />,
        icon: <Play />,
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
    element: <WindowsFrame />,
  },
];

export const router = createHashRouter(routes);

// 导出路由配置以便其他组件访问icon等自定义字段
export { routes };
