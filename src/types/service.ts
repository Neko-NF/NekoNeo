export interface TickResult {
  success: boolean;
  timestamp: string;
  appName: string;
  batteryLevel: number;
  isCharging: boolean;
  hasBattery: boolean;
  userStatus: 'online' | 'away';
  hasScreenshot: boolean;
  screenshotBlurred: boolean;
  error?: string;
}

export interface ServiceStatus {
  running: boolean;
  uptimeSec: number;
  consecutiveFailures: number;
  autoRestartCount: number;
}

export interface LogEntry {
  level: 'info' | 'success' | 'warn' | 'error';
  message: string;
  time: string;
}
