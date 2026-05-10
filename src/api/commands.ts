import { invoke } from '@tauri-apps/api/core';
import type { AppConfig, AppError, ServiceStatus, SystemMetrics, TickResult, UpdateInfo, WindowInfo } from '@/types';

export const commands = {
  configGetAll: () => invoke<AppConfig>('config_get_all'),
  configSet: <K extends keyof AppConfig>(key: K, value: AppConfig[K]) => invoke<void>('config_set', { key, value }),
  serviceStart: () => invoke<ServiceStatus>('service_start'),
  serviceStop: () => invoke<ServiceStatus>('service_stop'),
  serviceStatus: () => invoke<ServiceStatus>('service_status'),
  serviceLastResult: () => invoke<TickResult | null>('service_last_result'),
  systemGetMetrics: () => invoke<SystemMetrics>('system_get_metrics'),
  systemHealthCheck: () => invoke<Record<string, string>>('system_health_check'),
  systemGetFonts: () => invoke<string[]>('system_get_fonts'),
  privacyGetWindows: () => invoke<WindowInfo[]>('privacy_get_windows'),
  updateCheck: (channel: string) => invoke<UpdateInfo | null>('update_check', { channel }),
};

export function getErrorMessage(error: unknown): string {
  if (typeof error === 'object' && error !== null && 'message' in error) {
    return String((error as AppError).message);
  }

  return '未知错误';
}
