<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  loading?: boolean;
}>();

const emit = defineEmits<{
  (e: "register"): void;
}>();

const hotkeyKey = defineModel<string>("hotkeyKey", { required: true });

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

const computedShortcut = computed(() => hotkeyKey.value.toUpperCase());
</script>

<template>
  <div class="card hotkey-card neu-flat flex flex-row flex-wrap items-center justify-between gap-[10px] p-4 w-full h-full">
    <h2 class="card-title m-0">GLOBAL SHORTCUT</h2>

    <div class="select-wrapper neu-pressed flex-1 flex items-center h-[40px] px-3">
      <select id="hotkey-select" v-model="hotkeyKey" :disabled="props.loading" class="select-input h-full w-full bg-transparent">
        <option v-for="key in keyOptions" :key="key" :value="key">
          {{ key }}
        </option>
      </select>
      <span class="custom-arrow">▼</span>
    </div>

    <button type="button" @click="emit('register')" :disabled="props.loading" class="hotkey-btn gap-2 flex items-center h-[40px] neu-btn px-4 whitespace-nowrap">
      <Icon icon="material-symbols:keyboard" />
      <span>Apply ({{ computedShortcut }})</span>
    </button>
  </div>
</template>

<style scoped>
.card-title {
  font-size: 1rem;
  color: var(--green-500);
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 1px;
  width: 100%;
}

label {
  font-weight: 600;
  color: var(--text-muted);
  font-size: 0.9em;
}

.select-wrapper {
  position: relative;
}

.select-input {
  width: 100%;
  border: none;
  background: transparent;
  font-size: 1rem;
  color: var(--text-main);
  appearance: none;
  cursor: pointer;
}
.select-input:focus {
  outline: none;
}

.custom-arrow {
  position: absolute;
  right: 15px;
  color: var(--green-200);
  pointer-events: none;
  font-size: 0.8rem;
}

.hotkey-btn:hover:not(:disabled) {
  color: var(--green-200);
}
</style>
