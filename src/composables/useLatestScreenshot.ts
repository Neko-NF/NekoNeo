import { computed, ref } from 'vue';
import { commands, getErrorMessage } from '@/api/commands';
import type { LatestScreenshot } from '@/types';

export function useLatestScreenshot() {
  const latest = ref<LatestScreenshot | null>(null);
  const loading = ref(false);
  const capturing = ref(false);
  const error = ref<string | null>(null);

  const previewSrc = computed(() => {
    return latest.value?.dataUrl ?? null;
  });

  async function loadLatest() {
    loading.value = true;
    error.value = null;

    try {
      latest.value = await commands.screenshotGetLatest();
    } catch (err) {
      error.value = getErrorMessage(err);
    } finally {
      loading.value = false;
    }
  }

  async function captureNow() {
    capturing.value = true;
    error.value = null;

    try {
      await commands.screenshotCaptureNow();
      await loadLatest();
    } catch (err) {
      error.value = getErrorMessage(err);
    } finally {
      capturing.value = false;
    }
  }

  return {
    latest,
    loading,
    capturing,
    error,
    previewSrc,
    loadLatest,
    captureNow,
  };
}
