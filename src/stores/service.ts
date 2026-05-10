import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import router from '@/router';
import { useConfigStore } from '@/stores/config';
import { commands, getErrorCode, getErrorMessage } from '@/api/commands';
import { useToastStore } from '@/stores/toast';
import type { KeyStatusEvent, LogEntry, ServiceStatus, TickResult } from '@/types';

const FOCUS_SETTINGS_ERROR_CODES = [
  'MissingDeviceKey',
  'MissingServerConfig',
  'TakeoverRequired',
  'InvalidKey',
  'KeyRevoked',
  'DeviceNotFound',
];

export const useServiceStore = defineStore('service', () => {
  const configStore = useConfigStore();
  const running = ref(false);
  const status = ref<ServiceStatus | null>(null);
  const lastResult = ref<TickResult | null>(null);
  const logs = ref<LogEntry[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const keyStatus = ref<KeyStatusEvent | null>(null);
  const initialized = ref(false);
  let timer: ReturnType<typeof setInterval> | null = null;
  let serviceTickUnlisten: (() => void) | null = null;
  let serviceStatusUnlisten: (() => void) | null = null;
  let serviceLogUnlisten: (() => void) | null = null;
  let serviceKeyStatusUnlisten: (() => void) | null = null;

  const statusText = computed(() => (running.value ? '运行中' : '已停止'));

  function appendLog(level: LogEntry['level'], message: string) {
    logs.value.unshift({
      level,
      message,
      time: new Date().toLocaleTimeString('zh-CN', { hour12: false }),
    });

    if (logs.value.length > 200) {
      logs.value.length = 200;
    }
  }

  function openSettingsForDeviceKey() {
    const current = router.currentRoute.value;
    if (current.name === 'settings') {
      return;
    }
    void router.push({ name: 'settings', query: { focus: 'deviceKey' } });
  }

  async function refresh() {
    status.value = await commands.serviceStatus();
    lastResult.value = await commands.serviceLastResult();
    running.value = status.value.running;
  }

  async function setup() {
    if (initialized.value) {
      return;
    }

    initialized.value = true;
    await refresh();
    serviceTickUnlisten = await listen<TickResult>('service:tick', (event) => {
      lastResult.value = event.payload;
    });
    serviceStatusUnlisten = await listen<ServiceStatus>('service:status', (event) => {
      status.value = event.payload;
      running.value = event.payload.running;
    });
    serviceLogUnlisten = await listen<LogEntry>('service:log', (event) => {
      logs.value.unshift(event.payload);
      if (logs.value.length > 200) {
        logs.value.length = 200;
      }
    });
    serviceKeyStatusUnlisten = await listen<KeyStatusEvent>('service:key_status', (event) => {
      keyStatus.value = event.payload;
      error.value = event.payload.message;
      appendLog('warn', `${event.payload.code}: ${event.payload.message}`);

      if (FOCUS_SETTINGS_ERROR_CODES.includes(event.payload.code)) {
        void configStore.loadAll();
        openSettingsForDeviceKey();
      }
    });

    appendLog('info', '服务状态监听已启动');
    timer = window.setInterval(() => {
      void refresh();
    }, 3000);
  }

  async function start() {
    loading.value = true;
    error.value = null;
    keyStatus.value = null;

    try {
      status.value = await commands.serviceStart();
      running.value = status.value.running;
      lastResult.value = await commands.serviceLastResult();
      appendLog('success', '上报服务已启动');
      useToastStore().success('服务已启动');
    } catch (err) {
      const code = getErrorCode(err);
      error.value = getErrorMessage(err);
      appendLog('error', `启动失败: ${error.value}`);
      useToastStore().error(error.value!);

      if (FOCUS_SETTINGS_ERROR_CODES.includes(code)) {
        keyStatus.value = { code, message: error.value };
        openSettingsForDeviceKey();
      }
    } finally {
      loading.value = false;
    }
  }

  async function stop() {
    loading.value = true;
    error.value = null;
    try {
      status.value = await commands.serviceStop();
      running.value = status.value.running;
      appendLog('warn', '上报服务已停止');
      useToastStore().info('服务已停止');
    } catch (err) {
      error.value = getErrorMessage(err);
      appendLog('error', `停止失败: ${error.value}`);
      useToastStore().error(error.value!);
    } finally {
      loading.value = false;
    }
  }

  return {
    running,
    status,
    lastResult,
    logs,
    loading,
    error,
    keyStatus,
    statusText,
    setup,
    start,
    stop,
    refresh,
  };
});
