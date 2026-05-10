<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import { listen } from '@tauri-apps/api/event';
import { commands } from '@/api/commands';
import { useDesktopNotifications } from '@/composables/useDesktopNotifications';
import AppShell from '@/components/layout/AppShell.vue';
import ToastContainer from '@/components/widgets/ToastContainer.vue';
import { useConfigStore } from '@/stores/config';
import { useServiceStore } from '@/stores/service';
import { useMetricsStore } from '@/stores/metrics';
import { useToastStore } from '@/stores/toast';
import { useTheme } from '@/composables/useTheme';
import type { KeyStatusEvent, UpdateInfo } from '@/types';

const configStore = useConfigStore();
const serviceStore = useServiceStore();
const metricsStore = useMetricsStore();
const toastStore = useToastStore();
const { applyTheme, watchSystemTheme } = useTheme();
const { notify } = useDesktopNotifications();
const unlisteners: Array<() => void> = [];

onMounted(async () => {
  await configStore.loadAll();
  applyTheme(configStore.config);
  watchSystemTheme();
  await serviceStore.setup();
  await metricsStore.setup();
  if (configStore.config.enableAutoServiceStart) {
    await serviceStore.start();
  }

  unlisteners.push(
    await listen('app:close-requested', async () => {
      const minimize = window.confirm('Close action is set to ask.\n\nPress OK to minimize to tray, or Cancel to exit.');
      await commands.appResolveCloseRequest(minimize ? 'minimize' : 'exit');
    }),
  );

  unlisteners.push(
    await listen<KeyStatusEvent>('service:key_status', async (event) => {
      toastStore.error(event.payload.message);
      await notify('error', 'Service attention needed', event.payload.message);
    }),
  );

  unlisteners.push(
    await listen<{ level: 'info' | 'success' | 'warn' | 'error'; title: string; body: string }>(
      'notify',
      async (event) => {
        toastStore.push(event.payload.title, event.payload.level, 4000);
        await notify(event.payload.level, event.payload.title, event.payload.body);
      },
    ),
  );

  unlisteners.push(
    await listen<UpdateInfo>('update:available', (event) => {
      const info = event.payload;
      if (info.mandatory) {
        toastStore.warn(`强制更新: v${info.version}`, 10000);
      } else if (info.downloaded) {
        toastStore.info(`新版本 v${info.version} 已下载，前往设置安装`, 6000);
      } else {
        toastStore.info(`有新版本 v${info.version} 可用`, 5000);
      }
    }),
  );

  unlisteners.push(
    await listen<{ message: string }>('update:error', (event) => {
      toastStore.warn(`更新: ${event.payload.message}`, 4000);
    }),
  );
});

onUnmounted(() => {
  unlisteners.splice(0).forEach((unlisten) => unlisten());
});
</script>

<template>
  <template v-if="$route.name === 'picker'">
    <RouterView />
  </template>
  <template v-else>
    <AppShell>
      <RouterView v-slot="{ Component }">
        <Transition name="page" mode="out-in">
          <component :is="Component" :key="$route.fullPath" />
        </Transition>
      </RouterView>
    </AppShell>
  </template>
  <ToastContainer />
</template>
