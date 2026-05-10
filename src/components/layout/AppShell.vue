<script setup lang="ts">
import Dock from './Dock.vue';
import PageContainer from './PageContainer.vue';
import TopBar from './TopBar.vue';
</script>

<template>
  <div class="app-shell">
    <TopBar />
    <PageContainer>
      <slot />
    </PageContainer>
    <Dock />
  </div>
</template>

<style scoped>
.app-shell {
  min-height: 100vh;
  background:
    radial-gradient(circle at top, color-mix(in srgb, var(--color-primary) 12%, transparent) 0%, transparent 38%),
    linear-gradient(180deg, color-mix(in srgb, var(--bg-surface) 35%, var(--bg-app) 65%) 0%, var(--bg-app) 100%);
}
</style>
