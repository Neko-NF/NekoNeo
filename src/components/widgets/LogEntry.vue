<script setup lang="ts">
import type { LogEntry } from '@/types';

defineProps<{
  entry: LogEntry;
}>();
</script>

<template>
  <div class="log-entry" :class="`log-entry--${entry.level}`">
    <span class="log-entry__time">{{ entry.time }}</span>
    <span class="log-entry__msg">{{ entry.message }}</span>
  </div>
</template>

<style scoped>
.log-entry {
  display: flex;
  align-items: baseline;
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  line-height: 1.6;
  padding: 2px 0;
}

.log-entry__time {
  flex-shrink: 0;
  margin-right: var(--space-3);
  color: var(--text-tertiary);
  font-size: var(--text-xs);
  user-select: none;
}

.log-entry__msg {
  word-break: break-all;
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
