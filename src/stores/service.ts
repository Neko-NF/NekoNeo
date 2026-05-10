import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { commands, getErrorMessage } from '@/api/commands';
import type { LogEntry, ServiceStatus, TickResult } from '@/types';

export const useServiceStore = defineStore('service', () => {
  const running = ref(false);
  const status = ref<ServiceStatus | null>(null);
  const lastResult = ref<TickResult | null>(null);
  const logs = ref<LogEntry[]>([]);
  const loading = ref(false);
  const error = ref<string | null>(null);
  const initialized = ref(false);
  let timer: ReturnType<typeof setInterval> | null = null;

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
    appendLog('info', '服务状态轮询已启动');
    timer = window.setInterval(() => {
      void refresh();
    }, 3000);
  }

  async function start() {
    loading.value = true;
    error.value = null;
    try {
      status.value = await commands.serviceStart();
      running.value = status.value.running;
      lastResult.value = await commands.serviceLastResult();
      appendLog('success', '上报服务已启动');
    } catch (err) {
      error.value = getErrorMessage(err);
      appendLog('error', `启动失败: ${error.value}`);
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
    } catch (err) {
      error.value = getErrorMessage(err);
      appendLog('error', `停止失败: ${error.value}`);
    } finally {
      loading.value = false;
    }
  }

  return { running, status, lastResult, logs, loading, error, statusText, setup, start, stop, refresh };
});
