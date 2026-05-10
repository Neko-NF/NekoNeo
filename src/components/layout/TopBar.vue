<template>
  <header class="top-bar" data-tauri-drag-region>
    <div class="top-bar__brand">
      <span class="top-bar__dot" />
      <span>NekoNeo</span>
    </div>
    <div class="top-bar__meta">Tauri Alpha Scaffold</div>
  </header>
</template>

<style scoped>
.top-bar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 36px;
  padding: 0 var(--space-4);
  border-bottom: 1px solid var(--border-default);
  background: color-mix(in srgb, var(--bg-app) 92%, var(--bg-surface) 8%);
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.top-bar__brand {
  display: inline-flex;
  align-items: center;
  gap: var(--space-2);
  color: var(--text-primary);
  font-weight: var(--fw-semibold);
}

.top-bar__dot {
  width: 8px;
  height: 8px;
  border-radius: var(--radius-full);
  background: var(--color-primary);
}

.top-bar__meta {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
}
</style>
