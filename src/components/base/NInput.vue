<script setup lang="ts">
interface Props {
  modelValue: string | number;
  type?: 'text' | 'password' | 'number' | 'url';
  placeholder?: string;
  disabled?: boolean;
  min?: number;
  max?: number;
  step?: number;
}

withDefaults(defineProps<Props>(), {
  type: 'text',
  placeholder: '',
  disabled: false,
  min: undefined,
  max: undefined,
  step: undefined,
});

const emit = defineEmits<{
  'update:modelValue': [string];
  blur: [];
}>();
</script>

<template>
  <input
    class="n-input"
    :type="type"
    :value="modelValue"
    :placeholder="placeholder"
    :disabled="disabled"
    :min="min"
    :max="max"
    :step="step"
    @input="emit('update:modelValue', ($event.target as HTMLInputElement).value)"
    @blur="emit('blur')"
  />
</template>

<style scoped>
.n-input {
  width: 100%;
  min-height: var(--input-h);
  padding: 0 var(--space-3);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  background: var(--bg-input);
  color: var(--text-primary);
  font-size: var(--text-sm);
  transition: border-color var(--duration-fast) var(--ease-standard),
              background var(--duration-fast) var(--ease-standard),
              box-shadow var(--duration-fast) var(--ease-standard);
}

.n-input::placeholder {
  color: var(--text-tertiary);
}

.n-input:hover {
  border-color: var(--border-strong);
}

.n-input:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 1px var(--color-primary);
  background: var(--bg-surface);
}

.n-input:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}
</style>
