<script setup lang="ts">
import { ref, onMounted, onUnmounted, watch } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";
import { register, unregisterAll, isRegistered } from "@tauri-apps/plugin-global-shortcut";

import { useSoundSettings } from "@/composables/useSoundSettings";
import type { Monitor } from "@/types";

import MonitorSelectCard from "@/components/MonitorSelectCard.vue";
import SavePathCard from "@/components/SavePathCard.vue";
import HotkeyCard from "@/components/HotkeyCard.vue";
import SettingsDialog from "@/components/SettingsDialog.vue";
import BaseToast from "@/components/base/BaseToast.vue";
import { useToast } from "@/composables/useToast";
import { useVersion } from "./composables/useVersion";

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
    const filename = `Sharpshot_x${scaleFormatted}_${generateTimestamp()}.png`;
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
  <div class="app-container flex flex-col p-6 gap-[30px]">
    <header class="top-bar flex items-center justify-between px-[10px]">
      <h1 class="app-title flex-1">
        Sharpshot <span class="version">{{ version }}</span>
      </h1>
      <button type="button" class="hamburger-btn neu-btn flex items-center justify-center" @click="showSettings = true" aria-label="Open Settings">
        <Icon icon="material-symbols:menu" style="font-size: 25px" />
      </button>
    </header>

    <main class="content-area flex flex-col flex-1 gap-6 justify-around">
      <div class="cards-grid gap-6 items-stretch">
        <MonitorSelectCard :monitors="monitors" v-model:selectedMonitorId="selectedMonitor" v-model:selectedScale="selectedScale" :loading="loading" />

        <div class="sub-grid flex flex-col gap-6">
          <SavePathCard v-model:savePath="savePath" :loading="loading" />
          <HotkeyCard v-model:hotkeyKey="hotkeyKey" :loading="loading" @register="registerShortcut" />
        </div>
      </div>

      <div class="footer-action flex flex-col items-center gap-5 mb-5">
        <button @click="capture" :disabled="loading || selectedMonitor === null || !savePath" class="capture-btn neu-btn flex items-center justify-center gap-4">
          <Icon class="btn-content" :class="{ spinning: loading }" :icon="loading ? 'material-symbols:hourglass-top' : 'material-symbols:radio-button-checked'" />
          <span class="btn-text">
            {{ loading ? "Processing..." : "CAPTURE NOW" }}
          </span>
        </button>
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
}

.hamburger-btn {
  width: 48px;
  height: 48px;
  color: var(--text-muted);
}
.hamburger-btn:hover {
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

.cards-grid {
  display: grid;
  grid-template-columns: 1.2fr 1fr;
}

.capture-btn {
  width: 100%;
  max-width: 400px;
  height: 80px;
  font-size: 1.2rem;
  color: var(--green-500);
  transition: transform 0.1s;
}

.capture-btn:active:not(:disabled) {
  transform: scale(0.98);
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

@media (max-width: 700px) {
  .cards-grid {
    grid-template-columns: 1fr;
  }
}
</style>
