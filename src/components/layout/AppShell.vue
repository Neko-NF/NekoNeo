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
  background: var(--bg-app);
}
</style>
