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

interface LoginFormProps {
  onLoginSuccess?: (secretKey: string) => void;
}

export function LoginForm({ onLoginSuccess }: LoginFormProps) {
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
      alert("请输入密钥");
      return;
    }

    setIsLoading(true);

    try {
      // 调用后端登录验证，返回 [success, expirationTimestamp]
      const [success, expirationTimestamp] = await invoke<[boolean, number]>(
        "login",
        { password: cardKey }
      );

      if (success) {
        // 登录成功后保存密钥和过期时间
        await saveSecretKey(cardKey);
        await saveExpirationTime(expirationTimestamp);

        await invoke("start_session_monitor", { password: cardKey });

        if (onLoginSuccess) {
          onLoginSuccess(cardKey);
        }

        // 跳转到首页
        navigate("/home");
      }
    } catch (error) {
      console.error("登录失败:", error);
      const errorMessage =
        typeof error === "string"
          ? error
          : error instanceof Error
          ? error.message
          : "未知错误";
      alert(`登录失败: ${errorMessage}`);
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Card className="w-full max-w-sm bg-black/30! backdrop-blur-md text-center rounded-sm">
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
            className="w-full cursor-pointer bg-white/20"
            disabled={isLoading}
            type="submit"
          >
            {isLoading ? "登录中..." : "登录"}
          </Button>
        </CardFooter>
      </form>
    </Card>
  );
}
