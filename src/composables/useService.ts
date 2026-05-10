import { computed } from 'vue';
import { useServiceStore } from '@/stores/service';

export function useService() {
  const store = useServiceStore();

  return {
    running: computed(() => store.running),
    status: computed(() => store.status),
    lastResult: computed(() => store.lastResult),
    logs: computed(() => store.logs),
    loading: computed(() => store.loading),
    error: computed(() => store.error),
    statusText: computed(() => store.statusText),
    start: store.start,
    stop: store.stop,
  };
}
