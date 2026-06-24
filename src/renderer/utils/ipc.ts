/**
 * IPC 通信封装
 * 统一的 IPC 调用和错误处理
 */

/** IPC 响应基础结构 */
interface IPCResponse {
  success: boolean;
  error?: string;
}

/**
 * 调用 IPC 并自动处理错误
 * @param channel IPC 通道名
 * @param args 传递参数
 * @returns 响应数据（不含 success/error 字段）
 */
export async function invokeIPC<T extends IPCResponse>(
  channel: string,
  ...args: unknown[]
): Promise<Omit<T, 'success' | 'error'>> {
  const result = await window.electronAPI.invoke(channel, ...args) as T;
  if (!result.success) {
    throw new Error(result.error || `IPC 调用失败: ${channel}`);
  }
  const { success: _, error: __, ...data } = result;
  return data as Omit<T, 'success' | 'error'>;
}

/**
 * 调用 IPC（无返回数据，仅检查成功/失败）
 */
export async function invokeIPCVoid(
  channel: string,
  ...args: unknown[]
): Promise<void> {
  const result = await window.electronAPI.invoke(channel, ...args) as IPCResponse;
  if (!result.success) {
    throw new Error(result.error || `IPC 调用失败: ${channel}`);
  }
}

/**
 * 调用 IPC（返回原始结果，不自动处理错误）
 * 用于需要自定义错误处理的场景
 */
export async function invokeIPCRaw<T>(
  channel: string,
  ...args: unknown[]
): Promise<T> {
  return await window.electronAPI.invoke(channel, ...args) as T;
}
