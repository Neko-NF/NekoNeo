import { computed } from 'vue';
import { useMetricsStore } from '@/stores/metrics';

export function useMetrics() {
  const store = useMetricsStore();

  return {
    metrics: computed(() => store.metrics),
    setup: store.setup,
  };
}
