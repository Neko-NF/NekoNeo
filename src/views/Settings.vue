<script setup lang="ts">
import { computed } from 'vue';
import NSwitch from '@/components/base/NSwitch.vue';
import { useConfigStore } from '@/stores/config';

const configStore = useConfigStore();

const rows = computed(() => [
  {
    key: 'enableAutoServiceStart' as const,
    label: '启动后自动上报',
    description: '应用启动时自动启动上报服务',
    value: configStore.config.enableAutoServiceStart,
  },
  {
    key: 'enableAutoRestart' as const,
    label: '自动恢复',
    description: '失败时按看门狗策略重启服务',
    value: configStore.config.enableAutoRestart,
  },
  {
    key: 'autoCheckUpdate' as const,
    label: '自动检查更新',
    description: '启动后检查稳定版或测试版更新',
    value: configStore.config.autoCheckUpdate,
  },
]);

async function update(key: keyof typeof configStore.config, value: boolean) {
  await configStore.set(key as never, value as never);
}
</script>

<template>
  <section class="page-card">
    <p class="page-card__eyebrow">应用设置</p>
    <h1>基础运行策略</h1>
    <div class="settings-list">
      <div v-for="row in rows" :key="row.key" class="settings-row">
        <div class="settings-row__info">
          <strong>{{ row.label }}</strong>
          <span>{{ row.description }}</span>
        </div>
        <NSwitch :model-value="row.value" @update:model-value="update(row.key, $event)" />
      </div>
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

.settings-list {
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.settings-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-4);
  border-bottom: 1px solid var(--border-default);
}

.settings-row:last-child {
  border-bottom: 0;
}

.settings-row__info {
  display: grid;
  gap: var(--space-1);
}

.settings-row__info span {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}
</style>
