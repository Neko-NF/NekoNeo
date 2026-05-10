import { createRouter, createWebHashHistory } from 'vue-router';

const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', name: 'dashboard', component: () => import('@/views/Dashboard.vue') },
    { path: '/settings', name: 'settings', component: () => import('@/views/Settings.vue') },
    { path: '/privacy', name: 'privacy', component: () => import('@/views/Privacy.vue') },
    { path: '/console', name: 'console', component: () => import('@/views/Console.vue') },
    { path: '/about', name: 'about', component: () => import('@/views/About.vue') },
  ],
});

export default router;
