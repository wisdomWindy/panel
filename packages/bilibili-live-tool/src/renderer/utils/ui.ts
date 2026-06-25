/**
 * UI 工具函数
 * 状态标签映射、确认对话框封装等
 */

import { ElMessageBox, ElMessage } from 'element-plus';

/** 录制状态 → 标签类型 */
export function getRecordingStatusTagType(status: string): string {
  switch (status) {
    case 'recording': return 'danger';
    case 'paused': return 'warning';
    case 'completed': return 'success';
    case 'failed': return 'info';
    default: return '';
  }
}

/** 录制状态 → 中文文本 */
export function getRecordingStatusText(status: string): string {
  switch (status) {
    case 'recording': return '录制中';
    case 'paused': return '已暂停';
    case 'completed': return '已完成';
    case 'failed': return '失败';
    default: return status;
  }
}

/** 高光原因 → 标签类型 */
export function getHighlightReasonTagType(reason: string): string {
  switch (reason) {
    case 'danmaku_surge': return 'danger';
    case 'high_value_gift': return 'warning';
    case 'captain': return 'success';
    case 'manual': return 'info';
    default: return '';
  }
}

/** 高光原因 → 中文文本 */
export function getHighlightReasonText(reason: string): string {
  switch (reason) {
    case 'danmaku_surge': return '弹幕激增';
    case 'high_value_gift': return '高价值礼物';
    case 'captain': return '舰长赠送';
    case 'manual': return '手动标记';
    default: return reason;
  }
}

/**
 * 确认删除对话框
 * @returns true 如果用户确认，false 如果用户取消
 */
export async function confirmDelete(message: string): Promise<boolean> {
  try {
    await ElMessageBox.confirm(message, '确认删除', {
      confirmButtonText: '删除',
      cancelButtonText: '取消',
      type: 'warning'
    });
    return true;
  } catch {
    return false;
  }
}

/**
 * 显示成功消息
 */
export function showSuccess(message: string): void {
  ElMessage.success(message);
}

/**
 * 显示错误消息
 */
export function showError(message: string): void {
  ElMessage.error(message);
}

/**
 * 显示警告消息
 */
export function showWarning(message: string): void {
  ElMessage.warning(message);
}
