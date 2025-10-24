import { useState } from 'react';
import { useUserData } from '@/lib/hooks/use-user-data';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Label } from '@/components/ui/label';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '@/components/ui/card';

/**
 * 用户数据管理示例组件
 */
export function UserDataExample() {
  const { data, loading, error, updateField, removeField } = useUserData();
  const [fieldKey, setFieldKey] = useState('');
  const [fieldValue, setFieldValue] = useState('');

  if (loading) {
    return <div>加载中...</div>;
  }

  const handleAddField = async () => {
    try {
      await updateField(fieldKey, fieldValue);
      setFieldKey('');
      setFieldValue('');
    } catch (err) {
      console.error('添加字段失败:', err);
    }
  };

  const handleRemoveField = async (key: string) => {
    try {
      await removeField(key);
    } catch (err) {
      console.error('删除字段失败:', err);
    }
  };

  // 快捷操作：保存密钥
  const handleSaveSecretKey = async () => {
    await updateField('secret_key', fieldValue);
    setFieldValue('');
  };

  return (
    <Card className="w-full max-w-2xl">
      <CardHeader>
        <CardTitle>用户数据管理</CardTitle>
        <CardDescription>使用 YAML 文件存储数据，在 React 中管理字段</CardDescription>
      </CardHeader>
      <CardContent className="space-y-6">
        {error && (
          <div className="text-red-500 text-sm">错误: {error}</div>
        )}

        {/* 当前数据 */}
        <div className="space-y-2">
          <h3 className="font-semibold">当前数据</h3>
          <div className="bg-muted p-4 rounded-lg">
            {Object.keys(data).length === 0 ? (
              <p className="text-muted-foreground text-sm">暂无数据</p>
            ) : (
              <div className="space-y-2">
                {Object.entries(data).map(([key, value]) => (
                  <div key={key} className="flex items-center justify-between text-sm">
                    <div className="flex items-center gap-2">
                      <code className="font-mono">{key}:</code>
                      <span>{String(value)}</span>
                    </div>
                    <Button
                      size="sm"
                      variant="ghost"
                      onClick={() => handleRemoveField(key)}
                    >
                      删除
                    </Button>
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>

        {/* 快捷操作：密钥 */}
        <div className="space-y-2">
          <h3 className="font-semibold">快捷操作：设置密钥</h3>
          <div className="flex gap-2">
            <Input
              type="text"
              placeholder="输入密钥..."
              value={fieldValue}
              onChange={(e) => setFieldValue(e.target.value)}
              className="flex-1"
            />
            <Button onClick={handleSaveSecretKey} disabled={!fieldValue}>
              保存密钥
            </Button>
          </div>
          {data.secret_key && (
            <div className="text-sm">
              当前密钥: <code className="bg-muted px-2 py-1 rounded">{data.secret_key}</code>
            </div>
          )}
        </div>

        {/* 添加自定义字段 */}
        <div className="space-y-2">
          <h3 className="font-semibold">添加自定义字段</h3>
          <div className="grid grid-cols-2 gap-2">
            <div>
              <Label htmlFor="field-key">字段名</Label>
              <Input
                id="field-key"
                type="text"
                placeholder="例如: username"
                value={fieldKey}
                onChange={(e) => setFieldKey(e.target.value)}
              />
            </div>
            <div>
              <Label htmlFor="field-value">字段值</Label>
              <Input
                id="field-value"
                type="text"
                placeholder="例如: John"
                value={fieldValue}
                onChange={(e) => setFieldValue(e.target.value)}
              />
            </div>
          </div>
          <Button onClick={handleAddField} disabled={!fieldKey || !fieldValue} className="w-full">
            添加字段
          </Button>
        </div>

        <div className="text-xs text-muted-foreground">
          <p>数据存储位置: %APPDATA%\Client\user_data.yml (Windows)</p>
          <p className="mt-1">所有字段的增删改查都在 React 端进行，Rust 只负责 YAML 文件读写</p>
        </div>
      </CardContent>
    </Card>
  );
}
