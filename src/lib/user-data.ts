import { invoke } from '@tauri-apps/api/core';

// 用户数据类型（可根据需要扩展）
export interface UserData {
  secret_key?: string;
  [key: string]: any; // 允许动态字段
}

/**
 * 读取 YAML 数据
 */
export async function loadUserData(): Promise<UserData> {
  return await invoke<UserData>('load_data');
}

/**
 * 保存数据到 YAML
 */
export async function saveUserData(data: UserData): Promise<void> {
  await invoke('save_data', { data });
}

/**
 * 获取密钥
 */
export async function getSecretKey(): Promise<string> {
  const data = await loadUserData();
  return data.secret_key || '';
}

/**
 * 保存密钥
 */
export async function saveSecretKey(secretKey: string): Promise<void> {
  const data = await loadUserData();
  data.secret_key = secretKey;
  await saveUserData(data);
}

/**
 * 清除密钥
 */
export async function clearSecretKey(): Promise<void> {
  const data = await loadUserData();
  delete data.secret_key;
  await saveUserData(data);
}

/**
 * 设置字段
 */
export async function setField(key: string, value: any): Promise<void> {
  const data = await loadUserData();
  data[key] = value;
  await saveUserData(data);
}

/**
 * 获取字段
 */
export async function getField(key: string): Promise<any> {
  const data = await loadUserData();
  return data[key];
}

/**
 * 删除字段
 */
export async function deleteField(key: string): Promise<void> {
  const data = await loadUserData();
  delete data[key];
  await saveUserData(data);
}

/**
 * 保存过期时间戳
 */
export async function saveExpirationTime(timestamp: number): Promise<void> {
  const data = await loadUserData();
  data.expiration_time = timestamp;
  await saveUserData(data);
}

/**
 * 获取过期时间戳
 */
export async function getExpirationTime(): Promise<number | null> {
  const data = await loadUserData();
  return data.expiration_time || null;
}

/**
 * 检查会话是否过期（调用后端验证）
 * 如果已过期返回 null，未过期返回时间戳
 */
export async function checkSessionExpiration(): Promise<number | null> {
  const expirationTime = await getExpirationTime();
  if (!expirationTime) {
    return null; // 没有过期时间
  }
  
  try {
    // 调用后端检查是否过期
    const timestamp = await invoke<number>('check_session_expiration', { 
      expirationTimestamp: expirationTime 
    });
    return timestamp; // 未过期，返回时间戳
  } catch (error) {
    // 已过期或检查失败
    return null;
  }
}
