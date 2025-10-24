import { useState, useEffect, useCallback } from 'react';
import { 
  UserData,
  loadUserData, 
  saveUserData,
  setField,
  getField,
  deleteField
} from '@/lib/user-data';

/**
 * 用户数据管理 Hook
 */
export function useUserData() {
  const [data, setData] = useState<UserData>({});
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // 加载数据
  const load = useCallback(async () => {
    try {
      setLoading(true);
      setError(null);
      const userData = await loadUserData();
      setData(userData);
    } catch (err) {
      setError(err instanceof Error ? err.message : '加载数据失败');
      console.error('加载数据失败:', err);
    } finally {
      setLoading(false);
    }
  }, []);

  // 初始化时加载数据
  useEffect(() => {
    load();
  }, [load]);

  // 保存数据
  const save = async (newData: UserData) => {
    try {
      setError(null);
      await saveUserData(newData);
      setData(newData);
    } catch (err) {
      setError(err instanceof Error ? err.message : '保存数据失败');
      throw err;
    }
  };

  // 更新字段
  const updateField = async (key: string, value: any) => {
    try {
      setError(null);
      await setField(key, value);
      await load(); // 重新加载数据
    } catch (err) {
      setError(err instanceof Error ? err.message : '更新字段失败');
      throw err;
    }
  };

  // 获取字段
  const getFieldValue = async (key: string) => {
    try {
      setError(null);
      return await getField(key);
    } catch (err) {
      setError(err instanceof Error ? err.message : '获取字段失败');
      throw err;
    }
  };

  // 删除字段
  const removeField = async (key: string) => {
    try {
      setError(null);
      await deleteField(key);
      await load(); // 重新加载数据
    } catch (err) {
      setError(err instanceof Error ? err.message : '删除字段失败');
      throw err;
    }
  };

  return {
    data,
    loading,
    error,
    reload: load,
    save,
    updateField,
    getFieldValue,
    removeField,
  };
}
