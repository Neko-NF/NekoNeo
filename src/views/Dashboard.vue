<script setup lang="ts">
import { computed, onMounted, onUnmounted, ref, watch } from 'vue';
import NButton from '@/components/base/NButton.vue';
import MetricRow from '@/components/widgets/MetricRow.vue';
import QuickToggle from '@/components/widgets/QuickToggle.vue';
import { useLatestScreenshot } from '@/composables/useLatestScreenshot';
import { useConfigStore } from '@/stores/config';
import { useMetricsStore } from '@/stores/metrics';
import { useServiceStore } from '@/stores/service';

const configStore = useConfigStore();
const serviceStore = useServiceStore();
const metricsStore = useMetricsStore();
const {
  latest: latestScreenshot,
  loading: latestScreenshotLoading,
  capturing: screenshotCapturing,
  error: latestScreenshotError,
  previewSrc: latestScreenshotPreviewSrc,
  loadLatest,
  captureNow,
} = useLatestScreenshot();

const now = ref(Date.now());
let timer: ReturnType<typeof setInterval> | null = null;

onMounted(() => {
  void loadLatest();
  timer = setInterval(() => { now.value = Date.now(); }, 1000);
});

onUnmounted(() => {
  if (timer) clearInterval(timer);
});

watch(
  () => serviceStore.lastResult?.timestamp ?? null,
  () => {
    if (serviceStore.lastResult?.hasScreenshot) void loadLatest();
  },
);

function formatBytes(bps: number): string {
  if (bps >= 1_048_576) return `${(bps / 1_048_576).toFixed(1)} MB/s`;
  if (bps >= 1024) return `${(bps / 1024).toFixed(1)} KB/s`;
  return `${bps} B/s`;
}

const toggles = computed(() => [
  { key: 'enableScreenshot' as const,  label: '截图上报', desc: '周期采集桌面截图' },
  { key: 'enableIncognito' as const,   label: '隐身模式', desc: '脱敏敏感窗口与内容' },
  { key: 'enableNotification' as const, label: '系统通知', desc: '服务状态与更新提醒' },
  { key: 'doNotDisturb' as const,      label: '勿扰模式', desc: '暂停非紧急本地通知' },
]);

const secondsSinceLastTick = computed(() => {
  if (!serviceStore.lastResult?.timestamp) return null;
  const diff = (now.value - new Date(serviceStore.lastResult.timestamp).getTime()) / 1000;
  return Math.round(diff);
});

const lastTickText = computed(() => {
  const s = secondsSinceLastTick.value;
  if (s == null) return '尚未上报';
  if (s < 5) return '刚刚';
  if (s < 60) return `${s} 秒前`;
  if (s < 3600) return `${Math.floor(s / 60)} 分钟前`;
  return `${Math.floor(s / 3600)} 小时前`;
});

const screenshotCapturedAt = computed(() => {
  if (!latestScreenshot.value) return '未采集';
  return new Date(latestScreenshot.value.capturedAt).toLocaleString('zh-CN', { hour12: false });
});

async function handleToggle(key: keyof typeof configStore.config, value: boolean) {
  await configStore.set(key as never, value as never);
}
</script>

<template>
  <div class="dashboard">
    <!-- Hero status bar — like Task Manager's compact header -->
    <div class="hero" :class="{ 'hero--live': serviceStore.running }">
      <div class="hero__left">
        <span class="hero__dot" :class="{ 'hero__dot--live': serviceStore.running }" />
        <div>
          <span class="hero__state">{{ serviceStore.running ? '服务运行中' : '服务已停止' }}</span>
          <span class="hero__sub">
            {{ serviceStore.running ? `最近上报: ${lastTickText}` : '点击按钮启动服务' }}
          </span>
        </div>
      </div>

      <div class="hero__stats">
        <span class="hero__stat">
          <span class="hero__stat-label">前台</span>
          <span class="hero__stat-val">{{ serviceStore.lastResult?.appName ?? '—' }}</span>
        </span>
        <span class="hero__stat">
          <span class="hero__stat-label">状态</span>
          <span class="hero__stat-val">{{ serviceStore.lastResult?.userStatus === 'away' ? '离开' : '在线' }}</span>
        </span>
        <span class="hero__stat">
          <span class="hero__stat-label">电量</span>
          <span class="hero__stat-val">{{ serviceStore.lastResult?.hasBattery ? `${serviceStore.lastResult.batteryLevel}%` : '—' }}</span>
        </span>
      </div>

      <div class="hero__right">
        <NButton
          v-if="!serviceStore.running"
          variant="primary"
          size="sm"
          :disabled="serviceStore.loading"
          @click="serviceStore.start()"
        >
          {{ serviceStore.loading ? '启动中...' : '启动服务' }}
        </NButton>
        <NButton
          v-else
          variant="secondary"
          size="sm"
          @click="serviceStore.stop()"
        >
          停止
        </NButton>
      </div>
    </div>

    <!-- Two-column content: Toggles | Metrics -->
    <div class="dashboard__grid">
      <!-- Quick controls -->
      <section class="card">
        <div class="card__hd">
          <div>
            <h2 class="card__title">快捷控制</h2>
            <p class="card__sub">即时生效</p>
          </div>
        </div>
        <div class="toggles">
          <QuickToggle
            v-for="t in toggles"
            :key="t.key"
            :label="t.label"
            :description="t.desc"
            :model-value="configStore.config[t.key]"
            @update:model-value="handleToggle(t.key, $event)"
          />
        </div>
      </section>

      <!-- System metrics -->
      <section class="card">
        <div class="card__hd">
          <div>
            <h2 class="card__title">系统指标</h2>
            <p class="card__sub">每 {{ configStore.config.reportInterval }}s 刷新</p>
          </div>
        </div>
        <div class="metrics">
          <MetricRow
            label="CPU 使用率"
            :value="`${(metricsStore.metrics?.cpuPct ?? 0).toFixed(1)}%`"
            :percent="metricsStore.metrics?.cpuPct ?? 0"
          />
          <MetricRow
            label="内存占用"
            :value="`${(metricsStore.metrics?.memPct ?? 0).toFixed(1)}%`"
            :percent="metricsStore.metrics?.memPct ?? 0"
          />
          <MetricRow label="网络下行" :value="formatBytes(metricsStore.metrics?.netDownBps ?? 0)" />
          <MetricRow label="网络延迟" :value="`${metricsStore.metrics?.networkLatency ?? -1} ms`" />
        </div>
        <div class="card__footer">
          <span>{{ metricsStore.metrics?.hostname ?? '—' }}</span>
          <span>{{ metricsStore.metrics?.cpuModel ?? '—' }}</span>
        </div>
      </section>
    </div>

    <!-- Screenshot section -->
    <section class="card">
      <div class="card__hd card__hd--actions">
        <div>
          <h2 class="card__title">截图中心</h2>
          <p class="card__sub">最近采集与预览</p>
        </div>
        <div class="card__hd-btns">
          <NButton variant="ghost" size="sm" :disabled="latestScreenshotLoading" @click="loadLatest()">
            刷新
          </NButton>
          <NButton variant="primary" size="sm" :disabled="screenshotCapturing" @click="captureNow()">
            {{ screenshotCapturing ? '采集中...' : '立即截图' }}
          </NButton>
        </div>
      </div>

      <div class="capture">
        <div class="capture__preview">
          <img
            v-if="latestScreenshotPreviewSrc"
            :src="latestScreenshotPreviewSrc"
            alt="最近截图"
            class="capture__img"
          />
          <div v-else class="capture__empty">
            <span>暂无截图</span>
            <small>点击「立即截图」或启动服务等待周期采集</small>
          </div>
        </div>
        <div class="capture__meta-row">
          <span>{{ screenshotCapturedAt }}</span>
          <span>·</span>
          <span :class="{ 'capture__blurred': latestScreenshot?.blurred }">
            {{ latestScreenshot?.blurred ? '已模糊' : '原始' }}
          </span>
          <span>·</span>
          <span>{{ configStore.config.enableScreenshot ? '截图上报开' : '截图上报关' }}</span>
        </div>
      </div>
      <p v-if="latestScreenshotError" class="capture__err">{{ latestScreenshotError }}</p>
    </section>
  </div>
</template>

<style scoped>
.dashboard {
  display: grid;
  gap: var(--space-5);
}

/* ── Hero ──────────────────────────────────────────────────────────── */

.hero {
  display: flex;
  align-items: center;
  gap: var(--space-5);
  padding: var(--space-4) var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.hero--live {
  border-color: var(--color-primary-muted);
}

.hero__left {
  display: flex;
  align-items: center;
  gap: var(--space-3);
  flex: 1;
}

.hero__dot {
  width: 8px;
  height: 8px;
  border-radius: var(--radius-full);
  background: var(--border-strong);
  flex-shrink: 0;
}

.hero__dot--live {
  background: var(--color-success);
  box-shadow: 0 0 4px var(--color-success-muted);
}

.hero__state {
  display: block;
  font-size: var(--text-base);
  font-weight: var(--fw-semibold);
}

.hero__sub {
  display: block;
  margin-top: 1px;
  color: var(--text-tertiary);
  font-size: var(--text-caption);
}

.hero__stats {
  display: flex;
  gap: var(--space-4);
}

.hero__stat {
  display: grid;
  gap: 1px;
  text-align: center;
  min-width: 64px;
}

.hero__stat-label {
  color: var(--text-tertiary);
  font-size: var(--text-caption);
}

.hero__stat-val {
  font-size: var(--text-sm);
  font-weight: var(--fw-semibold);
  font-family: var(--font-mono);
}

.hero__right {
  flex-shrink: 0;
}

/* ── Two-column grid ────────────────────────────────────────────────── */

.dashboard__grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-5);
  align-items: stretch;
}

/* ── Card ───────────────────────────────────────────────────────────── */

.card {
  display: grid;
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
}

.card__hd {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: var(--space-4);
}

.card__hd--actions {
  align-items: center;
}

.card__hd-btns {
  display: flex;
  gap: var(--space-2);
  flex-shrink: 0;
}

.card__title {
  margin: 0;
  font-size: var(--text-lg);
  font-weight: var(--fw-semibold);
}

.card__sub {
  margin: 2px 0 0;
  color: var(--text-tertiary);
  font-size: var(--text-caption);
}

.card__footer {
  margin-top: var(--space-4);
  padding-top: var(--space-3);
  border-top: 1px solid var(--border-default);
  display: grid;
  gap: var(--space-1);
  color: var(--text-tertiary);
  font-size: var(--text-caption);
  font-family: var(--font-mono);
}

/* ── Toggles ────────────────────────────────────────────────────────── */

.toggles {
  display: grid;
  gap: var(--space-2);
}

/* ── Metrics ────────────────────────────────────────────────────────── */

.metrics {
  display: grid;
  gap: var(--space-1);
}

/* ── Capture ────────────────────────────────────────────────────────── */

.capture {
  display: grid;
  gap: var(--space-3);
}

.capture__preview {
  min-height: 200px;
  max-height: 400px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  overflow: hidden;
}

.capture__img {
  width: 100%;
  height: 100%;
  object-fit: contain;
}

.capture__empty {
  display: grid;
  gap: var(--space-1);
  justify-items: center;
  padding: var(--space-6);
  color: var(--text-tertiary);
  font-size: var(--text-base);
}

.capture__empty small {
  font-size: var(--text-caption);
  color: var(--text-tertiary);
}

.capture__meta-row {
  display: flex;
  justify-content: center;
  gap: var(--space-2);
  color: var(--text-tertiary);
  font-size: var(--text-caption);
  font-family: var(--font-mono);
}

.capture__blurred {
  color: var(--color-warning);
}

.capture__err {
  margin: 0;
  color: var(--color-danger);
  font-size: var(--text-sm);
}

/* ── Responsive ─────────────────────────────────────────────────────── */
@media (min-width: 1400px) {
  .dashboard__grid {
    grid-template-columns: 1fr 1fr 1fr;
  }
  .capture {
    grid-template-columns: 1fr 1fr;
  }
}

@media (max-width: 900px) {
  .hero {
    flex-wrap: wrap;
  }
  .hero__stats {
    order: 3;
    width: 100%;
    justify-content: flex-start;
  }
  .dashboard__grid {
    grid-template-columns: 1fr;
  }
  .capture {
    grid-template-columns: 1fr;
  }
}
</style>
