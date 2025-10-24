import { createHashRouter, Navigate } from "react-router";
import Login from "./Login";

export const router = createHashRouter([
  {
    path: "/",
    element: <Navigate to="/login" replace />,
  },
  {
    path: "/login",
    element: <Login />,
  },
]);
