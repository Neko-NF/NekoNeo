<script setup lang="ts">
import { computed, onMounted, reactive, ref, watch } from 'vue';
import NButton from '@/components/base/NButton.vue';
import NInput from '@/components/base/NInput.vue';
import NSelect from '@/components/base/NSelect.vue';
import NSwitch from '@/components/base/NSwitch.vue';
import { commands, getErrorCode, getErrorMessage } from '@/api/commands';
import { useDesktopNotifications } from '@/composables/useDesktopNotifications';
import { useConfigStore } from '@/stores/config';
import { useAuthStore } from '@/stores/auth';
import { useTheme } from '@/composables/useTheme';
import type {
  AppConfig,
  ConnectivityStatus,
  DeviceProfile,
  UpdateInfo,
  UpdateProgress,
} from '@/types';

const configStore = useConfigStore();
const authStore = useAuthStore();
const { notify } = useDesktopNotifications();
const { applyTheme } = useTheme();

const savingKey = ref<keyof AppConfig | null>(null);
const errorMessage = ref('');
const statusMessage = ref('');
const diagnostics = ref<Record<string, string>>({});
const fonts = ref<string[]>([]);
const diagnosticsLoading = ref(false);
const updateInfo = ref<UpdateInfo | null>(null);
const checkingUpdate = ref(false);
const testingConnectivity = ref(false);
const syncingMetadata = ref(false);
const downloadingUpdate = ref(false);
const installingUpdate = ref(false);
const connectivityStatus = ref<ConnectivityStatus | null>(null);
const deviceProfile = ref<DeviceProfile | null>(null);
const updateProgress = ref<UpdateProgress | null>(null);
const activeSection = ref('appearance');

// Auth state
const authUsername = ref('');
const authPassword = ref('');
const generatingKey = ref(false);

// Font autocomplete
const fontSuggestions = ref<string[]>([]);

const draft = reactive<AppConfig>({ ...configStore.config });

const sections = [
  { id: 'appearance', label: '外观' },
  { id: 'account', label: '账户' },
  { id: 'backend', label: '后端' },
  { id: 'capture', label: '采集' },
  { id: 'runtime', label: '运行时' },
  { id: 'updates', label: '更新' },
];

const serverModeOptions = [
  { label: 'Production', value: 'production' },
  { label: 'Local', value: 'local' },
];
const updateChannelOptions = [
  { label: 'Stable', value: 'stable' },
  { label: 'Beta', value: 'beta' },
];
const closeActionOptions = [
  { label: 'Ask', value: 'ask' },
  { label: 'Minimize', value: 'minimize' },
  { label: 'Exit', value: 'exit' },
];
const incognitoScopeOptions = [
  { label: 'Screenshots', value: 'screenshot' },
  { label: 'Titles', value: 'title' },
  { label: 'Both', value: 'both' },
];
const themeModeOptions = [
  { label: '深色', value: 'dark' },
  { label: '浅色', value: 'light' },
  { label: '跟随系统', value: 'system' },
];

const activeServerUrl = computed(() =>
  draft.serverMode === 'production' ? draft.serverUrlProd : draft.serverUrlLocal,
);

watch(
  () => configStore.config,
  (value) => {
    Object.assign(draft, JSON.parse(JSON.stringify(value)));
  },
  { deep: true, immediate: true },
);

function setStatus(m: string) {
  statusMessage.value = m;
  errorMessage.value = '';
}
function setError(e: unknown) {
  errorMessage.value = getErrorMessage(e);
  statusMessage.value = '';
}

async function persist<K extends keyof AppConfig>(key: K) {
  savingKey.value = key;
  try {
    await configStore.set(key, draft[key]);
    setStatus('已保存');
    if (['themeMode', 'seedColor', 'uiScale', 'uiFont'].includes(key)) {
      applyTheme(configStore.config);
    }
  } catch (error) {
    Object.assign(draft, JSON.parse(JSON.stringify(configStore.config)));
    setError(error);
  } finally {
    savingKey.value = null;
  }
}

async function persistDeviceKey() {
  const nextKey = draft.deviceKey.trim();
  const currentKey = configStore.config.deviceKey;
  if (!nextKey) {
    draft.deviceKey = '';
    await persist('deviceKey');
    return;
  }
  try {
    await commands.configValidateDeviceKey(nextKey, activeServerUrl.value);
  } catch (error) {
    const code = getErrorCode(error);
    if (code === 'TakeoverRequired') {
      const ok = window.confirm(`${getErrorMessage(error)}\n\n确认接管后将继续保存该密钥。`);
      if (!ok) {
        draft.deviceKey = currentKey;
        setStatus('已取消密钥更新');
        return;
      }
    } else if (['InvalidKey', 'KeyRevoked', 'DeviceNotFound', 'MissingServerConfig'].includes(code)) {
      setError(error);
      return;
    }
  }
  await persist('deviceKey');
}

async function updateToggle<K extends keyof AppConfig>(key: K, value: AppConfig[K]) {
  draft[key] = value;
  await persist(key);
}

function switchTab(id: string) {
  activeSection.value = id;
}

function onFontInput(value: string) {
  draft.uiFont = value;
  const q = value.trim();
  if (!q) {
    fontSuggestions.value = commonFonts().slice(0, 10);
    return;
  }
  const lower = q.toLowerCase();
  const all = [...new Set([...commonFonts(), ...fonts.value])];
  fontSuggestions.value = all
    .filter((f) => f.toLowerCase().includes(lower))
    .slice(0, 10);
}

function onFontFocus() {
  fontSuggestions.value = commonFonts().slice(0, 10);
}

function commonFonts(): string[] {
  return [
    'Segoe UI', 'Microsoft YaHei UI', '微软雅黑', 'SimSun', '宋体',
    'KaiTi', '楷体', 'FangSong', '仿宋', 'JetBrains Mono',
    'Cascadia Code', 'Consolas', 'Arial', 'Helvetica Neue',
  ];
}

function selectFont(font: string) {
  draft.uiFont = font;
  fontSuggestions.value = [];
  persist('uiFont');
}

async function loadDiagnostics() {
  diagnosticsLoading.value = true;
  try {
    const [health, systemFonts] = await Promise.all([
      commands.systemHealthCheck(),
      commands.systemGetFonts(),
    ]);
    diagnostics.value = health;
    fonts.value = systemFonts;
    setStatus('诊断信息已刷新');
  } catch (error) {
    setError(error);
  } finally {
    diagnosticsLoading.value = false;
  }
}

async function checkUpdate() {
  checkingUpdate.value = true;
  try {
    updateInfo.value = await commands.updateCheck(draft.updateChannel);
    setStatus('更新检查完成');
    if (updateInfo.value)
      await notify('info', '有可用更新', `${updateInfo.value.version} (${updateInfo.value.channel})`);
  } catch (error) {
    setError(error);
  } finally {
    checkingUpdate.value = false;
  }
}

async function testConnectivity() {
  testingConnectivity.value = true;
  try {
    connectivityStatus.value = await commands.configTestConnectivity(
      activeServerUrl.value,
      draft.deviceKey,
    );
    setStatus(connectivityStatus.value.reachable ? '连通性测试通过' : '连通性测试失败');
  } catch (error) {
    setError(error);
  } finally {
    testingConnectivity.value = false;
  }
}

async function syncDeviceMetadata() {
  syncingMetadata.value = true;
  try {
    deviceProfile.value = await commands.configSyncDeviceMetadata();
    setStatus('设备元信息已同步');
  } catch (error) {
    setError(error);
  } finally {
    syncingMetadata.value = false;
  }
}

async function downloadUpdate() {
  downloadingUpdate.value = true;
  updateProgress.value = {
    downloaded: 0,
    total: 0,
    percent: 0,
    assetName: updateInfo.value?.assetName ?? 'update',
  };
  try {
    await commands.updateDownload(draft.updateChannel);
    updateInfo.value = await commands.updateCheck(draft.updateChannel);
    setStatus('更新包已下载');
  } catch (error) {
    setError(error);
  } finally {
    downloadingUpdate.value = false;
  }
}

async function skipVersion() {
  if (!updateInfo.value) return;
  draft.skippedVersion = updateInfo.value.version;
  await persist('skippedVersion');
  updateInfo.value = null;
  setStatus(`已跳过 v${draft.skippedVersion}`);
}

async function installUpdate() {
  installingUpdate.value = true;
  try {
    await commands.updateInstall();
  } catch (error) {
    setError(error);
    installingUpdate.value = false;
  }
}

async function doAuth() {
  try {
    const credentials = { username: authUsername.value, password: authPassword.value };
    await authStore.doAuth(credentials);
    authUsername.value = '';
    authPassword.value = '';
    setStatus(authStore.mode === 'login' ? '登录成功' : '注册成功');
  } catch (error) {
    setError(error);
  }
}

async function doGenerateDeviceKey() {
  if (!authStore.token) return;
  generatingKey.value = true;
  try {
    const result = await authStore.generateDeviceKey(
      `NekoNeo-${draft.hostname || 'Desktop'}`,
    );
    draft.deviceKey = result.deviceKey;
    await persist('deviceKey');
    setStatus('设备密钥已自动生成并保存');
  } catch (error) {
    setError(error);
  } finally {
    generatingKey.value = false;
  }
}

function doLogout() {
  authStore.logout();
  setStatus('已退出登录');
}

onMounted(() => {
  void loadDiagnostics();
  void commands.systemGetDeviceProfile().then((p) => {
    deviceProfile.value = p;
  });
  void import('@tauri-apps/api/event').then(async ({ listen }) => {
    await listen<UpdateProgress>('update:progress', (e) => {
      updateProgress.value = e.payload;
    });
  });
});
</script>

<template>
  <div class="settings">
    <!-- Tab nav -->
    <nav class="settings-nav">
      <button
        v-for="sec in sections"
        :key="sec.id"
        class="settings-nav__btn"
        :class="{ 'settings-nav__btn--active': activeSection === sec.id }"
        @click="switchTab(sec.id)"
      >
        {{ sec.label }}
      </button>
      <div class="settings-nav__status">
        <span v-if="savingKey" class="settings-nav__saving">保存中...</span>
        <span v-else-if="statusMessage" class="settings-nav__ok">{{ statusMessage }}</span>
        <span v-else class="settings-nav__ready">就绪</span>
      </div>
      <span v-if="errorMessage" class="settings-nav__err">{{ errorMessage }}</span>
    </nav>

    <!-- Tab content -->
    <div class="settings-content">
      <!-- Appearance -->
      <section v-show="activeSection === 'appearance'" class="settings-card">
        <div class="settings-card__head">
          <h2>外观</h2>
          <p class="settings-card__desc">主题模式、颜色与字体</p>
        </div>

        <div class="field">
          <label class="field__label">主题模式</label>
          <NSelect
            v-model="draft.themeMode"
            :options="themeModeOptions"
            @change="persist('themeMode')"
          />
        </div>

        <div class="field">
          <label class="field__label">主题色</label>
          <div class="color-pick">
            <input
              type="color"
              :value="draft.seedColor"
              class="color-pick__inp"
              @change="
                draft.seedColor = ($event.target as HTMLInputElement).value;
                persist('seedColor');
              "
            />
            <span class="color-pick__val">{{ draft.seedColor }}</span>
          </div>
        </div>

        <div class="field-row">
          <div class="field">
            <label class="field__label">界面缩放 (%)</label>
            <NInput
              :model-value="draft.uiScale"
              type="number"
              :min="80"
              :max="200"
              :step="5"
              @update:model-value="draft.uiScale = Number($event || 100)"
              @blur="persist('uiScale')"
            />
          </div>
          <div class="field">
            <label class="field__label">界面字体</label>
            <div class="font-pick">
              <NInput
                :model-value="draft.uiFont"
                placeholder="Segoe UI"
                @update:model-value="onFontInput($event)"
                @focus="onFontFocus()"
                @blur="persist('uiFont')"
              />
              <ul v-if="fontSuggestions.length" class="font-pick__list">
                <li
                  v-for="f in fontSuggestions"
                  :key="f"
                  class="font-pick__item"
                  @mousedown.prevent="selectFont(f)"
                >
                  {{ f }}
                </li>
              </ul>
            </div>
          </div>
        </div>

        <p class="field-hint">主题色与字体即时生效；拖拽窗口边框或重新打开窗口以完全应用缩放。</p>
      </section>

      <!-- Account -->
      <section v-show="activeSection === 'account'" class="settings-card">
        <div class="settings-card__head">
          <h2>账户</h2>
          <p class="settings-card__desc">登录 NekoNeo 账户以自动生成设备密钥</p>
        </div>

        <!-- Logged in -->
        <div v-if="authStore.user" class="account-card">
          <div class="account-card__user">
            <div class="account-card__avatar">{{ authStore.user.username.charAt(0).toUpperCase() }}</div>
            <div>
              <strong class="account-card__name">{{ authStore.user.username }}</strong>
              <span class="account-card__id">ID: {{ authStore.user.id }}</span>
            </div>
            <NButton variant="ghost" size="sm" @click="doLogout">退出</NButton>
          </div>
          <div class="account-card__action">
            <div>
              <strong>生成设备密钥</strong>
              <span>使用当前账户创建新的设备密钥并自动填入配置</span>
            </div>
            <NButton variant="primary" size="sm" :loading="generatingKey" @click="doGenerateDeviceKey">
              生成密钥
            </NButton>
          </div>
        </div>

        <!-- Logged out -->
        <div v-else class="account-form">
          <div class="account-form__tabs">
            <button
              class="account-form__tab"
              :class="{ 'account-form__tab--active': authStore.mode === 'login' }"
              @click="authStore.mode = 'login'"
            >
              登录
            </button>
            <button
              class="account-form__tab"
              :class="{ 'account-form__tab--active': authStore.mode === 'register' }"
              @click="authStore.mode = 'register'"
            >
              注册
            </button>
          </div>
          <div class="field">
            <label class="field__label">用户名</label>
            <NInput v-model="authUsername" placeholder="输入用户名" />
          </div>
          <div class="field">
            <label class="field__label">密码</label>
            <NInput v-model="authPassword" type="password" placeholder="输入密码" />
          </div>
          <NButton variant="primary" :loading="authStore.loading" @click="doAuth">
            {{ authStore.mode === 'login' ? '登录' : '注册' }}
          </NButton>
        </div>

        <p class="field-hint">登录后可使用「生成密钥」自动获取设备密钥并填入后端设置。</p>
      </section>

      <!-- Backend -->
      <section v-show="activeSection === 'backend'" class="settings-card">
        <div class="settings-card__head">
          <h2>后端</h2>
          <p class="settings-card__desc">服务器地址与设备身份</p>
        </div>

        <div class="field">
          <label class="field__label">设备密钥</label>
          <NInput
            v-model="draft.deviceKey"
            placeholder="设备访问密钥"
            @blur="persistDeviceKey()"
          />
        </div>

        <div class="field-row">
          <div class="field">
            <label class="field__label">服务器模式</label>
            <NSelect
              v-model="draft.serverMode"
              :options="serverModeOptions"
              @change="persist('serverMode')"
            />
          </div>
          <div class="field">
            <label class="field__label">上报间隔 (秒)</label>
            <NInput
              :model-value="draft.reportInterval"
              type="number"
              :min="5"
              :step="1"
              @update:model-value="draft.reportInterval = Number($event || 0)"
              @blur="persist('reportInterval')"
            />
          </div>
        </div>

        <div class="field">
          <label class="field__label">生产环境 URL</label>
          <NInput
            v-model="draft.serverUrlProd"
            type="url"
            placeholder="https://api.example.com"
            @blur="persist('serverUrlProd')"
          />
        </div>
        <div class="field">
          <label class="field__label">本地环境 URL</label>
          <NInput
            v-model="draft.serverUrlLocal"
            type="url"
            placeholder="http://127.0.0.1:3000"
            @blur="persist('serverUrlLocal')"
          />
        </div>

        <div class="card-actions">
          <NButton
            variant="secondary"
            size="sm"
            :disabled="testingConnectivity"
            @click="testConnectivity()"
          >
            {{ testingConnectivity ? '检测中...' : '连通性测试' }}
          </NButton>
          <NButton
            variant="ghost"
            size="sm"
            :disabled="syncingMetadata"
            @click="syncDeviceMetadata()"
          >
            {{ syncingMetadata ? '同步中...' : '同步设备元信息' }}
          </NButton>
        </div>

        <div v-if="connectivityStatus" class="infobox">
          <strong>连通性</strong>
          <p>{{ connectivityStatus.url }} — {{ connectivityStatus.reachable ? '可达' : '不可达' }}</p>
          <p class="infobox__detail">{{ connectivityStatus.detail }}</p>
        </div>
        <div v-if="deviceProfile" class="infobox">
          <strong>设备信息</strong>
          <p>{{ deviceProfile.hostname }} · {{ deviceProfile.osFriendlyName }}</p>
          <p>{{ deviceProfile.cpuModel }} · {{ deviceProfile.cpuCores }} 核 · v{{ deviceProfile.appVersion }}</p>
        </div>
      </section>

      <!-- Capture -->
      <section v-show="activeSection === 'capture'" class="settings-card">
        <div class="settings-card__head">
          <h2>采集</h2>
          <p class="settings-card__desc">截图与隐私策略</p>
        </div>

        <div class="switch-list">
          <div class="switch-row">
            <div>
              <strong>启用截图</strong>
              <span>允许服务在周期上报中采集截图</span>
            </div>
            <NSwitch
              :model-value="draft.enableScreenshot"
              @update:model-value="updateToggle('enableScreenshot', $event)"
            />
          </div>
          <div class="switch-row">
            <div>
              <strong>同步截图间隔</strong>
              <span>截图间隔跟随主上报周期</span>
            </div>
            <NSwitch
              :model-value="draft.syncScreenshotInterval"
              @update:model-value="updateToggle('syncScreenshotInterval', $event)"
            />
          </div>
          <div class="switch-row">
            <div>
              <strong>隐身模式</strong>
              <span>上报时脱敏已标记应用的标题或截图</span>
            </div>
            <NSwitch
              :model-value="draft.enableIncognito"
              @update:model-value="updateToggle('enableIncognito', $event)"
            />
          </div>
          <div class="switch-row">
            <div>
              <strong>模糊所有截图</strong>
              <span>安全调试用，强制对所有截图做隐私模糊</span>
            </div>
            <NSwitch
              :model-value="draft.blurAllScreenshots"
              @update:model-value="updateToggle('blurAllScreenshots', $event)"
            />
          </div>
        </div>

        <div class="field-row">
          <div class="field">
            <label class="field__label">截图间隔 (秒)</label>
            <NInput
              :model-value="draft.screenshotInterval"
              type="number"
              :min="5"
              :step="1"
              @update:model-value="draft.screenshotInterval = Number($event || 0)"
              @blur="persist('screenshotInterval')"
            />
          </div>
          <div class="field">
            <label class="field__label">隐身范围</label>
            <NSelect
              v-model="draft.incognitoScope"
              :options="incognitoScopeOptions"
              @change="persist('incognitoScope')"
            />
          </div>
        </div>
      </section>

      <!-- Runtime -->
      <section v-show="activeSection === 'runtime'" class="settings-card">
        <div class="settings-card__head">
          <h2>运行时</h2>
          <p class="settings-card__desc">启动行为与故障恢复</p>
        </div>

        <div class="switch-list">
          <div class="switch-row">
            <div>
              <strong>自动启动服务</strong>
              <span>应用启动时自动开始上报循环</span>
            </div>
            <NSwitch
              :model-value="draft.enableAutoServiceStart"
              @update:model-value="updateToggle('enableAutoServiceStart', $event)"
            />
          </div>
          <div class="switch-row">
            <div>
              <strong>自动重启</strong>
              <span>遇到临时后端故障时看门狗自动尝试恢复</span>
            </div>
            <NSwitch
              :model-value="draft.enableAutoRestart"
              @update:model-value="updateToggle('enableAutoRestart', $event)"
            />
          </div>
          <div class="switch-row">
            <div>
              <strong>系统通知</strong>
              <span>显示更新和服务状态相关的本地通知</span>
            </div>
            <NSwitch
              :model-value="draft.enableNotification"
              @update:model-value="updateToggle('enableNotification', $event)"
            />
          </div>
          <div class="switch-row">
            <div>
              <strong>勿扰模式</strong>
              <span>降低本地提示频率，后端活动不受影响</span>
            </div>
            <NSwitch
              :model-value="draft.doNotDisturb"
              @update:model-value="updateToggle('doNotDisturb', $event)"
            />
          </div>
        </div>

        <div class="field-row field-row--4">
          <div class="field">
            <label class="field__label">最大重启次数</label>
            <NInput
              :model-value="draft.maxRestarts"
              type="number"
              :min="0"
              :step="1"
              @update:model-value="draft.maxRestarts = Number($event || 0)"
              @blur="persist('maxRestarts')"
            />
          </div>
          <div class="field">
            <label class="field__label">重启窗口 (秒)</label>
            <NInput
              :model-value="draft.restartIntervalSec"
              type="number"
              :min="5"
              :step="1"
              @update:model-value="draft.restartIntervalSec = Number($event || 0)"
              @blur="persist('restartIntervalSec')"
            />
          </div>
          <div class="field">
            <label class="field__label">看门狗超时 (秒)</label>
            <NInput
              :model-value="draft.watchdogTimeoutSec"
              type="number"
              :min="10"
              :step="1"
              @update:model-value="draft.watchdogTimeoutSec = Number($event || 0)"
              @blur="persist('watchdogTimeoutSec')"
            />
          </div>
          <div class="field">
            <label class="field__label">关闭行为</label>
            <NSelect
              v-model="draft.closeAction"
              :options="closeActionOptions"
              @change="persist('closeAction')"
            />
          </div>
        </div>
      </section>

      <!-- Updates -->
      <section v-show="activeSection === 'updates'" class="settings-card">
        <div class="settings-card__head">
          <h2>更新</h2>
          <p class="settings-card__desc">版本检查与诊断</p>
        </div>

        <div class="card-actions">
          <NButton
            variant="secondary"
            size="sm"
            :disabled="checkingUpdate"
            @click="checkUpdate()"
          >
            {{ checkingUpdate ? '检测中...' : '检查更新' }}
          </NButton>
          <NButton
            variant="primary"
            size="sm"
            :disabled="!updateInfo || downloadingUpdate"
            @click="downloadUpdate()"
          >
            {{ downloadingUpdate ? '下载中...' : '下载更新' }}
          </NButton>
          <NButton
            variant="ghost"
            size="sm"
            :disabled="!updateInfo?.downloaded || installingUpdate"
            @click="installUpdate()"
          >
            {{ installingUpdate ? '启动中...' : '安装' }}
          </NButton>
          <NButton
            variant="ghost"
            size="sm"
            :disabled="diagnosticsLoading"
            @click="loadDiagnostics()"
          >
            {{ diagnosticsLoading ? '刷新中...' : '诊断' }}
          </NButton>
        </div>

        <div class="switch-list">
          <div class="switch-row">
            <div>
              <strong>自动检查更新</strong>
              <span>启动时请求更新元数据</span>
            </div>
            <NSwitch
              :model-value="draft.autoCheckUpdate"
              @update:model-value="updateToggle('autoCheckUpdate', $event)"
            />
          </div>
          <div class="switch-row">
            <div>
              <strong>自动下载</strong>
              <span>后台预下载更新包</span>
            </div>
            <NSwitch
              :model-value="draft.autoDownload"
              @update:model-value="updateToggle('autoDownload', $event)"
            />
          </div>
        </div>

        <div class="field-row">
          <div class="field">
            <label class="field__label">更新通道</label>
            <NSelect
              v-model="draft.updateChannel"
              :options="updateChannelOptions"
              @change="persist('updateChannel')"
            />
          </div>
          <div class="field">
            <label class="field__label">跳过版本</label>
            <NInput
              v-model="draft.skippedVersion"
              placeholder="2.0.0-beta.2"
              @blur="persist('skippedVersion')"
            />
          </div>
        </div>

        <div v-if="updateInfo" class="infobox">
          <strong>
            最新候选
            <span v-if="updateInfo.mandatory" class="infobox__mandatory">强制更新</span>
            <span v-else-if="updateInfo.downloaded" class="infobox__downloaded">已下载</span>
          </strong>
          <p>{{ updateInfo.version }} · {{ updateInfo.channel }} · {{ updateInfo.assetName || '暂无桌面包' }}</p>
          <p class="infobox__detail">{{ updateInfo.releaseNotes }}</p>
          <div v-if="!updateInfo.downloaded" class="infobox__actions">
            <NButton variant="ghost" size="sm" :disabled="saving" @click="skipVersion()">
              跳过此版本
            </NButton>
          </div>
        </div>
        <div v-if="updateProgress" class="infobox">
          <strong>下载进度</strong>
          <p>{{ updateProgress.assetName }} — {{ updateProgress.percent }}%</p>
          <p class="infobox__detail">{{ updateProgress.downloaded }} / {{ updateProgress.total ?? 0 }} bytes</p>
        </div>

        <div class="diag-grid">
          <div class="infobox">
            <strong>健康检查</strong>
            <p v-for="(v, k) in diagnostics" :key="k">{{ k }}: {{ v }}</p>
          </div>
          <div class="infobox">
            <strong>检测到的字体 ({{ fonts.length }})</strong>
            <p v-for="f in fonts.slice(0, 8)" :key="f">{{ f }}</p>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.settings {
  display: grid;
  gap: var(--space-5);
}

/* ── Tab nav ────────────────────────────────────────────────────────── */
.settings-nav {
  display: flex;
  align-items: center;
  gap: 0;
  border-bottom: 1px solid var(--border-default);
  overflow-x: auto;
}

.settings-nav__btn {
  padding: var(--space-3) var(--space-4);
  border: none;
  border-bottom: 2px solid transparent;
  margin-bottom: -1px;
  background: transparent;
  color: var(--text-secondary);
  font-size: var(--text-sm);
  font-weight: var(--fw-medium);
  cursor: pointer;
  white-space: nowrap;
  transition: color var(--duration-fast) var(--ease-standard),
              border-color var(--duration-fast) var(--ease-standard);
}

.settings-nav__btn:hover {
  color: var(--text-primary);
}

.settings-nav__btn--active {
  color: var(--color-primary);
  border-bottom-color: var(--color-primary);
}

.settings-nav__status {
  margin-left: auto;
  padding: 0 var(--space-2);
  font-size: var(--text-caption);
  white-space: nowrap;
}

.settings-nav__saving { color: var(--color-warning); }
.settings-nav__ok { color: var(--color-success); }
.settings-nav__ready { color: var(--text-tertiary); }

.settings-nav__err {
  padding: 0 var(--space-3);
  color: var(--color-danger);
  font-size: var(--text-caption);
  max-width: 260px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* ── Content ────────────────────────────────────────────────────────── */
.settings-content {
  min-height: 400px;
}

.settings-card {
  padding: var(--space-5);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-lg);
  background: var(--bg-surface);
  display: grid;
  gap: var(--space-4);
}

.settings-card__head h2 {
  margin: 0;
  font-size: var(--text-xl);
  font-weight: var(--fw-semibold);
}

.settings-card__desc {
  margin: 2px 0 0;
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.card-actions {
  display: flex;
  flex-wrap: wrap;
  gap: var(--space-2);
}

/* ── Fields ─────────────────────────────────────────────────────────── */
.field-row {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-3);
}

.field-row--4 {
  grid-template-columns: repeat(4, minmax(0, 1fr));
}

.field {
  display: grid;
  gap: var(--space-2);
}

.field__label {
  color: var(--text-secondary);
  font-size: var(--text-sm);
  font-weight: var(--fw-medium);
}

.field-hint {
  margin: 0;
  color: var(--text-tertiary);
  font-size: var(--text-caption);
}

/* ── Color picker ───────────────────────────────────────────────────── */
.color-pick {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.color-pick__inp {
  width: 40px;
  height: 40px;
  padding: 2px;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-input);
  cursor: pointer;
  flex-shrink: 0;
  transition: border-color var(--duration-fast) var(--ease-standard);
}

.color-pick__inp:hover {
  border-color: var(--border-strong);
}

.color-pick__inp::-webkit-color-swatch-wrapper { padding: 0; }
.color-pick__inp::-webkit-color-swatch { border: none; border-radius: var(--radius-sm); }

.color-pick__val {
  color: var(--text-tertiary);
  font-size: var(--text-sm);
  font-family: var(--font-mono);
}

/* ── Font autocomplete ──────────────────────────────────────────────── */
.font-pick {
  position: relative;
}

.font-pick__list {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  z-index: 10;
  margin: var(--space-1) 0 0;
  padding: var(--space-1) 0;
  list-style: none;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  max-height: 200px;
  overflow-y: auto;
}

.font-pick__item {
  padding: var(--space-2) var(--space-3);
  font-size: var(--text-sm);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-standard);
}

.font-pick__item:hover {
  background: var(--bg-hover);
}

/* ── Switch list ────────────────────────────────────────────────────── */
.switch-list { display: grid; gap: var(--space-1); }

.switch-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-4);
  padding: var(--space-3) 0;
  border-bottom: 1px solid var(--border-default);
}

.switch-row:last-child { padding-bottom: 0; border-bottom: 0; }

.switch-row strong { display: block; font-weight: var(--fw-medium); }

.switch-row span {
  display: block;
  margin-top: 1px;
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

/* ── Infobox ────────────────────────────────────────────────────────── */
.infobox {
  display: grid;
  gap: var(--space-1);
  padding: var(--space-3) var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
}

.infobox strong { font-weight: var(--fw-medium); }

.infobox p {
  margin: 0;
  color: var(--text-secondary);
  font-size: var(--text-sm);
}

.infobox__mandatory {
  padding: 1px var(--space-2);
  border-radius: var(--radius-full);
  background: var(--color-danger-muted);
  color: var(--color-danger);
  font-size: var(--text-caption);
  font-weight: var(--fw-medium);
}

.infobox__downloaded {
  padding: 1px var(--space-2);
  border-radius: var(--radius-full);
  background: var(--color-success-muted);
  color: var(--color-success);
  font-size: var(--text-caption);
  font-weight: var(--fw-medium);
}

.infobox__actions {
  margin-top: var(--space-2);
}

.infobox__detail {
  color: var(--text-tertiary) !important;
  font-size: var(--text-caption) !important;
}

.diag-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: var(--space-3);
}

/* ── Account ────────────────────────────────────────────────────────── */
.account-card {
  padding: var(--space-4);
  border: 1px solid var(--border-default);
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  display: grid;
  gap: var(--space-4);
}

.account-card__user {
  display: flex;
  align-items: center;
  gap: var(--space-3);
}

.account-card__avatar {
  width: 40px;
  height: 40px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: var(--radius-full);
  background: var(--color-primary-muted);
  color: var(--color-primary);
  font-size: var(--text-lg);
  font-weight: var(--fw-semibold);
  flex-shrink: 0;
}

.account-card__name {
  display: block;
  font-size: var(--text-base);
  font-weight: var(--fw-semibold);
}

.account-card__id {
  display: block;
  margin-top: 1px;
  color: var(--text-tertiary);
  font-size: var(--text-caption);
}

.account-card__user .n-btn {
  margin-left: auto;
}

.account-card__action {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: var(--space-3);
  padding-top: var(--space-3);
  border-top: 1px solid var(--border-default);
}

.account-card__action strong {
  display: block;
  font-size: var(--text-sm);
  font-weight: var(--fw-medium);
}

.account-card__action span {
  display: block;
  margin-top: 1px;
  color: var(--text-secondary);
  font-size: var(--text-caption);
}

.account-form {
  display: grid;
  gap: var(--space-4);
}

.account-form__tabs {
  display: flex;
  border: 1px solid var(--border-default);
  border-radius: var(--radius-sm);
  overflow: hidden;
}

.account-form__tab {
  flex: 1;
  padding: var(--space-2) var(--space-4);
  border: none;
  background: transparent;
  color: var(--text-secondary);
  font-size: var(--text-sm);
  font-weight: var(--fw-medium);
  cursor: pointer;
  transition: background var(--duration-fast) var(--ease-standard),
              color var(--duration-fast) var(--ease-standard);
}

.account-form__tab--active {
  background: var(--color-primary-muted);
  color: var(--color-primary);
}

/* ── Responsive ─────────────────────────────────────────────────────── */
@media (max-width: 1200px) {
  .field-row,
  .diag-grid {
    grid-template-columns: 1fr;
  }
  .field-row--4 {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 720px) {
  .field-row--4 {
    grid-template-columns: 1fr;
  }
  .settings-nav__status {
    margin-left: 0;
  }
}
</style>
