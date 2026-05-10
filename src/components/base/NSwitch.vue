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
  width: var(--switch-w);
  height: var(--switch-h);
  border-radius: var(--radius-full);
  background: var(--border-strong);
  cursor: pointer;
  flex-shrink: 0;
  transition: background var(--duration-fast) var(--ease-standard);
}

.n-switch::after {
  content: '';
  position: absolute;
  top: calc((var(--switch-h) - var(--switch-knob)) / 2);
  left: calc((var(--switch-h) - var(--switch-knob)) / 2);
  width: var(--switch-knob);
  height: var(--switch-knob);
  border-radius: var(--radius-full);
  background: #fff;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.2);
  transition: transform var(--duration-fast) var(--ease-standard);
}

.n-switch--on {
  background: var(--color-primary);
}

.n-switch--on::after {
  transform: translateX(calc(var(--switch-w) - var(--switch-h)));
}

.n-switch--disabled {
  opacity: 0.4;
  cursor: not-allowed;
}
</style>
