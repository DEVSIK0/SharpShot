<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const loading = ref(false);
const msg = ref("");

async function capture() {
  loading.value = true;
  msg.value = "";
  try {
    const outPath = await invoke<string>("capture_and_scale_x2");
    msg.value = `Guardado: ${outPath}`;
  } catch (e: any) {
    msg.value = `Error: ${e?.toString?.() ?? e}`;
  } finally {
    loading.value = false;
  }
}
</script>

<template>
  <div style="display: grid; place-items: center; min-height: 100vh; gap: 12px">
    <h1>Photomode (Tauri + Vue)</h1>
    <button @click="capture" :disabled="loading" style="padding: 10px 16px">
      {{ loading ? "Capturando..." : "Capturar y escalar x2" }}
    </button>
    <p v-if="msg">{{ msg }}</p>
  </div>
</template>

<style>
html,
body,
#app {
  height: 100%;
  margin: 0;
}
</style>
