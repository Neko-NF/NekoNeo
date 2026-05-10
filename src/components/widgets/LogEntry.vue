<script setup lang="ts">
import type { LogEntry } from '@/types';

defineProps<{
  entry: LogEntry;
}>();
</script>

<template>
  <div class="log-entry" :class="`log-entry--${entry.level}`">
    <span class="log-entry__time">{{ entry.time }}</span>
    <span>{{ entry.message }}</span>
  </div>
</template>

<style scoped>
.log-entry {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  line-height: 1.6;
}

.log-entry__time {
  margin-right: var(--space-2);
  color: var(--text-tertiary);
}

.log-entry--info {
  color: var(--text-secondary);
}

.log-entry--success {
  color: var(--color-success);
}

.log-entry--warn {
  color: var(--color-warning);
}

.log-entry--error {
  color: var(--color-danger);
}
</style>
