<template>
  <main class="page-container">
    <slot />
  </main>
</template>

<style scoped>
.page-container {
  display: flex;
  flex-direction: column;
  height: calc(100vh - 36px);
  padding: var(--space-6);
  padding-bottom: calc(var(--dock-height) + var(--space-8));
  overflow-y: auto;
  box-sizing: border-box;
}
</style>
