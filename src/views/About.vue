<script setup lang="ts">
import { computed, onMounted, ref } from 'vue';
import NButton from '@/components/base/NButton.vue';
import NBadge from '@/components/base/NBadge.vue';
import { commands } from '@/api/commands';
import { useConfigStore } from '@/stores/config';
import { useMetricsStore } from '@/stores/metrics';
import type { UpdateInfo } from '@/types';

const configStore = useConfigStore();
const metricsStore = useMetricsStore();
const updateInfo = ref<UpdateInfo | null>(null);
const checking = ref(false);
const fingerprint = ref('');

const serverUrl = computed(() =>
  configStore.config.serverMode === 'production'
    ? configStore.config.serverUrlProd
    : configStore.config.serverUrlLocal,
);

const appRows = [
  { label: '版本', value: '1.0.0' },
  { label: '通道', value: configStore.config.updateChannel === 'stable' ? 'Stable' : 'Beta' },
  { label: '服务器', value: serverUrl.value },
];

const sysRows = computed(() => [
  { label: '系统', value: metricsStore.metrics?.osFriendlyName ?? '—' },
  { label: '主机', value: metricsStore.metrics?.hostname ?? '—' },
  { label: 'CPU', value: metricsStore.metrics?.cpuModel ?? '—' },
  { label: '核心', value: metricsStore.metrics?.cpuCores ? `${metricsStore.metrics.cpuCores} 核` : '—' },
  { label: '运行时间', value: metricsStore.metrics?.uptime ? `${Math.floor(metricsStore.metrics.uptime / 3600)}h` : '—' },
]);

async function checkUpdate() {
  checking.value = true;
  try {
    updateInfo.value = await commands.updateCheck(configStore.config.updateChannel);
  } finally {
    checking.value = false;
  }
}

onMounted(async () => {
  void checkUpdate();
  try {
    fingerprint.value = await commands.systemGetDeviceFingerprint();
  } catch { /* ignore */ }
});
</script>

<template>
  <div class="about">
    <!-- Brand -->
    <section class="brand">
      <div class="brand__logo">
        <span class="brand__dot" />
      </div>
      <div>
        <h1>NekoNeo</h1>
        <p>v1.0.0 · Tauri 2 · Vue 3</p>
      </div>
    </section>

    <!-- Info tables -->
    <div class="about-grid">
      <section class="card">
        <h2>应用</h2>
        <div class="rows">
          <div v-for="r in appRows" :key="r.label" class="row">
            <span>{{ r.label }}</span>
            <strong>{{ r.value }}</strong>
          </div>
        </div>
      </section>
      <section class="card">
        <h2>系统</h2>
        <div class="rows">
          <div v-for="r in sysRows" :key="r.label" class="row">
            <span>{{ r.label }}</span>
            <strong>{{ r.value }}</strong>
          </div>
        </div>
      </section>
    </div>

    <!-- Update -->
    <section class="card">
      <div class="card__head">
        <h2>更新</h2>
        <NButton variant="primary" size="sm" :disabled="checking" @click="checkUpdate()">
          {{ checking ? '检测中...' : '检查更新' }}
        </NButton>
      </div>
      <div v-if="updateInfo" class="update">
        <div class="update__head">
          <NBadge :tone="updateInfo.downloaded ? 'success' : 'neutral'">
            {{ updateInfo.downloaded ? '已下载' : '可更新' }}
          </NBadge>
          <strong>{{ updateInfo.version }}</strong>
          <span>{{ updateInfo.channel }}</span>
        </div>
        <p v-if="updateInfo.releaseNotes" class="update__notes">{{ updateInfo.releaseNotes }}</p>
      </div>
      <p v-else class="empty">当前已是最新版本。</p>
    </section>

    <!-- Fingerprint -->
    <section v-if="fingerprint" class="card">
      <h2>设备指纹</h2>
      <code class="fingerprint">{{ fingerprint }}</code>
    </section>

    <footer class="foot">
      <span>Built with Tauri 2 + Rust + Vue 3</span>
      <span>© 2026 NekoNeo</span>
    </footer>
  </div>
</template>

<style scoped>
.about {
  display: grid;
  gap: var(--space-5);
}

/* Brand */
.brand {
  display: flex;
  align-items: center;
  gap: var(--space-4);
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.brand__logo {
  width: 48px;
  height: 48px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  background: var(--color-primary-muted);
  flex-shrink: 0;
}

.brand__dot {
  width: 14px;
  height: 14px;
  border-radius: var(--radius-full);
  background: var(--color-primary);
}

.brand h1 {
  margin: 0;
  font-size: var(--text-xl);
  font-weight: var(--fw-bold);
}

.brand p {
  margin: 2px 0 0;
  color: var(--text-secondary);
  font-size: var(--text-sm);
  font-family: var(--font-mono);
}

/* Grid */
.about-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-5);
}

/* Card */
.card {
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.card h2 {
  margin: 0 0 var(--space-3);
  font-size: var(--text-md);
  font-weight: var(--fw-semibold);
}

.card__head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  margin-bottom: var(--space-3);
}
.card__head h2 { margin: 0; }

/* Rows */
.rows {
  display: grid;
  gap: 1px;
}

.row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: var(--space-3);
  padding: var(--space-2) var(--space-3);
  border-radius: var(--radius-sm);
  background: var(--bg-elevated);
}

.row span {
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.row strong {
  font-size: var(--text-sm);
  font-weight: var(--fw-semibold);
  font-family: var(--font-mono);
  text-align: right;
}

.empty {
  color: var(--text-tertiary);
  font-size: var(--text-sm);
  margin: 0;
}

/* Update */
.update__head {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex-wrap: wrap;
}
.update__head strong { font-family: var(--font-mono); }
.update__head span { color: var(--text-secondary); font-size: var(--text-sm); }

.update__notes {
  margin: var(--space-3) 0 0;
  padding: var(--space-3);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  color: var(--text-secondary);
  font-size: var(--text-sm);
  line-height: var(--leading-normal);
  white-space: pre-line;
}

/* Fingerprint */
.fingerprint {
  display: block;
  padding: var(--space-3);
  border-radius: var(--radius-sm);
  background: var(--bg-elevated);
  color: var(--text-secondary);
  font-size: var(--text-caption);
  font-family: var(--font-mono);
  word-break: break-all;
}

/* Footer */
.foot {
  display: flex;
  justify-content: space-between;
  gap: var(--space-4);
  padding-top: var(--space-3);
  border-top: 1px solid var(--border-default);
  color: var(--text-tertiary);
  font-size: var(--text-caption);
}

@media (max-width: 800px) {
  .about-grid { grid-template-columns: 1fr; }
  .brand { flex-direction: column; text-align: center; }
}
</style>
