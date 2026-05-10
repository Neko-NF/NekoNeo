<script setup lang="ts">
import { computed } from 'vue';
import NButton from '@/components/base/NButton.vue';
import MetricRow from '@/components/widgets/MetricRow.vue';
import QuickToggle from '@/components/widgets/QuickToggle.vue';
import StatusCard from '@/components/widgets/StatusCard.vue';
import { useConfigStore } from '@/stores/config';
import { useMetricsStore } from '@/stores/metrics';
import { useServiceStore } from '@/stores/service';

const configStore = useConfigStore();
const serviceStore = useServiceStore();
const metricsStore = useMetricsStore();

const toggles = computed(() => [
  {
    key: 'enableScreenshot' as const,
    label: '截图上报',
    description: '控制周期性截图采集',
    value: configStore.config.enableScreenshot,
  },
  {
    key: 'enableIncognito' as const,
    label: '隐身模式',
    description: '优先保护敏感内容',
    value: configStore.config.enableIncognito,
  },
  {
    key: 'enableNotification' as const,
    label: '系统通知',
    description: '显示服务与更新提醒',
    value: configStore.config.enableNotification,
  },
  {
    key: 'doNotDisturb' as const,
    label: '勿扰模式',
    description: '降低提示干扰',
    value: configStore.config.doNotDisturb,
  },
]);

async function handleToggle(key: keyof typeof configStore.config, value: boolean) {
  await configStore.set(key as never, value as never);
}
</script>

<template>
  <div class="dashboard">
    <StatusCard :running="serviceStore.running" :result="serviceStore.lastResult" />

    <section class="dashboard__panel">
      <div class="dashboard__section-head">
        <div>
          <p class="dashboard__eyebrow">快捷控制</p>
          <h2>核心开关</h2>
        </div>
        <div class="dashboard__actions">
          <NButton variant="primary" :disabled="serviceStore.running || serviceStore.loading" @click="serviceStore.start()">
            启动服务
          </NButton>
          <NButton variant="secondary" :disabled="!serviceStore.running || serviceStore.loading" @click="serviceStore.stop()">
            停止服务
          </NButton>
        </div>
      </div>
      <div class="dashboard__toggles">
        <QuickToggle
          v-for="toggle in toggles"
          :key="toggle.key"
          :label="toggle.label"
          :description="toggle.description"
          :model-value="toggle.value"
          @update:model-value="handleToggle(toggle.key, $event)"
        />
      </div>
    </section>

    <section class="dashboard__panel">
      <p class="dashboard__eyebrow">系统摘要</p>
      <h2>实时指标</h2>
      <div class="dashboard__metrics">
        <MetricRow label="CPU" :value="`${metricsStore.metrics?.cpuPct ?? 0}%`" :percent="metricsStore.metrics?.cpuPct ?? 0" />
        <MetricRow label="内存" :value="`${metricsStore.metrics?.memPct ?? 0}%`" :percent="metricsStore.metrics?.memPct ?? 0" />
        <MetricRow label="下行" :value="`${metricsStore.metrics?.netDownBps ?? 0} B/s`" />
        <MetricRow label="延迟" :value="`${metricsStore.metrics?.networkLatency ?? -1} ms`" />
      </div>
    </section>
  </div>
</template>

<style scoped>
.dashboard {
  display: grid;
  gap: var(--space-6);
}

.dashboard__panel {
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.dashboard__section-head {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
  margin-bottom: var(--space-5);
}

.dashboard__eyebrow {
  margin: 0 0 var(--space-1);
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.dashboard__section-head h2,
.dashboard__panel h2 {
  margin: 0;
  font-size: var(--text-xl);
}

.dashboard__actions {
  display: flex;
  gap: var(--space-2);
}

.dashboard__toggles {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-4);
}

.dashboard__metrics {
  display: grid;
  gap: var(--space-2);
}
</style>
