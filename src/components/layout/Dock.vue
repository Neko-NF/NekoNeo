<script setup lang="ts">
import { useRoute, useRouter } from 'vue-router';

const route = useRoute();
const router = useRouter();

const navItems = [
  { name: 'dashboard', label: '仪表盘', icon: '⌂' },
  { name: 'settings', label: '设置', icon: '⚙' },
  { name: 'privacy', label: '隐私', icon: '◫' },
  { name: 'console', label: '日志', icon: '▤' },
  { name: 'about', label: '关于', icon: '◎' },
];
</script>

<template>
  <nav class="dock" aria-label="主导航">
    <button
      v-for="item in navItems"
      :key="item.name"
      class="dock-item"
      :class="{ 'dock-item--active': route.name === item.name }"
      :aria-current="route.name === item.name ? 'page' : undefined"
      :aria-label="item.label"
      @click="router.push({ name: item.name })"
    >
      <span class="dock-item__icon">{{ item.icon }}</span>
      <span class="dock-item__label">{{ item.label }}</span>
    </button>
  </nav>
</template>

<style scoped>
.dock {
  position: fixed;
  left: 50%;
  bottom: var(--space-4);
  transform: translateX(-50%);
  z-index: 100;
  display: flex;
  align-items: center;
  gap: var(--dock-gap);
  padding: var(--dock-padding);
  background: var(--dock-bg);
  border: 1px solid var(--dock-border);
  border-radius: var(--radius-full);
  box-shadow: var(--dock-shadow);
}

.dock-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 3px;
  width: var(--dock-item-size);
  height: var(--dock-item-size);
  border: 0;
  border-radius: var(--radius-md);
  background: transparent;
  color: var(--text-tertiary);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-standard), color var(--duration-fast) var(--ease-standard);
}

.dock-item:hover {
  background: var(--bg-hover);
  color: var(--text-secondary);
}

.dock-item--active {
  background: var(--color-primary-muted);
  color: var(--color-primary);
}

.dock-item__icon {
  font-size: 18px;
  line-height: 1;
}

.dock-item__label {
  font-size: var(--text-xs);
  font-weight: var(--fw-medium);
  line-height: 1;
}
</style>
