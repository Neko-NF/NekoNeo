<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { commands } from '@/api/commands';
import type { WindowInfo } from '@/types';

const windows = ref<WindowInfo[]>([]);
const hovered = ref<WindowInfo | null>(null);
const cursor = ref({ x: 0, y: 0 });
const excluded = ref<string[]>([]);
let timer: ReturnType<typeof setInterval> | null = null;

onMounted(async () => {
  try {
    const [wins, config] = await Promise.all([
      commands.privacyGetWindows(),
      commands.configGetAll(),
    ]);
    excluded.value = config.privacyRules;
    windows.value = wins;
  } catch {
    await commands.privacyClosePicker(null);
  }

  window.addEventListener('keydown', onKey);
  // Poll cursor position since overlay ignores cursor events
  timer = setInterval(pollCursor, 50);
});

onUnmounted(() => {
  window.removeEventListener('keydown', onKey);
  if (timer) clearInterval(timer);
});

async function pollCursor() {
  try {
    const [x, y] = await commands.privacyGetCursorPos();
    cursor.value = { x, y };
    // Find window under cursor
    const hit = windows.value.find((w) => {
      if (!w.bounds) return false;
      return (
        x >= w.bounds.x &&
        x <= w.bounds.x + w.bounds.width &&
        y >= w.bounds.y &&
        y <= w.bounds.y + w.bounds.height
      );
    });
    hovered.value = hit ?? null;
  } catch {
    // Silently retry
  }
}

const isExcluded = computed(() => {
  if (!hovered.value) return false;
  const name = hovered.value.processName;
  const rule = `${name}::${hovered.value.title}`;
  return excluded.value.some(
    (r) => r === rule || r === name || r === hovered.value!.title,
  );
});

const frameStyle = computed(() => {
  if (!hovered.value?.bounds) return { display: 'none' };
  const b = hovered.value.bounds;
  return {
    left: `${b.x}px`,
    top: `${b.y}px`,
    width: `${b.width}px`,
    height: `${b.height}px`,
  };
});

async function select() {
  if (hovered.value && !isExcluded.value) {
    await commands.privacyClosePicker(hovered.value);
  }
}

async function onKey(e: KeyboardEvent) {
  if (e.key === 'Escape') {
    await commands.privacyClosePicker(null);
  } else if (e.key === 'Enter') {
    await select();
  }
}
</script>

<template>
  <div class="picker">
    <div v-if="hovered" class="picker__frame" :style="frameStyle">
      <span class="picker__label">
        {{ hovered.title || '未命名' }} — {{ hovered.processName }}
        <template v-if="isExcluded">(已排除)</template>
      </span>
    </div>
    <div class="picker__hint">
      <span>将鼠标悬停在目标窗口上 · Enter 选择 · Esc 取消</span>
    </div>
  </div>
</template>

<style>
/* Override body bg for transparent overlay — NOT scoped so it hits <body> */
html, body, #app {
  background: transparent !important;
}
</style>

<style scoped>
.picker {
  position: fixed;
  inset: 0;
  pointer-events: none;
  user-select: none;
}

.picker__frame {
  position: fixed;
  border: 2px solid var(--color-primary);
  border-radius: 2px;
  pointer-events: none;
  z-index: 10;
  box-shadow: 0 0 0 1px rgba(0, 0, 0, 0.3), inset 0 0 0 1px rgba(255, 255, 255, 0.1);
}

.picker__label {
  position: absolute;
  top: -28px;
  left: 0;
  padding: 3px 10px;
  background: var(--color-primary);
  color: var(--text-on-primary);
  font-size: 12px;
  font-weight: var(--fw-medium);
  border-radius: var(--radius-sm);
  white-space: nowrap;
  max-width: 500px;
  overflow: hidden;
  text-overflow: ellipsis;
}

.picker__hint {
  position: fixed;
  bottom: 60px;
  left: 50%;
  transform: translateX(-50%);
  padding: var(--space-2) var(--space-5);
  border-radius: var(--radius-full);
  background: var(--bg-surface);
  border: 1px solid var(--border-default);
  color: var(--text-secondary);
  font-size: var(--text-sm);
  z-index: 20;
}
</style>
