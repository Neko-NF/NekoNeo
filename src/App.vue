<script setup lang="ts">
import { onMounted } from 'vue';
import AppShell from '@/components/layout/AppShell.vue';
import { useConfigStore } from '@/stores/config';
import { useServiceStore } from '@/stores/service';
import { useMetricsStore } from '@/stores/metrics';
import { useTheme } from '@/composables/useTheme';

const configStore = useConfigStore();
const serviceStore = useServiceStore();
const metricsStore = useMetricsStore();
const { applyTheme, watchSystemTheme } = useTheme();

onMounted(async () => {
  await configStore.loadAll();
  applyTheme(configStore.config);
  watchSystemTheme();
  await serviceStore.setup();
  await metricsStore.setup();
});
</script>

<template>
  <AppShell>
    <RouterView v-slot="{ Component }">
      <Transition name="page" mode="out-in">
        <component :is="Component" :key="$route.fullPath" />
      </Transition>
    </RouterView>
  </AppShell>
</template>
