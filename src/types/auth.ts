export interface AuthCredentials {
  username: string;
  password: string;
}

export interface AuthResponse {
  token: string;
  user: UserInfo;
}

export interface UserInfo {
  id: number;
  username: string;
  email?: string | null;
}

export interface DeviceKeyRequest {
  deviceName: string;
}

export interface DeviceKeyResponse {
  deviceKey: string;
  deviceId: number;
}
