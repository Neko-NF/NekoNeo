<script setup lang="ts">
interface Props {
  modelValue: boolean;
  disabled?: boolean;
}

const props = withDefaults(defineProps<Props>(), {
  disabled: false,
});

const emit = defineEmits<{
  'update:modelValue': [boolean];
}>();

function toggle() {
  if (!props.disabled) {
    emit('update:modelValue', !props.modelValue);
  }
}
</script>

<template>
  <div
    class="n-switch"
    :class="{ 'n-switch--on': modelValue, 'n-switch--disabled': disabled }"
    role="switch"
    :aria-checked="modelValue"
    :tabindex="disabled ? -1 : 0"
    @click="toggle"
    @keydown.enter.prevent="toggle"
    @keydown.space.prevent="toggle"
  />
</template>

<style scoped>
.n-switch {
  position: relative;
  width: 36px;
  height: 20px;
  border-radius: var(--radius-full);
  background: var(--border-strong);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-standard);
}

.n-switch::after {
  content: '';
  position: absolute;
  top: 3px;
  left: 3px;
  width: 14px;
  height: 14px;
  border-radius: var(--radius-full);
  background: var(--text-on-primary);
  transition: transform var(--duration-fast) var(--ease-standard);
}

.n-switch--on {
  background: var(--color-primary);
}

.n-switch--on::after {
  transform: translateX(16px);
}

.n-switch--disabled {
  opacity: 0.45;
  cursor: not-allowed;
}
</style>
