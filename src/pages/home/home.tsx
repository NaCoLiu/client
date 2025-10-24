import { WindowsControl } from "@/components/windows-control";
import { Outlet, useLocation, useNavigate } from "react-router";
import { routes } from "@/router";
import type { ExtendedRouteObject } from "@/types/router";
import { cloneElement } from "react";

export default function Home() {
  const location = useLocation();
  const navigate = useNavigate();


  // 获取home路由的子路由
  const homeRoute = routes.find(
    (route) => route.path === "/home"
  ) as ExtendedRouteObject;
  const childRoutes = (homeRoute?.children?.filter(
    (child) => child.path !== ""
  ) || []) as ExtendedRouteObject[];

 

  const renderIcon = (icon: ExtendedRouteObject["icon"]) => {
    return cloneElement(icon as any, { className: "w-5 h-5" });
  };

  const handleRouteClick = (routePath: string) => {
    const targetPath = `/home/${routePath}`;
    navigate(targetPath);
  };

  return (
    <div className="w-full h-full flex justify-center items-center flex-col">
      <WindowsControl fixed />
      <div className="flex-1 w-full h-full">
        <Outlet />
      </div>
      <div className=" bg-gray-400/15 border-t w-full justify-center items-center flex">
        {childRoutes.map((route) => (
          <div
            key={route.path}
            onClick={() => handleRouteClick(route.path || "")}
            className={`flex flex-col items-center justify-center p-3 rounded-none cursor-pointer transition-colors  ${
              location.pathname.includes(route.path || "")
                ? "bg-white/20 text-white"
                : " hover:bg-white/10 hover:text-white"
            }`}
          >
            {renderIcon(route.icon)}
          </div>
        ))}
      </div>
    </div>
  );
}
