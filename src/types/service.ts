export interface TickResult {
  success: boolean;
  timestamp: string;
  appName: string;
  batteryLevel: number;
  isCharging: boolean;
  hasBattery: boolean;
  userStatus: 'online' | 'away';
  idleMs: number;
  hasScreenshot: boolean;
  screenshotBlurred: boolean;
  screenshotPath?: string | null;
  error?: string;
}

export interface ServiceStatus {
  running: boolean;
  uptimeSec: number;
  consecutiveFailures: number;
  autoRestartCount: number;
}

export interface KeyStatusEvent {
  code: string;
  message: string;
}

export interface ValidateKeyResponse {
  valid: boolean;
  deviceId: number | null;
  warning?: string;
  message?: string;
}

export interface LatestScreenshot {
  path: string;
  blurred: boolean;
  capturedAt: string;
  dataUrl?: string | null;
}

export interface LogEntry {
  level: 'info' | 'success' | 'warn' | 'error';
  message: string;
  time: string;
}
