import { defineStore } from 'pinia';
import { ref } from 'vue';
import { commands, getErrorMessage } from '@/api/commands';
import type { AuthCredentials, AuthResponse, DeviceKeyResponse, UserInfo } from '@/types';

export const useAuthStore = defineStore('auth', () => {
  const token = ref('');
  const user = ref<UserInfo | null>(null);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const mode = ref<'login' | 'register'>('login');

  const isLoggedIn = () => !!token.value && !!user.value;

  async function doAuth(credentials: AuthCredentials) {
    loading.value = true;
    error.value = null;
    try {
      const result: AuthResponse =
        mode.value === 'login'
          ? await commands.authLogin(credentials)
          : await commands.authRegister(credentials);
      token.value = result.token;
      user.value = result.user;
      return result;
    } catch (err) {
      error.value = getErrorMessage(err);
      throw err;
    } finally {
      loading.value = false;
    }
  }

  async function generateDeviceKey(deviceName: string) {
    if (!token.value) throw new Error('Not logged in');
    const result: DeviceKeyResponse = await commands.authGenerateDeviceKey(token.value, {
      deviceName,
    });
    return result;
  }

  function logout() {
    token.value = '';
    user.value = null;
    error.value = null;
  }

  return { token, user, loading, error, mode, isLoggedIn, doAuth, generateDeviceKey, logout };
});
