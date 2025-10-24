import { createHashRouter, Navigate } from "react-router";
import Login from "./Login";
import Home from "./Home";
import { WindowsFrame } from "./Windows";

export const router = createHashRouter([
  {
    path: "/",
    element: <Navigate to="/login" replace />,
  },
  {
    path: "/login",
    element: <Login />,
  },
  {
    path: "/home",
    element: <Home />,
  },
  {
    path: "/windows",
    element: <WindowsFrame />,
  }
]);
