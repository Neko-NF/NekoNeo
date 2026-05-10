import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useUiStore = defineStore('ui', () => {
  const busy = ref(false);
  const pageTitle = ref('NekoNeo');

  return { busy, pageTitle };
});
