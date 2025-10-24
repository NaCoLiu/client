import { useState, useEffect } from "react";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardFooter } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  saveSecretKey,
  getSecretKey,
  saveExpirationTime,
} from "@/lib/user-data";
import { invoke } from "@tauri-apps/api/core";
import { useNavigate } from "react-router";
import { DialogUtils } from "@/lib/dialog-utils";
import { DoorClosedLocked, Globe } from "lucide-react";
import { WindowsControl } from "@/components/windows-control";

export function LoginForm() {
  const [cardKey, setCardKey] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const navigate = useNavigate(); // 在组件顶层调用 hook

  // 组件加载时直接加载已保存的密钥（不检查过期）
  useEffect(() => {
    loadExistingKey();
  }, []);

  const loadExistingKey = async () => {
    try {
      const existingKey = await getSecretKey();
      if (existingKey) {
        setCardKey(existingKey);
      }
    } catch (error) {
      console.error("加载密钥失败:", error);
    }
  };

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();

    if (!cardKey.trim()) {
      DialogUtils.warning("输入错误", "请输入密钥后再进行登录");
      return;
    }

    setIsLoading(true);

    try {
      // 调用后端登录验证，返回 [success, expirationTimestamp]
      const [_, expirationTimestamp] = await invoke<[boolean, number]>(
        "login",
        { password: cardKey }
      );

      await saveSecretKey(cardKey);
      await saveExpirationTime(expirationTimestamp);

      await invoke("start_session_monitor", { password: cardKey });

      navigate("/home");
    } catch (error) {
      // 卡密验证失败
      DialogUtils.error(
        "卡密验证失败",
        error instanceof Error ? error.message : String(error)
      );

      // 清空输入框，让用户重新输入
      setCardKey("");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="w-full h-full bg-black/75 flex flex-col">
      <video
        src="/background.mp4"
        autoPlay
        muted
        loop
        className="fixed inset-0 w-full h-full object-cover -z-1 "
      />
      <div className="w-screen h-screen flex flex-col">
        <WindowsControl />
        <div className="flex-1 flex justify-center items-center">
          <Card className="w-full max-w-sm bg-black/30! gap-4 backdrop-blur-md text-center rounded-sm">
            <form onSubmit={handleSubmit}>
              <CardContent>
                <div className="flex items-center justify-center flex-col">
                  <Label htmlFor="cardkey" className="">
                    [ <span className="px-10">密钥</span> ]
                  </Label>
                  <Input
                    className="mt-5"
                    id="cardkey"
                    type="text"
                    placeholder="请输入您的 Card Key"
                    value={cardKey}
                    onChange={(e) => setCardKey(e.target.value)}
                    required
                    disabled={isLoading}
                  />
                </div>
              </CardContent>
              <CardFooter className="mt-4">
                <Button
                  variant="ghost"
                  className="w-full cursor-pointer bg-white/10 hover:bg-white/35!"
                  disabled={isLoading}
                  type="submit"
                >
                  {isLoading ? "登录中..." : "登录"}
                </Button>
              </CardFooter>
            </form>

            <div className="flex flex-row gap-5 justify-between">
              <Button variant="link" className="text-xs">
                <DoorClosedLocked />
                解绑HWID
              </Button>

              <Button variant="link" className="text-xs">
                <Globe />
                语言切换
              </Button>
            </div>
          </Card>
        </div>
      </div>
    </div>
  );
}
