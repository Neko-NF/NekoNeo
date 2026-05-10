<script setup lang="ts">
import { onMounted } from 'vue';
import { ref } from 'vue';
import NButton from '@/components/base/NButton.vue';
import { commands } from '@/api/commands';
import type { WindowInfo } from '@/types';

const windows = ref<WindowInfo[]>([]);
const loading = ref(false);

async function loadWindows() {
  loading.value = true;
  windows.value = await commands.privacyGetWindows();
  loading.value = false;
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
        <h1>窗口排除基础骨架</h1>
      </div>
      <NButton variant="secondary" :disabled="loading" @click="loadWindows()">
        {{ loading ? '加载中' : '刷新窗口列表' }}
      </NButton>
    </div>

    <div class="privacy-list">
      <div v-for="item in windows" :key="`${item.pid}-${item.title}`" class="privacy-item">
        <strong>{{ item.title || '未命名窗口' }}</strong>
        <span>{{ item.processName }} · PID {{ item.pid }}</span>
      </div>
      <p v-if="windows.length === 0" class="privacy-empty">当前仅提供占位窗口列表与后续接入点。</p>
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

.privacy-item {
  display: grid;
  gap: var(--space-1);
  padding: var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
}

.privacy-item span,
.privacy-empty {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}
</style>
