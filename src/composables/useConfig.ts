import { computed } from 'vue';
import { useConfigStore } from '@/stores/config';

export function useConfig() {
  const store = useConfigStore();

  return {
    config: computed(() => store.config),
    loading: computed(() => store.loading),
    error: computed(() => store.error),
    loadAll: store.loadAll,
    set: store.set,
  };
}
