export interface UpdateInfo {
  version: string;
  releaseNotes: string;
  mandatory: boolean;
  channel: string;
  assetName?: string | null;
  publishedAt?: string | null;
  downloaded: boolean;
}

export interface UpdateProgress {
  downloaded: number;
  total?: number | null;
  percent: number;
  assetName: string;
}
