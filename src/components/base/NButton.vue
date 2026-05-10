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
  <button
    class="n-btn"
    :class="[`n-btn--${variant}`, `n-btn--${size}`]"
    :disabled="disabled"
    :type="type"
  >
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
  min-height: var(--btn-h-md);
  border: 1px solid transparent;
  border-radius: var(--radius-sm);
  font-size: var(--text-base);
  font-weight: var(--fw-medium);
  line-height: var(--leading-tight);
  cursor: pointer;
  user-select: none;
  white-space: nowrap;
  transition: background var(--duration-fast) var(--ease-standard),
              border-color var(--duration-fast) var(--ease-standard),
              color var(--duration-fast) var(--ease-standard),
              opacity var(--duration-fast) var(--ease-standard);
}

.n-btn--primary {
  background: var(--color-primary);
  color: var(--text-on-primary);
}

.n-btn--primary:hover {
  filter: brightness(1.1);
}

.n-btn--primary:active {
  filter: brightness(0.95);
}

.n-btn--secondary {
  background: var(--bg-elevated);
  border-color: var(--border-default);
  color: var(--text-primary);
}

.n-btn--secondary:hover {
  background: var(--bg-active);
  border-color: var(--border-strong);
}

.n-btn--secondary:active {
  background: var(--bg-hover);
}

.n-btn--danger {
  background: var(--color-danger-muted);
  border-color: var(--color-danger);
  color: var(--color-danger);
}

.n-btn--danger:hover {
  background: var(--color-danger);
  color: #fff;
}

.n-btn--ghost {
  background: transparent;
  color: var(--text-secondary);
}

.n-btn--ghost:hover {
  background: var(--bg-hover);
  color: var(--text-primary);
}

.n-btn--sm {
  padding: var(--space-1) var(--space-3);
  min-height: var(--btn-h-sm);
  font-size: var(--text-sm);
  border-radius: var(--radius-sm);
}

.n-btn--lg {
  padding: var(--space-3) var(--space-5);
  min-height: var(--btn-h-lg);
  font-size: var(--text-md);
}

.n-btn:disabled {
  opacity: 0.4;
  cursor: not-allowed;
  filter: none;
}
</style>
