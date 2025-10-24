import React from "react";
import ReactDOM from "react-dom/client";
import { RouterProvider } from "react-router";
import { router } from "./router";
import "./main.css";
import { ThemeProvider } from "@/components/theme-provider";
import { WindowsControl } from "./components/windows-control";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <main className="relative font-family-mono flex flex-col w-screen h-screen">
        <video
          src="/background.mp4"
          autoPlay
          muted
          loop
          className="fixed inset-0 w-full h-full object-cover -z-1 hue-rotate-40"
        />
        <WindowsControl />
        <div className="w-full h-full flex-1 justify-center items-center flex flex-col gap-48">
          <RouterProvider router={router} />
        </div>
      </main>
    </ThemeProvider>
  </React.StrictMode>
);
