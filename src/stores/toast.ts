import { defineStore } from 'pinia';
import { ref } from 'vue';

export type ToastType = 'success' | 'warn' | 'error' | 'info';

export interface Toast {
  id: number;
  text: string;
  type: ToastType;
}

let nextId = 0;
const DEFAULT_DURATION = 3500;

export const useToastStore = defineStore('toast', () => {
  const items = ref<Toast[]>([]);

  function push(text: string, type: ToastType = 'info', durationMs: number = DEFAULT_DURATION) {
    // Dedup: skip if same text+type is already showing
    if (items.value.some((t) => t.text === text && t.type === type)) return;

    const id = nextId++;
    items.value.push({ id, text, type });

    setTimeout(() => {
      remove(id);
    }, durationMs);
  }

  function remove(id: number) {
    const idx = items.value.findIndex((t) => t.id === id);
    if (idx !== -1) items.value.splice(idx, 1);
  }

  function success(text: string, dur?: number) { push(text, 'success', dur); }
  function warn(text: string, dur?: number) { push(text, 'warn', dur); }
  function error(text: string, dur?: number) { push(text, 'error', dur); }
  function info(text: string, dur?: number) { push(text, 'info', dur); }

  return { items, push, remove, success, warn, error, info };
});
