import { invoke } from '@tauri-apps/api/core';
import type {
  AppConfig,
  AppError,
  AuthCredentials,
  AuthResponse,
  ConnectivityStatus,
  DeviceKeyRequest,
  DeviceKeyResponse,
  DeviceProfile,
  LatestScreenshot,
  MediaInfo,
  PendingCloseAction,
  ServiceStatus,
  SystemMetrics,
  TickResult,
  UpdateInfo,
  UserInfo,
  ValidateKeyResponse,
  WindowInfo,
} from '@/types';

export const commands = {
  configGetAll: () => invoke<AppConfig>('config_get_all'),
  configSet: <K extends keyof AppConfig>(key: K, value: AppConfig[K]) =>
    invoke<void>('config_set', { key, value }),
  configValidateDeviceKey: (key: string, serverUrl?: string) =>
    invoke<ValidateKeyResponse>('config_validate_device_key', { key, serverUrl }),
  configTestConnectivity: (serverUrl?: string, key?: string) =>
    invoke<ConnectivityStatus>('config_test_connectivity', { serverUrl, key }),
  configSyncDeviceMetadata: () => invoke<DeviceProfile>('config_sync_device_metadata'),
  screenshotCaptureNow: () => invoke<string>('screenshot_capture_now'),
  screenshotGetLatest: () => invoke<LatestScreenshot | null>('screenshot_get_latest'),
  serviceStart: () => invoke<ServiceStatus>('service_start'),
  serviceStop: () => invoke<ServiceStatus>('service_stop'),
  serviceStatus: () => invoke<ServiceStatus>('service_status'),
  serviceLastResult: () => invoke<TickResult | null>('service_last_result'),
  systemGetMetrics: () => invoke<SystemMetrics>('system_get_metrics'),
  systemHealthCheck: () => invoke<Record<string, string>>('system_health_check'),
  systemGetDeviceProfile: () => invoke<DeviceProfile>('system_get_device_profile'),
  systemGetFonts: () => invoke<string[]>('system_get_fonts'),
  systemGetMediaInfo: () => invoke<MediaInfo | null>('system_get_media_info'),
  systemGetDeviceFingerprint: () => invoke<string>('system_get_device_fingerprint'),
  systemGetProcessIcon: (exePath: string) => invoke<number[] | null>('system_get_process_icon', { exePath }),
  privacyGetWindows: () => invoke<WindowInfo[]>('privacy_get_windows'),
  privacyOpenPicker: () => invoke<void>('privacy_open_picker'),
  privacyGetCursorPos: () => invoke<[number, number]>('privacy_get_cursor_pos'),
  privacyClosePicker: (window: WindowInfo | null) =>
    invoke<void>('privacy_close_picker', { windowJson: window ? JSON.stringify(window) : null }),
  updateCheck: (channel: string) => invoke<UpdateInfo | null>('update_check', { channel }),
  updateDownload: (channel: string) => invoke<string>('update_download', { channel }),
  updateInstall: () => invoke<void>('update_install'),
  appResolveCloseRequest: (action: PendingCloseAction) =>
    invoke<void>('app_resolve_close_request', { action }),
  authRegister: (credentials: AuthCredentials) =>
    invoke<AuthResponse>('auth_register', { credentials }),
  authLogin: (credentials: AuthCredentials) =>
    invoke<AuthResponse>('auth_login', { credentials }),
  authGetMe: (token: string) => invoke<UserInfo>('auth_get_me', { token }),
  authGenerateDeviceKey: (token: string, request: DeviceKeyRequest) =>
    invoke<DeviceKeyResponse>('auth_generate_device_key', { token, request }),
};

export function getErrorCode(error: unknown): string {
  if (typeof error === 'object' && error !== null && 'code' in error) {
    return String((error as AppError).code);
  }

  return 'UnknownError';
}

export function getErrorMessage(error: unknown): string {
  if (typeof error === 'object' && error !== null && 'message' in error) {
    return String((error as AppError).message);
  }

  return 'Unknown error';
}
