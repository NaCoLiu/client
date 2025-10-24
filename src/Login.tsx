// import { useEffect } from "react";

import { LoginForm } from "./components/auth";

import { WindowsControl } from "./components/windows-control";

function App() {
  // const preventRefresh = (e: KeyboardEvent) => {
  //   if (e.key === "F5" || (e.ctrlKey && e.key === "r")) {
  //     e.preventDefault();
  //   }
  // };

  // useEffect(() => {
  //   window.addEventListener("keydown", preventRefresh);
  //   return () => {
  //     window.removeEventListener("keydown", preventRefresh);
  //   };
  // }, []);

  // // 禁止右键菜单
  // useEffect(() => {
  //   const handleContextMenu = (e: MouseEvent) => {
  //     e.preventDefault();
  //   };
  //   window.addEventListener("contextmenu", handleContextMenu);
  //   return () => {
  //     window.removeEventListener("contextmenu", handleContextMenu);
  //   };
  // }, []);

  return (
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
        <LoginForm />
      </div>
    </main>
  );
}

export default App;
