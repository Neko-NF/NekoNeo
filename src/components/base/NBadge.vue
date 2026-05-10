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
    <span class="n-badge__dot" />
    <slot />
  </span>
</template>

<style scoped>
.n-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-1);
  padding: var(--space-1) var(--space-3);
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--fw-medium);
  line-height: var(--leading-tight);
  white-space: nowrap;
}

.n-badge__dot {
  width: 8px;
  height: 8px;
  border-radius: var(--radius-full);
  flex-shrink: 0;
}

.n-badge--running {
  background: var(--color-primary-muted);
  color: var(--color-primary);
}
.n-badge--running .n-badge__dot { background: var(--color-primary); }

.n-badge--success {
  background: var(--color-success-muted);
  color: var(--color-success);
}
.n-badge--success .n-badge__dot { background: var(--color-success); }

.n-badge--warning {
  background: var(--color-warning-muted);
  color: var(--color-warning);
}
.n-badge--warning .n-badge__dot { background: var(--color-warning); }

.n-badge--danger {
  background: var(--color-danger-muted);
  color: var(--color-danger);
}
.n-badge--danger .n-badge__dot { background: var(--color-danger); }

.n-badge--neutral {
  background: var(--bg-elevated);
  color: var(--text-secondary);
}
.n-badge--neutral .n-badge__dot { background: var(--text-tertiary); }
</style>
