<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";
import { register, unregisterAll, isRegistered } from "@tauri-apps/plugin-global-shortcut";

import { useSoundSettings } from "@/composables/useSoundSettings";
import type { Monitor } from "@/types";

import MonitorSelectCard from "@/components/MonitorSelectCard.vue";
import CaptureScaleCard from "@/components/CaptureScaleCard.vue";
import SavePathCard from "@/components/SavePathCard.vue";
import HotkeyCard from "@/components/HotkeyCard.vue";
import SettingsDialog from "@/components/SettingsDialog.vue";
import BaseToast from "@/components/base/BaseToast.vue";
import { useToast } from "@/composables/useToast";
import { useVersion } from "./composables/useVersion";
import { useTheme } from "./composables/useTheme";

const SAVE_PATH_KEY = "save_path";
const HOTKEY_KEY_KEY = "hotkey_key";
const SELECTED_MONITOR_KEY = "selected_monitor";

const loading = ref(false);
const monitors = ref<Monitor[]>([]);
const selectedMonitor = ref<number | null>(null);
const savePath = ref<string>("");
const selectedScale = ref<number>(2.0);

const hotkeyKey = ref("F12");
const currentShortcut = ref<string>("");
const showSettings = ref(false);

const toast = useToast("top-right");
const { version } = useVersion();
const { isDark, toggleTheme } = useTheme();

const { playCameraSound } = useSoundSettings();

const generateTimestamp = () => {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, "0");
  const day = String(now.getDate()).padStart(2, "0");
  const hours = String(now.getHours()).padStart(2, "0");
  const minutes = String(now.getMinutes()).padStart(2, "0");
  const seconds = String(now.getSeconds()).padStart(2, "0");
  return `${day}_${month}_${year}__${hours}h_${minutes}m_${seconds}s_`;
};

const loadMonitors = async () => {
  try {
    const monitorList = await invoke<Monitor[]>("list_monitors");
    monitors.value = monitorList;

    const savedMonitorId = localStorage.getItem(SELECTED_MONITOR_KEY);
    let targetMonitorId = null;

    if (savedMonitorId) {
      const parsedId = parseInt(savedMonitorId);
      if (monitorList.some((m) => m.id === parsedId)) {
        targetMonitorId = parsedId;
      }
    }

    if (targetMonitorId === null) {
      const primary = monitorList.find((m) => m.is_primary);
      targetMonitorId = primary ? primary.id : monitorList.length > 0 ? monitorList[0].id : null;
    }

    selectedMonitor.value = targetMonitorId;
  } catch (e: any) {
    toast.show({ title: "Monitor Error", description: e?.toString(), type: "error" });
  }
};

const registerShortcut = async () => {
  const newShortcut = hotkeyKey.value.toUpperCase();

  if (currentShortcut.value === newShortcut && (await isRegistered(newShortcut))) {
    toast.show({ title: "Shortcut Active", description: `key ${newShortcut} is already set.`, type: "info" });
    return;
  }

  if (currentShortcut.value) {
    try {
      await unregisterAll();
    } catch {}
  }

  if (!newShortcut) {
    toast.show({ title: "Invalid Shortcut", type: "warning" });
    return;
  }

  try {
    await register(newShortcut, capture);
    currentShortcut.value = newShortcut;
    localStorage.setItem(HOTKEY_KEY_KEY, hotkeyKey.value);
    toast.show({ title: "Shortcut Saved", description: `New hotkey: ${newShortcut}`, type: "success", duration: 3000 });
  } catch (e: any) {
    console.error(e);
    toast.show({ title: "Shortcut Error", description: "Failed to register hotkey. Check if it's used by system.", type: "error" });
  }
};

const capture = async () => {
  if (loading.value) return;
  if (!selectedMonitor.value) {
    toast.show({ title: "No Monitor", description: "Select a monitor.", type: "warning" });
    return;
  }
  if (!savePath.value) {
    toast.show({ title: "No Path", description: "Select a destination folder.", type: "warning" });
    return;
  }

  loading.value = true;
  try {
    playCameraSound();

    const scaleFormatted = parseFloat(selectedScale.value.toString()).toFixed(1).replace(".", "p");
    const filename = `Sharpshot_x${scaleFormatted}_${generateTimestamp()}.jpg`;
    const filePath = await join(savePath.value, filename);

    await invoke<string>("capture_and_scale", {
      monitorId: selectedMonitor.value,
      scale: parseFloat(parseFloat(selectedScale.value.toString()).toFixed(1)),
      savePath: filePath,
    });

    toast.show({ title: "Capture Saved", description: "Image saved successfully.", type: "success" });
  } catch (e: any) {
    toast.show({ title: "Capture Error", description: e?.toString(), type: "error" });
  } finally {
    loading.value = false;
  }
};

onMounted(() => {
  loadMonitors();

  const savedPath = localStorage.getItem(SAVE_PATH_KEY);
  if (savedPath) savePath.value = savedPath;

  const savedKey = localStorage.getItem(HOTKEY_KEY_KEY);
  if (savedKey) hotkeyKey.value = savedKey;

  registerShortcut();
});

watch(savePath, (newVal) => {
  if (newVal) localStorage.setItem(SAVE_PATH_KEY, newVal);
});

watch(selectedMonitor, (newId) => {
  if (newId !== null) localStorage.setItem(SELECTED_MONITOR_KEY, newId.toString());
});

onUnmounted(async () => {
  await unregisterAll();
});
</script>

<template>
  <div class="app-container flex flex-col p-8 gap-[20px]">
    <header class="top-bar flex items-center justify-between px-[5px]">
      <h1 class="app-title flex-1">
        Sharpshot <span class="version">{{ version }}</span>
      </h1>
      <div class="flex items-center gap-5">
        <button type="button" class="theme-btn neu-btn flex items-center justify-center" @click="toggleTheme" aria-label="Toggle Theme">
          <Icon :icon="isDark ? 'material-symbols:dark-mode' : 'material-symbols:light-mode-outline-rounded'" style="font-size: 25px" />
        </button>
        <button type="button" class="hamburger-btn neu-btn flex items-center justify-center" @click="showSettings = true" aria-label="Open Settings">
          <Icon icon="material-symbols:menu" style="font-size: 25px" />
        </button>
      </div>
    </header>

    <main class="content-area flex flex-col flex-1 gap-8 pb-8 px-[15px] pt-[15px] overflow-auto">
      <div class="custom-grid gap-8">
        <div class="grid-item item-monitors">
          <MonitorSelectCard :monitors="monitors" v-model:selectedMonitorId="selectedMonitor" :loading="loading" class="h-full" />
        </div>
        
        <div class="grid-item item-scale">
          <CaptureScaleCard v-model:selectedScale="selectedScale" :loading="loading" class="h-full" />
        </div>

        <div class="grid-item item-path">
          <SavePathCard v-model:savePath="savePath" :loading="loading" class="h-full" />
        </div>
        
        <div class="grid-item item-hotkey">
          <HotkeyCard v-model:hotkeyKey="hotkeyKey" :loading="loading" @register="registerShortcut" class="h-full" />
        </div>

        <div class="grid-item item-empty-left"></div>

        <div class="grid-item item-button">
          <button @click="capture" :disabled="loading || selectedMonitor === null || !savePath" class="capture-btn neu-btn card flex items-center justify-center gap-4 w-full h-full">
            <Icon class="btn-content" :class="{ spinning: loading }" :icon="loading ? 'material-symbols:hourglass-top' : 'material-symbols:radio-button-checked'" />
            <span class="btn-text" style="font-weight: 800; letter-spacing: 1px;">
              {{ loading ? "PROCESSING..." : "CAPTURE NOW" }}
            </span>
          </button>
        </div>

        <div class="grid-item item-empty-right"></div>
      </div>
    </main>

    <BaseToast />

    <SettingsDialog v-model="showSettings" />
  </div>
</template>

<style scoped>
.app-container {
  height: 100vh;
  background-color: var(--bg-color);
  transition: background-color 0.3s ease;
}

.hamburger-btn,
.theme-btn {
  width: 48px;
  height: 48px;
  color: var(--text-muted);
}
.hamburger-btn:hover,
.theme-btn:hover {
  color: var(--green-200);
}

.app-title {
  font-size: 1.8rem;
  font-weight: 700;
  color: var(--text-main);
  letter-spacing: -0.5px;
}

.version {
  display: inline-block;
  font-size: 0.9rem;
  color: var(--green-200);
}

.custom-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-template-rows: repeat(4, minmax(100px, 1fr));
  width: 100%;
  height: 100%;
}

.item-monitors {
  grid-column: 1 / span 2;
  grid-row: 1 / span 3;
}

.item-scale {
  grid-column: 3 / span 2;
  grid-row: 1;
}

.item-path {
  grid-column: 3 / span 2;
  grid-row: 2;
}

.item-hotkey {
  grid-column: 3 / span 2;
  grid-row: 3;
}

.item-empty-left {
  grid-column: 1;
  grid-row: 4;
}

.item-button {
  grid-column: 2 / span 2;
  grid-row: 4;
}

.item-empty-right {
  grid-column: 4;
  grid-row: 4;
}

@media (max-width: 800px) {
  .custom-grid {
    grid-template-columns: 1fr;
    grid-template-rows: auto;
  }
  .item-monitors,
  .item-scale,
  .item-path,
  .item-hotkey,
  .item-button {
    grid-column: 1 / -1;
    grid-row: auto;
  }
  .item-empty-left,
  .item-empty-right {
    display: none;
  }
}

.capture-btn {
  width: 100%;
  font-size: 1.3rem;
  color: var(--green-400);
  transition: transform 0.1s, box-shadow 0.2s;
  cursor: pointer;
  border: none;
}

.capture-btn:active:not(:disabled) {
  transform: scale(0.98);
}
.capture-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.status-msg {
  color: var(--text-muted);
  font-size: 0.9rem;
  text-align: center;
  max-width: 80%;
}

.spinning {
  animation: spin 1s linear infinite;
  display: inline-block;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
