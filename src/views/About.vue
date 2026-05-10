<script setup lang="ts">
import { computed, ref } from 'vue';
import NButton from '@/components/base/NButton.vue';
import { commands } from '@/api/commands';
import { useConfigStore } from '@/stores/config';
import { useMetricsStore } from '@/stores/metrics';
import type { UpdateInfo } from '@/types';

const configStore = useConfigStore();
const metricsStore = useMetricsStore();
const updateInfo = ref<UpdateInfo | null>(null);

const serverUrl = computed(() =>
  configStore.config.serverMode === 'production'
    ? configStore.config.serverUrlProd
    : configStore.config.serverUrlLocal,
);

async function checkUpdate() {
  const result = await commands.updateCheck(configStore.config.updateChannel);
  if (result) {
    updateInfo.value = result;
  }
}
</script>

<template>
  <section class="page-card">
    <p class="page-card__eyebrow">关于应用</p>
    <h1>NekoNeo Alpha</h1>
    <div class="about-grid">
      <div class="about-item">
        <span>当前通道</span>
        <strong>{{ configStore.config.updateChannel }}</strong>
      </div>
      <div class="about-item">
        <span>服务器</span>
        <strong>{{ serverUrl }}</strong>
      </div>
      <div class="about-item">
        <span>系统</span>
        <strong>{{ metricsStore.metrics?.osFriendlyName ?? 'Unknown OS' }}</strong>
      </div>
      <div class="about-item">
        <span>设备</span>
        <strong>{{ metricsStore.metrics?.hostname ?? 'Unknown Host' }}</strong>
      </div>
    </div>
    <div class="about-update">
      <NButton variant="secondary" @click="checkUpdate()">检查更新</NButton>
      <p v-if="updateInfo" class="about-update__text">
        发现版本 {{ updateInfo.version }} · {{ updateInfo.releaseNotes }}
      </p>
      <p v-else class="about-update__text">当前为占位更新通道，可继续接入真实 updater。</p>
    </div>
  </section>
</template>

<style scoped>
.page-card {
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.page-card__eyebrow {
  margin: 0 0 var(--space-1);
  color: var(--text-secondary);
}

.page-card h1 {
  margin: 0 0 var(--space-5);
}

.about-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-4);
}

.about-item {
  display: grid;
  gap: var(--space-1);
  padding: var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
}

.about-item span {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.about-update {
  margin-top: var(--space-5);
  display: grid;
  gap: var(--space-2);
}

.about-update__text {
  margin: 0;
  color: var(--text-secondary);
  font-size: var(--text-sm);
}
</style>
