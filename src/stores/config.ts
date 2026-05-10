import { defineStore } from 'pinia';
import { ref } from 'vue';
import { commands, getErrorMessage } from '@/api/commands';
import type { AppConfig } from '@/types';

export const useConfigStore = defineStore('config', () => {
  const config = ref<AppConfig>({
    deviceKey: '',
    deviceId: null,
    reportInterval: 30,
    serverMode: 'production',
    serverUrlProd: 'https://api.example.com',
    serverUrlLocal: 'http://127.0.0.1:3000',
    enableScreenshot: true,
    screenshotInterval: 60,
    syncScreenshotInterval: true,
    enableAutoStart: false,
    minimizeOnAutoStart: true,
    startupDelayMs: 0,
    enableAutoServiceStart: false,
    closeAction: 'minimize',
    themeMode: 'dark',
    seedColor: '#06b6d4',
    uiScale: 100,
    uiFont: 'Segoe UI',
    enableNotification: true,
    doNotDisturb: false,
    enableIncognito: false,
    incognitoScope: 'both',
    blurAllScreenshots: false,
    privacyRules: [],
    enableAutoRestart: true,
    maxRestarts: 3,
    restartIntervalSec: 30,
    watchdogTimeoutSec: 120,
    autoCheckUpdate: true,
    updateChannel: 'stable',
    autoDownload: false,
    skippedVersion: '',
  });
  const loading = ref(false);
  const error = ref<string | null>(null);

  async function loadAll() {
    loading.value = true;
    error.value = null;
    try {
      config.value = await commands.configGetAll();
    } catch (err) {
      error.value = getErrorMessage(err);
    } finally {
      loading.value = false;
    }
  }

  async function set<K extends keyof AppConfig>(key: K, value: AppConfig[K]) {
    await commands.configSet(key, value);
    config.value[key] = value;
  }

  return { config, loading, error, loadAll, set };
});
