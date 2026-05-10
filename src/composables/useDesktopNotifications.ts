import { useConfigStore } from '@/stores/config';

type NotificationLevel = 'info' | 'success' | 'warn' | 'error';

export function useDesktopNotifications() {
  const configStore = useConfigStore();

  async function ensurePermission() {
    if (!('Notification' in window)) {
      return 'denied';
    }

    if (Notification.permission === 'default') {
      return Notification.requestPermission();
    }

    return Notification.permission;
  }

  async function notify(level: NotificationLevel, title: string, body: string) {
    if (!configStore.config.enableNotification) {
      return;
    }

    if (configStore.config.doNotDisturb && level !== 'error') {
      return;
    }

    const permission = await ensurePermission();
    if (permission !== 'granted') {
      return;
    }

    new Notification(title, { body });
  }

  return {
    notify,
  };
}
