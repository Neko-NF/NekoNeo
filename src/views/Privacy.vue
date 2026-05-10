<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref } from 'vue';
import { listen } from '@tauri-apps/api/event';
import NButton from '@/components/base/NButton.vue';
import NBadge from '@/components/base/NBadge.vue';
import NInput from '@/components/base/NInput.vue';
import { commands } from '@/api/commands';
import { useConfigStore } from '@/stores/config';
import { useToastStore } from '@/stores/toast';
import type { WindowInfo } from '@/types';

const windows = ref<WindowInfo[]>([]);
const loading = ref(false);
const saving = ref(false);
const pickerOpen = ref(false);
const searchQuery = ref('');

const configStore = useConfigStore();
const toastStore = useToastStore();
const privacyRules = computed(() => configStore.config.privacyRules);

let unlistenPicker: (() => void) | null = null;

function buildRule(w: WindowInfo) {
  return `${w.processName}::${w.title}`;
}
function hasRule(w: WindowInfo) {
  return privacyRules.value.includes(buildRule(w));
}

const filteredWindows = computed(() => {
  const q = searchQuery.value.toLowerCase().trim();
  if (!q) return windows.value;
  return windows.value.filter(
    (w) =>
      w.title.toLowerCase().includes(q) ||
      w.processName.toLowerCase().includes(q) ||
      (w.path && w.path.toLowerCase().includes(q)),
  );
});

const excludedCount = computed(() => windows.value.filter((w) => hasRule(w)).length);

async function loadWindows() {
  loading.value = true;
  windows.value = await commands.privacyGetWindows();
  loading.value = false;
}

async function addRule(w: WindowInfo) {
  saving.value = true;
  const next = [...privacyRules.value];
  const rule = buildRule(w);
  if (!next.includes(rule)) {
    next.push(rule);
    await configStore.set('privacyRules', next);
    toastStore.success(`已添加隐私规则: ${w.processName}`);
  }
  saving.value = false;
}

async function removeRule(rule: string) {
  saving.value = true;
  await configStore.set(
    'privacyRules',
    privacyRules.value.filter((r) => r !== rule),
  );
  saving.value = false;
}

async function openPicker() {
  pickerOpen.value = true;
  try {
    await commands.privacyOpenPicker();
  } catch {
    pickerOpen.value = false;
  }
}

onMounted(async () => {
  await loadWindows();
  unlistenPicker = await listen<WindowInfo | null>('picker:selected', async (event) => {
    pickerOpen.value = false;
    if (event.payload) {
      await addRule(event.payload);
    }
  });
});

onUnmounted(() => {
  unlistenPicker?.();
});
</script>

<template>
  <div class="privacy">
    <!-- Header -->
    <section class="privacy-hero">
      <div>
        <p class="privacy-hero__eyebrow">隐私规则</p>
        <h1>窗口排除规则</h1>
        <p class="privacy-hero__desc">
          命中规则的前台窗口在上报时会被脱敏处理，截图也会按策略进行模糊。
          当前已配置 <strong>{{ privacyRules.length }}</strong> 条规则，覆盖 <strong>{{ excludedCount }}</strong> 个可见窗口。
        </p>
      </div>
    </section>

    <!-- Active rules -->
    <section class="card">
      <div class="card__head">
        <h2>已生效规则</h2>
        <span class="card__count">{{ privacyRules.length }} 条</span>
      </div>

      <div v-if="privacyRules.length > 0" class="rule-list">
        <div v-for="rule in privacyRules" :key="rule" class="rule-item">
          <code class="rule-item__text">{{ rule }}</code>
          <NButton variant="ghost" size="sm" :disabled="saving" @click="removeRule(rule)">移除</NButton>
        </div>
      </div>
      <p v-else class="empty">暂无规则，从下方窗口列表中添加需要脱敏的窗口。</p>
    </section>

    <!-- Window list -->
    <section class="card">
      <div class="card__head">
        <h2>当前窗口</h2>
        <div class="card__head-actions">
          <span class="card__count">已排除 {{ excludedCount }} / {{ windows.length }}</span>
          <NButton variant="secondary" size="sm" :disabled="loading" @click="loadWindows()">
            {{ loading ? '刷新中...' : '刷新' }}
          </NButton>
          <NButton variant="primary" size="sm" :disabled="loading || windows.length === 0 || pickerOpen" @click="openPicker()">
            {{ pickerOpen ? '选择器中...' : '可视化选择器' }}
          </NButton>
        </div>
      </div>

      <div class="card__search">
        <NInput v-model="searchQuery" placeholder="搜索窗口标题、进程名或路径..." />
      </div>

      <div class="window-list">
        <div
          v-for="w in filteredWindows"
          :key="`${w.pid}-${w.title}`"
          class="window-item"
          :class="{ 'window-item--excluded': hasRule(w) }"
        >
          <div class="window-item__info">
            <strong>{{ w.title || '未命名窗口' }}</strong>
            <span>{{ w.processName }} · PID {{ w.pid }}</span>
            <code class="window-item__path">{{ w.path || '无路径信息' }}</code>
          </div>
          <div class="window-item__action">
            <NBadge v-if="hasRule(w)" tone="success">已排除</NBadge>
            <NButton v-else variant="secondary" size="sm" :disabled="saving" @click="addRule(w)">
              加入排除
            </NButton>
          </div>
        </div>
        <p v-if="filteredWindows.length === 0 && !loading" class="empty">
          {{ searchQuery ? '没有匹配的窗口。' : '当前未检索到可见窗口。' }}
        </p>
      </div>
    </section>

  </div>
</template>

<style scoped>
.privacy {
  display: grid;
  gap: var(--space-5);
}

/* ── Hero ─────────────────────────────────────────────────────── */

.privacy-hero {
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
  border-left: 3px solid var(--color-primary);
}

.privacy-hero__eyebrow {
  margin: 0 0 var(--space-1);
  color: var(--text-secondary);
  font-size: var(--text-sm);
  text-transform: uppercase;
  letter-spacing: 0.06em;
}

.privacy-hero h1 {
  margin: 0;
  font-size: var(--text-xl);
  font-weight: var(--fw-semibold);
}

.privacy-hero__desc {
  margin: var(--space-3) 0 0;
  color: var(--text-secondary);
  font-size: var(--text-sm);
  line-height: var(--leading-normal);
  max-width: 64ch;
}

/* ── Card ─────────────────────────────────────────────────────── */

.card {
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.card__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  margin-bottom: var(--space-4);
}

.card__head h2 {
  margin: 0;
  font-size: var(--text-md);
  font-weight: var(--fw-semibold);
}

.card__head-actions {
  display: flex;
  align-items: center;
  gap: var(--space-2);
}

.card__count {
  color: var(--text-tertiary);
  font-size: var(--text-xs);
  font-family: var(--font-mono);
  white-space: nowrap;
}

.card__search {
  margin-bottom: var(--space-3);
}

.empty {
  color: var(--text-tertiary);
  font-size: var(--text-sm);
  margin: var(--space-3) 0;
  text-align: center;
}

/* ── Rules ─────────────────────────────────────────────────────── */

.rule-list {
  display: grid;
  gap: var(--space-2);
}

.rule-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
}

.rule-item__text {
  font-family: var(--font-mono);
  font-size: var(--text-sm);
  color: var(--color-primary);
  word-break: break-all;
}

/* ── Window list ───────────────────────────────────────────────── */

.window-list {
  display: grid;
  gap: var(--space-2);
}

.window-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-3);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-surface);
  transition: border-color var(--duration-fast) var(--ease-standard);
}

.window-item:hover {
  border-color: var(--border-strong);
}

.window-item--excluded {
  background: var(--bg-elevated);
  opacity: 0.75;
}

.window-item__info {
  display: grid;
  gap: var(--space-1);
  min-width: 0;
}

.window-item__info strong {
  font-size: var(--text-base);
  font-weight: var(--fw-medium);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.window-item__info span {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.window-item__path {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  color: var(--text-tertiary);
  word-break: break-all;
}

.window-item__action {
  flex-shrink: 0;
}

@media (max-width: 900px) {
  .window-item { flex-direction: column; align-items: stretch; }
  .window-item__action { align-self: flex-end; }
}
</style>
