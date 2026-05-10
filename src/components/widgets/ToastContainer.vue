<script setup lang="ts">
import { useToastStore } from '@/stores/toast';

const toast = useToastStore();
</script>

<template>
  <Teleport to="body">
    <div v-if="toast.items.length" class="toast-host">
      <TransitionGroup name="toast">
        <div
          v-for="t in toast.items"
          :key="t.id"
          class="toast"
          :class="`toast--${t.type}`"
          @click="toast.remove(t.id)"
        >
          {{ t.text }}
        </div>
      </TransitionGroup>
    </div>
  </Teleport>
</template>

<style scoped>
.toast-host {
  position: fixed;
  top: 44px;
  left: 50%;
  transform: translateX(-50%);
  z-index: 9999;
  display: grid;
  gap: var(--space-2);
  justify-items: center;
  pointer-events: none;
}

.toast {
  padding: var(--space-2) var(--space-4);
  border-radius: var(--radius-full);
  font-size: var(--text-sm);
  font-weight: var(--fw-medium);
  line-height: var(--leading-tight);
  white-space: nowrap;
  pointer-events: auto;
  cursor: pointer;
  box-shadow: var(--elevation-2);
}

.toast--success {
  background: var(--color-success-muted);
  color: var(--color-success);
  border: 1px solid var(--color-success);
}

.toast--warn {
  background: var(--color-warning-muted);
  color: var(--color-warning);
  border: 1px solid var(--color-warning);
}

.toast--error {
  background: var(--color-danger-muted);
  color: var(--color-danger);
  border: 1px solid var(--color-danger);
}

.toast--info {
  background: var(--color-primary-muted);
  color: var(--color-primary);
  border: 1px solid var(--color-primary);
}

/* Transitions */
.toast-enter-active {
  transition: all var(--duration-base) var(--ease-decelerate);
}
.toast-leave-active {
  transition: all var(--duration-fast) var(--ease-accelerate);
}
.toast-enter-from {
  opacity: 0;
  transform: translateY(-10px) scale(0.9);
}
.toast-leave-to {
  opacity: 0;
  transform: translateY(-6px) scale(0.95);
}
</style>
