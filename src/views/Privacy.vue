<script setup lang="ts">
import { onMounted } from 'vue';
import { computed, ref } from 'vue';
import NButton from '@/components/base/NButton.vue';
import { commands } from '@/api/commands';
import { useConfigStore } from '@/stores/config';
import type { WindowInfo } from '@/types';

const windows = ref<WindowInfo[]>([]);
const loading = ref(false);
const saving = ref(false);
const configStore = useConfigStore();

const privacyRules = computed(() => configStore.config.privacyRules);

function buildRule(window: WindowInfo) {
  return `${window.processName}::${window.title}`;
}

function hasRule(window: WindowInfo) {
  return privacyRules.value.includes(buildRule(window));
}

async function loadWindows() {
  loading.value = true;
  windows.value = await commands.privacyGetWindows();
  loading.value = false;
}

async function addRule(window: WindowInfo) {
  saving.value = true;
  const nextRules = [...privacyRules.value];
  const rule = buildRule(window);
  if (!nextRules.includes(rule)) {
    nextRules.push(rule);
    await configStore.set('privacyRules', nextRules);
  }
  saving.value = false;
}

async function removeRule(rule: string) {
  saving.value = true;
  await configStore.set(
    'privacyRules',
    privacyRules.value.filter((item) => item !== rule),
  );
  saving.value = false;
}

onMounted(() => {
  void loadWindows();
});
</script>

<template>
  <section class="page-card">
    <div class="page-card__header">
      <div>
        <p class="page-card__eyebrow">隐私规则</p>
        <h1>窗口排除规则</h1>
      </div>
      <NButton variant="secondary" :disabled="loading" @click="loadWindows()">
        {{ loading ? '加载中' : '刷新窗口列表' }}
      </NButton>
    </div>

    <div class="privacy-active">
      <div class="privacy-active__head">
        <strong>已生效规则</strong>
        <span>{{ privacyRules.length }} 条</span>
      </div>
      <div v-if="privacyRules.length > 0" class="privacy-rule-list">
        <div v-for="rule in privacyRules" :key="rule" class="privacy-rule">
          <span>{{ rule }}</span>
          <NButton variant="ghost" size="sm" :disabled="saving" @click="removeRule(rule)">
            移除
          </NButton>
        </div>
      </div>
      <p v-else class="privacy-empty">当前没有隐私规则，命中规则的前台窗口会在上报中被脱敏。</p>
    </div>

    <div class="privacy-list">
      <div v-for="item in windows" :key="`${item.pid}-${item.title}`" class="privacy-item">
        <strong>{{ item.title || '未命名窗口' }}</strong>
        <span>{{ item.processName }} · PID {{ item.pid }}</span>
        <span class="privacy-item__path">{{ item.path || '无路径信息' }}</span>
        <div class="privacy-item__actions">
          <NButton
            v-if="!hasRule(item)"
            variant="secondary"
            size="sm"
            :disabled="saving"
            @click="addRule(item)"
          >
            加入排除
          </NButton>
          <NButton v-else variant="ghost" size="sm" :disabled="true">
            已排除
          </NButton>
        </div>
      </div>
      <p v-if="windows.length === 0" class="privacy-empty">当前未检索到可见窗口。</p>
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

.page-card__header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-4);
  margin-bottom: var(--space-5);
}

.page-card__eyebrow {
  margin: 0 0 var(--space-1);
  color: var(--text-secondary);
}

.page-card h1 {
  margin: 0;
}

.privacy-list {
  display: grid;
  gap: var(--space-3);
}

.privacy-active {
  display: grid;
  gap: var(--space-3);
  margin-bottom: var(--space-5);
  padding: var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
}

.privacy-active__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
}

.privacy-rule-list {
  display: grid;
  gap: var(--space-2);
}

.privacy-rule {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding: var(--space-3);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-surface);
}

.privacy-rule span {
  color: var(--text-primary);
  font-size: var(--text-sm);
  word-break: break-all;
}

.privacy-item {
  display: grid;
  gap: var(--space-1);
  padding: var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
}

.privacy-item__path {
  color: var(--text-tertiary);
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  word-break: break-all;
}

.privacy-item__actions {
  margin-top: var(--space-2);
}

.privacy-item span,
.privacy-empty {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}
</style>
