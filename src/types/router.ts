import { LucideProps } from "lucide-react";
import { ForwardRefExoticComponent, RefAttributes, ReactElement } from "react";
import { RouteObject } from "react-router";

// 扩展路由对象类型，添加icon字段
export type ExtendedRouteObject = RouteObject & {
  icon?: 
    | ReactElement 
    | ForwardRefExoticComponent<Omit<LucideProps, "ref"> & RefAttributes<SVGSVGElement>>
    | string;
  children?: ExtendedRouteObject[];
};

// 路由配置的辅助类型
export interface RouteConfig {
  path: string;
  element: React.ReactElement;
  icon?: 
    | ReactElement 
    | ForwardRefExoticComponent<Omit<LucideProps, "ref"> & RefAttributes<SVGSVGElement>>
    | string;
  children?: RouteConfig[];
}
