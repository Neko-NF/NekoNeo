<script setup lang="ts">
interface Props {
  label: string;
  value: string;
  percent?: number;
}

defineProps<Props>();
</script>

<template>
  <div class="metric-row">
    <span class="metric-row__label">{{ label }}</span>
    <span class="metric-row__value">{{ value }}</span>
    <div v-if="typeof percent === 'number'" class="metric-row__bar">
      <div
        class="metric-row__bar-fill"
        :class="{
          'metric-row__bar-fill--warn': percent >= 70 && percent < 90,
          'metric-row__bar-fill--danger': percent >= 90,
        }"
        :style="{ width: `${Math.max(0, Math.min(100, percent))}%` }"
      />
    </div>
  </div>
</template>

<style scoped>
.metric-row {
  display: grid;
  grid-template-columns: 1fr auto;
  align-items: center;
  gap: var(--space-2);
  padding: var(--space-2) 0;
}

.metric-row__label {
  color: var(--text-secondary);
  font-size: var(--text-sm);
  font-weight: var(--fw-medium);
}

.metric-row__value {
  color: var(--text-primary);
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  font-weight: var(--fw-semibold);
}

.metric-row__bar {
  grid-column: 1 / -1;
  height: 3px;
  overflow: hidden;
  border-radius: var(--radius-full);
  background: var(--border-default);
}

.metric-row__bar-fill {
  height: 100%;
  border-radius: var(--radius-full);
  background: var(--color-primary);
  transition: width var(--duration-slow) var(--ease-standard);
}

.metric-row__bar-fill--warn {
  background: var(--color-warning);
}

.metric-row__bar-fill--danger {
  background: var(--color-danger);
}
</style>
