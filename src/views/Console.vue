<script setup lang="ts">
import LogEntry from '@/components/widgets/LogEntry.vue';
import { useServiceStore } from '@/stores/service';

const serviceStore = useServiceStore();
</script>

<template>
  <section class="page-card">
    <p class="page-card__eyebrow">运行日志</p>
    <h1>服务事件流</h1>
    <div class="console-list">
      <LogEntry v-for="entry in serviceStore.logs" :key="`${entry.time}-${entry.message}`" :entry="entry" />
      <p v-if="serviceStore.logs.length === 0" class="console-empty">暂无日志，后端事件监听已预留。</p>
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

.console-list {
  display: grid;
  gap: var(--space-2);
}

.console-empty {
  color: var(--text-secondary);
}
</style>
