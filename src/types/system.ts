export interface SystemMetrics {
  cpuPct: number;
  memPct: number;
  memUsed: number;
  memTotal: number;
  netDownBps: number;
  netUpBps: number;
  networkLatency: number;
  cpuModel: string;
  cpuCores: number;
  uptime: number;
  hostname: string;
  osFriendlyName: string;
}

export interface WindowBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface WindowInfo {
  title: string;
  processName: string;
  pid: number;
  path: string;
  bounds?: WindowBounds | null;
}

export interface DeviceProfile {
  hostname: string;
  osFriendlyName: string;
  cpuModel: string;
  cpuCores: number;
  appVersion: string;
}

export interface MediaInfo {
  title: string;
  artist: string;
  album: string;
  thumbnailPath?: string | null;
}

export interface ConnectivityStatus {
  reachable: boolean;
  url: string;
  detail: string;
  statusCode?: number | null;
}
