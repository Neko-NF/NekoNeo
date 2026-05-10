<script setup lang="ts">
import NSwitch from '@/components/base/NSwitch.vue';

interface Props {
  label: string;
  description: string;
  modelValue: boolean;
}

defineProps<Props>();

const emit = defineEmits<{
  'update:modelValue': [boolean];
}>();
</script>

<template>
  <div class="quick-toggle">
    <div class="quick-toggle__info">
      <strong>{{ label }}</strong>
      <span>{{ description }}</span>
    </div>
    <NSwitch :model-value="modelValue" @update:model-value="emit('update:modelValue', $event)" />
  </div>
</template>

<style scoped>
.quick-toggle {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-surface);
}

.quick-toggle__info {
  display: grid;
  gap: var(--space-1);
}

.quick-toggle__info span {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}
</style>
