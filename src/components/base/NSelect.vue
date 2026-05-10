<script setup lang="ts">
interface Option {
  label: string;
  value: string;
}

interface Props {
  modelValue: string;
  options: Option[];
  disabled?: boolean;
}

withDefaults(defineProps<Props>(), {
  disabled: false,
});

const emit = defineEmits<{
  'update:modelValue': [string];
  change: [];
}>();
</script>

<template>
  <select
    class="n-select"
    :value="modelValue"
    :disabled="disabled"
    @change="
      emit('update:modelValue', ($event.target as HTMLSelectElement).value);
      emit('change');
    "
  >
    <option v-for="option in options" :key="option.value" :value="option.value">
      {{ option.label }}
    </option>
  </select>
</template>

<style scoped>
.n-select {
  width: 100%;
  min-height: var(--input-h);
  padding: 0 var(--space-3);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  background: var(--bg-input);
  color: var(--text-primary);
  font-size: var(--text-sm);
  cursor: pointer;
  appearance: none;
  background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 24 24' fill='none' stroke='%23999' stroke-width='2'%3E%3Cpolyline points='6 9 12 15 18 9'/%3E%3C/svg%3E");
  background-repeat: no-repeat;
  background-position: right 8px center;
  padding-right: 28px;
  transition: border-color var(--duration-fast) var(--ease-standard),
              background var(--duration-fast) var(--ease-standard),
              box-shadow var(--duration-fast) var(--ease-standard);
}

.n-select:hover {
  border-color: var(--border-strong);
}

.n-select:focus {
  outline: none;
  border-color: var(--color-primary);
  box-shadow: 0 0 0 1px var(--color-primary);
  background: var(--bg-surface);
}

.n-select:disabled {
  opacity: 0.45;
  cursor: not-allowed;
}
</style>
