<script setup lang="ts">
interface Props {
  tone?: 'running' | 'success' | 'warning' | 'danger' | 'neutral';
}

withDefaults(defineProps<Props>(), {
  tone: 'neutral',
});
</script>

<template>
  <span class="n-badge" :class="`n-badge--${tone}`">
    <slot />
  </span>
</template>

<style scoped>
.n-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: 2px var(--space-2);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--fw-medium);
}

.n-badge--running {
  background: var(--color-primary-muted);
  color: var(--color-primary);
}

.n-badge--success {
  background: var(--color-success-muted);
  color: var(--color-success);
}

.n-badge--warning {
  background: var(--color-warning-muted);
  color: var(--color-warning);
}

.n-badge--danger {
  background: var(--color-danger-muted);
  color: var(--color-danger);
}

.n-badge--neutral {
  background: var(--bg-elevated);
  color: var(--text-secondary);
}
</style>
