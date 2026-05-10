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

export interface WindowInfo {
  title: string;
  processName: string;
  pid: number;
  path: string;
}
