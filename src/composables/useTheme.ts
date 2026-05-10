import { onUnmounted } from 'vue';
import type { AppConfig } from '@/types';

function hexToRgba(hex: string, alpha: number): string {
  const r = parseInt(hex.slice(1, 3), 16);
  const g = parseInt(hex.slice(3, 5), 16);
  const b = parseInt(hex.slice(5, 7), 16);
  return `rgba(${r}, ${g}, ${b}, ${alpha})`;
}

function adjustColor(hex: string, amount: number): string {
  const normalize = (value: number) => Math.max(0, Math.min(255, value));
  const r = normalize(parseInt(hex.slice(1, 3), 16) + amount);
  const g = normalize(parseInt(hex.slice(3, 5), 16) + amount);
  const b = normalize(parseInt(hex.slice(5, 7), 16) + amount);
  return `#${r.toString(16).padStart(2, '0')}${g.toString(16).padStart(2, '0')}${b.toString(16).padStart(2, '0')}`;
}

export function useTheme() {
  let mediaQueryCleanup: (() => void) | null = null;

  function applyTheme(config: AppConfig) {
    const root = document.documentElement;
    const nextTheme =
      config.themeMode === 'system'
        ? window.matchMedia('(prefers-color-scheme: dark)').matches
          ? 'dark'
        : 'light'
        : config.themeMode;

    root.setAttribute('data-theme-mode', config.themeMode);
    root.setAttribute('data-theme', nextTheme);
    root.style.setProperty('--color-primary', config.seedColor);
    root.style.setProperty('--color-primary-hover', adjustColor(config.seedColor, -24));
    root.style.setProperty('--color-primary-muted', hexToRgba(config.seedColor, 0.12));
    root.style.setProperty('--ui-font', `"${config.uiFont}"`);
    root.style.setProperty('--ui-scale', `${config.uiScale}%`);
    document.body.style.zoom = `${config.uiScale}%`;
  }

  function watchSystemTheme() {
    const media = window.matchMedia('(prefers-color-scheme: dark)');
    const listener = () => {
      const root = document.documentElement;
      if (root.getAttribute('data-theme-mode') === 'system') {
        root.setAttribute('data-theme', media.matches ? 'dark' : 'light');
      }
    };

    media.addEventListener('change', listener);
    mediaQueryCleanup = () => media.removeEventListener('change', listener);
  }

  onUnmounted(() => {
    mediaQueryCleanup?.();
  });

  return { applyTheme, watchSystemTheme };
}
