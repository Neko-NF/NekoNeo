import { defineStore } from 'pinia';
import { ref } from 'vue';
import { commands } from '@/api/commands';
import type { SystemMetrics } from '@/types';

export const useMetricsStore = defineStore('metrics', () => {
  const metrics = ref<SystemMetrics | null>(null);
  const initialized = ref(false);
  let timer: ReturnType<typeof setInterval> | null = null;

  async function refresh() {
    metrics.value = await commands.systemGetMetrics();
  }

  async function setup() {
    if (initialized.value) {
      return;
    }

    initialized.value = true;
    await refresh();
    timer = window.setInterval(() => {
      void refresh();
    }, 5000);
  }

  return { metrics, setup, refresh };
});
