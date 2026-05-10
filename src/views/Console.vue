<script setup lang="ts">
import { computed, nextTick, ref, watch } from 'vue';
import NButton from '@/components/base/NButton.vue';
import NBadge from '@/components/base/NBadge.vue';
import LogEntry from '@/components/widgets/LogEntry.vue';
import { useServiceStore } from '@/stores/service';
import type { LogEntry as LogEntryType } from '@/types';

const LEVELS: Array<LogEntryType['level'] | 'all'> = ['all', 'info', 'success', 'warn', 'error'];

const serviceStore = useServiceStore();
const activeLevel = ref<LogEntryType['level'] | 'all'>('all');
const autoScroll = ref(true);
const listEl = ref<HTMLElement | null>(null);

const filteredLogs = computed(() => {
  const all = serviceStore.logs;
  if (activeLevel.value === 'all') return all;
  return all.filter(e => e.level === activeLevel.value);
});

watch(filteredLogs, async () => {
  if (autoScroll.value) {
    await nextTick();
    if (listEl.value) {
      listEl.value.scrollTop = listEl.value.scrollHeight;
    }
  }
});

function clearLogs() {
  serviceStore.logs.splice(0);
}

const levelBadge = (level: LogEntryType['level']) => {
  const map: Record<string, string> = { info: 'neutral', success: 'success', warn: 'warning', error: 'danger' };
  return map[level] || 'neutral';
};
</script>

<template>
  <div class="console">
    <!-- Toolbar -->
    <div class="console-bar">
      <div class="console-bar__filters">
        <button
          v-for="lvl in LEVELS"
          :key="lvl"
          class="console-bar__lvl"
          :class="{ 'console-bar__lvl--active': activeLevel === lvl }"
          @click="activeLevel = lvl"
        >
          {{ lvl === 'all' ? '全部' : lvl.toUpperCase() }}
        </button>
      </div>

      <span class="console-bar__count">
        {{ filteredLogs.length }} / {{ serviceStore.logs.length }} 条
      </span>

      <div class="console-bar__actions">
        <button
          class="console-bar__btn"
          :class="{ 'console-bar__btn--on': autoScroll }"
          :title="autoScroll ? '自动滚动已开启' : '自动滚动已关闭'"
          @click="autoScroll = !autoScroll"
        >
          自动滚动
        </button>
        <NButton variant="ghost" size="sm" :disabled="serviceStore.logs.length === 0" @click="clearLogs()">
          清空
        </NButton>
      </div>
    </div>

    <!-- Log list -->
    <div ref="listEl" class="console-list">
      <LogEntry v-for="(entry, i) in filteredLogs" :key="`${entry.time}-${i}`" :entry="entry" />

      <div v-if="filteredLogs.length === 0 && serviceStore.logs.length === 0" class="console-empty">
        <span class="console-empty__icon">▤</span>
        <strong>暂无日志</strong>
        <span>启动上报服务后，后端事件流将在此实时显示。</span>
      </div>
      <div v-else-if="filteredLogs.length === 0" class="console-empty">
        <span>当前筛选条件下没有匹配的日志。</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.console {
  display: flex;
  flex-direction: column;
  gap: var(--space-4);
  height: 100%;
}

/* ── Toolbar ──────────────────────────────────────────────────── */

.console-bar {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-3) var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
  margin-bottom: var(--space-4);
  flex-wrap: wrap;
}

.console-bar__filters {
  display: flex;
  gap: var(--space-1);
  padding: var(--space-1);
  border-radius: var(--radius-sm);
  background: var(--bg-elevated);
}

.console-bar__lvl {
  padding: 2px var(--space-3);
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-secondary);
  font-size: var(--text-xs);
  font-weight: var(--fw-semibold);
  font-family: var(--font-mono);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-standard),
              color var(--duration-fast) var(--ease-standard);
}

.console-bar__lvl:hover {
  color: var(--text-primary);
}

.console-bar__lvl--active {
  background: var(--bg-surface);
  color: var(--color-primary);
}

.console-bar__count {
  color: var(--text-tertiary);
  font-size: var(--text-xs);
  font-family: var(--font-mono);
  white-space: nowrap;
}

.console-bar__actions {
  margin-left: auto;
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.console-bar__btn {
  padding: 2px var(--space-3);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-secondary);
  font-size: var(--text-xs);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-standard),
              border-color var(--duration-fast) var(--ease-standard);
}

.console-bar__btn:hover {
  background: var(--bg-hover);
}

.console-bar__btn--on {
  border-color: var(--color-primary-muted);
  background: var(--color-primary-muted);
  color: var(--color-primary);
}

/* ── List ─────────────────────────────────────────────────────── */

.console-list {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.console-empty {
  display: grid;
  gap: var(--space-2);
  justify-items: center;
  padding: var(--space-12) var(--space-4);
  color: var(--text-tertiary);
  text-align: center;
}

.console-empty__icon {
  font-size: 28px;
  opacity: 0.3;
  margin-bottom: var(--space-2);
}

.console-empty strong {
  color: var(--text-secondary);
}

.console-empty span {
  font-size: var(--text-sm);
}
</style>
