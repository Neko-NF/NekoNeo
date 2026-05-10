<script setup lang="ts">
import NBadge from '@/components/base/NBadge.vue';
import type { TickResult } from '@/types';

interface Props {
  running: boolean;
  result: TickResult | null;
}

defineProps<Props>();
</script>

<template>
  <section class="status-card">
    <div class="status-card__header">
      <div>
        <p class="status-card__eyebrow">服务状态</p>
        <h2 class="status-card__title">当前上报摘要</h2>
      </div>
      <NBadge :tone="running ? 'running' : 'danger'">
        {{ running ? '运行中' : '已停止' }}
      </NBadge>
    </div>
    <div class="status-card__grid">
      <div class="status-card__item">
        <span class="status-card__label">用户状态</span>
        <strong>{{ result?.userStatus === 'away' ? '离开' : '在线' }}</strong>
      </div>
      <div class="status-card__item">
        <span class="status-card__label">前台应用</span>
        <strong>{{ result?.appName ?? '未采集' }}</strong>
      </div>
      <div class="status-card__item">
        <span class="status-card__label">电量</span>
        <strong>{{ result?.hasBattery ? `${result?.batteryLevel}%` : '无电池' }}</strong>
      </div>
      <div class="status-card__item">
        <span class="status-card__label">截图上报</span>
        <strong>{{ result?.hasScreenshot ? '已启用' : '未启用' }}</strong>
      </div>
    </div>
  </section>
</template>

<style scoped>
.status-card {
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.status-card__header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
  margin-bottom: var(--space-5);
}

.status-card__eyebrow {
  margin: 0 0 var(--space-1);
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.status-card__title {
  margin: 0;
  font-size: var(--text-2xl);
}

.status-card__grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-4);
}

.status-card__item {
  display: grid;
  gap: var(--space-1);
  padding: var(--space-4);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
}

.status-card__label {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}
</style>
