<script setup lang="ts">
import { computed } from 'vue';
import { useConfigStore } from '@/stores/config';
import { useTheme } from '@/composables/useTheme';
import NIcon from '@/components/base/NIcon.vue';

const configStore = useConfigStore();
const { applyTheme } = useTheme();

const isDark = computed(() => {
  return document.documentElement.getAttribute('data-theme') !== 'light';
});

async function toggleTheme() {
  const next = configStore.config.themeMode === 'dark' ? 'light' : 'dark';
  await configStore.set('themeMode', next);
  applyTheme(configStore.config);
}
</script>

<template>
  <header class="top-bar" data-tauri-drag-region>
    <div class="top-bar__brand">
      <span class="top-bar__dot" />
      <span>NekoNeo</span>
    </div>
    <div class="top-bar__actions">
      <button
        class="top-bar__theme-btn"
        :aria-label="isDark ? 'Switch to light theme' : 'Switch to dark theme'"
        :title="isDark ? '浅色模式' : '深色模式'"
        @click="toggleTheme"
      >
        <NIcon :name="isDark ? 'sun' : 'moon'" :size="16" />
      </button>
    </div>
  </header>
</template>

<style scoped>
.top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 36px;
  padding: 0 var(--space-3);
  background: var(--bg-app);
  border-bottom: 1px solid var(--border-default);
  color: var(--text-secondary);
  font-size: var(--text-sm);
  -webkit-app-region: drag;
  user-select: none;
}

.top-bar__brand {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--text-primary);
  font-weight: var(--fw-semibold);
  font-size: var(--text-base);
}

.top-bar__dot {
  width: 7px;
  height: 7px;
  border-radius: var(--radius-full);
  background: var(--color-primary);
}

.top-bar__actions {
  display: flex;
  align-items: center;
  -webkit-app-region: no-drag;
}

.top-bar__theme-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 28px;
  height: 28px;
  padding: 0;
  border: none;
  border-radius: var(--radius-sm);
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-standard),
              color var(--duration-fast) var(--ease-standard);
}

.top-bar__theme-btn:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}
</style>
