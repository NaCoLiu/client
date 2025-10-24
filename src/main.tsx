import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router";
import { router } from "./router";
import "./main.css";
import { ThemeProvider } from "@/components/theme-provider";
import { GlobalDialogProvider } from "@/components/global-dialog";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <GlobalDialogProvider>
        <main className="relative font-family-mono flex flex-col w-screen h-screen">
          <RouterProvider router={router} />
        </main>
      </GlobalDialogProvider>
    </ThemeProvider>
  </React.StrictMode>
);
