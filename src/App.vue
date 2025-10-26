<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { join } from "@tauri-apps/api/path";
import { open } from "@tauri-apps/plugin-dialog";
import { register, unregisterAll, isRegistered } from "@tauri-apps/plugin-global-shortcut";

// --- Claves de LocalStorage ---
const SAVE_PATH_KEY = "save_path";
const HOTKEY_KEY_KEY = "hotkey_key"; // Clave única para guardar la tecla

interface Monitor {
  id: number;
  name: string;
  width: number;
  height: number;
  is_primary: boolean;
}

const loading = ref(false);
const msg = ref("");
const monitors = ref<Monitor[]>([]);
const selectedMonitor = ref<number | null>(null);
const savePath = ref<string>("");

const selectedScale = ref<number>(2.0);
const scaleFixed = computed<number>(() => parseFloat(parseFloat(String(selectedScale.value)).toFixed(1)));

// --- 1. Nuevo Estado Simplificado para el Atajo ---
// Valor por defecto: F12 (una tecla común para utilidades)
const hotkeyKey = ref("F12");
const currentShortcut = ref<string>(""); // La tecla que está realmente registrada

// Opciones de teclas comunes que Tauri entiende (puedes expandir esta lista)
const keyOptions = [
  "F1",
  "F2",
  "F3",
  "F4",
  "F5",
  "F6",
  "F7",
  "F8",
  "F9",
  "F10",
  "F11",
  "F12",
  "A",
  "B",
  "C",
  "D",
  "E",
  "F",
  "G",
  "H",
  "I",
  "J",
  "K",
  "L",
  "M",
  "N",
  "O",
  "P",
  "Q",
  "R",
  "S",
  "T",
  "U",
  "V",
  "W",
  "X",
  "Y",
  "Z",
  "Space",
  "Enter",
  "Tab",
  "Escape",
  "Delete",
  "Insert",
  "Home",
  "End",
  "PageUp",
  "PageDown",
];

// --- 2. Atajo Computado (Simplificado) ---
// Simplemente devuelve la tecla seleccionada.
const computedShortcut = computed(() => {
  // Asegura que siempre esté en mayúsculas si es una letra
  return hotkeyKey.value.toUpperCase();
});

// --- Función de Timestamp (sin cambios) ---
const generateTimestamp = () => {
  const now = new Date();
  const year = now.getFullYear();
  const month = String(now.getMonth() + 1).padStart(2, "0");
  const day = String(now.getDate()).padStart(2, "0");
  const hours = String(now.getHours()).padStart(2, "0");
  const minutes = String(now.getMinutes()).padStart(2, "0");
  const seconds = String(now.getSeconds()).padStart(2, "0");
  const milliseconds = String(now.getMilliseconds()).padStart(3, "0");
  return `${year}${month}${day}${hours}${minutes}${seconds}${milliseconds}`;
};

// --- loadMonitors (sin cambios) ---
async function loadMonitors() {
  try {
    const monitorList = await invoke<Monitor[]>("list_monitors");
    monitors.value = monitorList;
    const primary = monitorList.find((m) => m.is_primary);
    if (primary != null) {
      selectedMonitor.value = primary.id;
    } else if (monitorList.length > 0) {
      selectedMonitor.value = monitorList[0].id;
    }
  } catch (e: any) {
    msg.value = `Error al cargar monitores: ${e?.toString?.() ?? e}`;
  }
}

// --- selectSavePath (sin cambios) ---
async function selectSavePath() {
  const initialPath = savePath.value;

  const path = await open({
    title: "Seleccionar ubicación para guardar la captura",
    multiple: false,
    directory: true,
    defaultPath: initialPath,
  });

  if (path != null) {
    const newPath = Array.isArray(path) ? path[0] : path;
    savePath.value = newPath;
    localStorage.setItem(SAVE_PATH_KEY, newPath);
    msg.value = `Ruta seleccionada: ${savePath.value}`;
  } else {
    msg.value = "Selección de ruta cancelada.";
  }
}

// --- 3. Lógica de Registro de Atajo (Simplificada) ---
async function registerShortcut() {
  // Limpia el atajo anterior si existía
  if (currentShortcut.value && (await isRegistered(currentShortcut.value))) {
    await unregisterAll();
  }

  const newShortcut = computedShortcut.value;
  if (!newShortcut) {
    msg.value = "Atajo inválido. Por favor, selecciona una tecla.";
    return;
  }

  try {
    // Registra el nuevo atajo (solo la tecla)
    await register(newShortcut, capture);

    // Guarda el éxito
    currentShortcut.value = newShortcut;
    localStorage.setItem(HOTKEY_KEY_KEY, hotkeyKey.value);

    msg.value = `Atajo guardado: ${newShortcut}. ¡Prueba a pulsarlo!`;
  } catch (e: any) {
    msg.value = `Error al registrar el atajo. ¿Quizás ya está en uso por el sistema? ${e?.toString?.() ?? e}`;
  }
}

// --- capture (sin cambios) ---
async function capture() {
  if (loading.value) {
    msg.value = "Ya hay una captura en proceso...";
    return;
  }

  if (selectedMonitor.value == null) {
    msg.value = "Error: No hay ningún monitor seleccionado.";
    return;
  }

  if (!savePath.value) {
    msg.value = "Error: No se ha seleccionado una carpeta de guardado.";
    return;
  }

  loading.value = true;

  try {
    msg.value = "Capturando y escalando...";
    const scaleFormatted = scaleFixed.value.toFixed(1).replace(".", "p");
    const filename = `Sharpshot_x${scaleFormatted}_${generateTimestamp()}.png`;
    const filePath = await join(savePath.value, filename);

    const outPath = await invoke<string>("capture_and_scale", {
      monitorId: selectedMonitor.value,
      scale: scaleFixed.value,
      savePath: filePath,
    });

    msg.value = `¡Captura guardada! ${outPath}`;
  } catch (e: any) {
    msg.value = `Error: ${e?.toString?.() ?? e}`;
  } finally {
    loading.value = false;
  }
}

// --- 4. onMounted (Simplificado) ---
onMounted(() => {
  loadMonitors();

  // Cargar ruta de guardado
  const savedPath = localStorage.getItem(SAVE_PATH_KEY);
  if (savedPath != null) {
    savePath.value = savedPath;
    if (savedPath) {
      msg.value = `Ruta de guardado cargada: ${savedPath}`;
    }
  }

  // Cargar tecla guardada
  const savedKey = localStorage.getItem(HOTKEY_KEY_KEY);

  if (savedKey) {
    hotkeyKey.value = savedKey;
  }

  // Registrar el atajo guardado (o el por defecto) al iniciar
  registerShortcut();
});

// --- onUnmounted (sin cambios) ---
onUnmounted(async () => {
  await unregisterAll();
});
</script>

<template>
  <div class="container">
    <h1 class="title">Sharpshot - Escalador de Capturas 📸</h1>

    <div class="card settings-card">
      <h2 class="card-title">1. Configuración de Captura</h2>

      <div class="control-group">
        <label for="monitor-select">Monitor a Capturar</label>
        <select id="monitor-select" v-model="selectedMonitor" :disabled="monitors.length === 0 || loading">
          <option :value="null" disabled>
            {{ monitors.length === 0 ? "Cargando monitores..." : "Seleccione un monitor" }}
          </option>
          <option v-for="monitor in monitors" :key="monitor.id" :value="monitor.id">
            {{ monitor.name }}
          </option>
        </select>
      </div>

      <div class="control-group">
        <label for="scale-range">Factor de Escalado: x{{ parseFloat(selectedScale.toString()).toFixed(1) }}</label>
        <input type="range" id="scale-range" v-model="selectedScale" min="1.0" max="5.0" step="0.1" :disabled="loading" />
      </div>
    </div>

    <div class="card save-card">
      <h2 class="card-title">2. Ubicación de Guardado</h2>

      <div class="path-info">
        <p v-if="savePath" class="path-display">
          Guardar en: <strong>{{ savePath }}</strong>
        </p>
        <p v-else class="path-display text-muted">Ninguna ruta seleccionada.</p>
      </div>

      <button @click="selectSavePath" :disabled="loading" class="select-btn">
        {{ savePath ? "Cambiar Carpeta" : "Seleccionar Carpeta" }}
      </button>
    </div>

    <div class="card hotkey-card">
      <h2 class="card-title">3. Atajo Global</h2>
      <div class="control-group">
        <label for="hotkey-select">Tecla de Captura</label>
        <select id="hotkey-select" v-model="hotkeyKey" :disabled="loading">
          <option v-for="key in keyOptions" :key="key" :value="key">
            {{ key }}
          </option>
        </select>
      </div>
      <button @click="registerShortcut" :disabled="loading" class="hotkey-btn">Guardar Tecla de Atajo ({{ computedShortcut }})</button>
    </div>

    <button @click="capture" :disabled="loading || selectedMonitor === null || !savePath" class="primary-action-btn">
      {{ loading ? "Procesando captura y guardado..." : "Capturar, Escalar y Guardar (Manual)" }}
    </button>

    <p v-if="msg" :class="['message', { loading: loading }]">{{ msg }}</p>
  </div>
</template>

<style scoped>
.container {
  display: flex;
  flex-direction: column;
  gap: 25px;
  align-items: center;
  padding: 30px;
  font-family: system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Open Sans", "Helvetica Neue", sans-serif;
}

.title {
  color: #007aff;
  margin-bottom: 5px;
}

.card {
  * {
    color: white !important;
  }
  border: 1px solid black;
  padding: 20px;
  border-radius: 12px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.08);
  width: 100%;
  max-width: 500px;
}

.card-title {
  font-size: 1.2em;
  color: #333;
  border-bottom: 1px solid #eee;
  padding-bottom: 10px;
  margin-top: 0;
  margin-bottom: 15px;
}

.control-group {
  margin-bottom: 15px;
  display: flex;
  flex-direction: column;
}

label {
  font-weight: 600;
  color: #555;
  margin-bottom: 5px;
}

select,
input[type="text"] {
  padding: 10px;
  border-radius: 8px;
  border: 1px solid #ddd;
  font-size: 15px;
  background-color: inherit;
  flex-grow: 1;
}

#scale-range {
  width: 100%;
  margin-top: 5px;
}

.select-btn {
  width: 100%;
  padding: 10px;
  background-color: #ff9500;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.hotkey-btn {
  width: 100%;
  padding: 10px;
  border: none;
  border-radius: 8px;
  cursor: pointer;
}

.primary-action-btn {
  width: 100%;
  max-width: 500px;
  padding: 15px;
  font-size: 1.1em;
  font-weight: bold;
  background-color: #28a745;
  color: white;
  border: none;
  border-radius: 8px;
  cursor: pointer;
}
.primary-action-btn:disabled {
  background-color: #aaa;
  cursor: not-allowed;
}

.path-info {
  min-height: 40px;
}
.path-display {
  font-size: 0.9em;
  color: #666;
  margin-bottom: 15px;
  word-break: break-all;
}
.text-muted {
  color: #aaa;
}

.message {
  margin-top: 10px;
  padding: 10px;
  border-radius: 6px;
  background-color: #e6f7ff;
  border: 1px solid #b3e0ff;
  color: #007bff;
  width: 100%;
  max-width: 500px;
  text-align: center;
}
.message.loading {
  background-color: #fff3cd;
  border-color: #ffeeba;
  color: #856404;
}
</style>
