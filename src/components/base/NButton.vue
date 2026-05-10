<script setup lang="ts">
interface Props {
  variant?: 'primary' | 'secondary' | 'danger' | 'ghost';
  size?: 'sm' | 'md' | 'lg';
  disabled?: boolean;
  type?: 'button' | 'submit' | 'reset';
}

withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  disabled: false,
  type: 'button',
});
</script>

<template>
  <button class="n-btn" :class="[`n-btn--${variant}`, `n-btn--${size}`]" :disabled="disabled" :type="type">
    <slot />
  </button>
</template>

<style scoped>
.n-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-2);
  padding: var(--space-2) var(--space-4);
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  font-size: var(--text-base);
  font-weight: var(--fw-medium);
  line-height: var(--leading-tight);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-standard), border-color var(--duration-fast) var(--ease-standard), color var(--duration-fast) var(--ease-standard);
}

.n-btn--primary {
  background: var(--color-primary);
  color: var(--text-on-primary);
}

.n-btn--secondary {
  background: var(--bg-elevated);
  border-color: var(--border-default);
  color: var(--text-primary);
}

.n-btn--danger {
  background: var(--color-danger-muted);
  border-color: var(--color-danger);
  color: var(--color-danger);
}

.n-btn--ghost {
  background: transparent;
  color: var(--text-secondary);
}

.n-btn--sm {
  padding: var(--space-1) var(--space-3);
  font-size: var(--text-sm);
}

.n-btn--lg {
  padding: var(--space-3) var(--space-6);
}

.n-btn:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}
</style>
