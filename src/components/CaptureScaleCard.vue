<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  loading?: boolean;
}>();

const selectedScale = defineModel<number>("selectedScale", { default: 2.0 });

const scaleFixed = computed(() => {
  return parseFloat(parseFloat(String(selectedScale.value)).toFixed(1));
});
</script>

<template>
  <div class="card settings-card neu-flat flex flex-col p-4 w-full h-full justify-center">
    <div class="section-scale flex gap-3 flex-col w-full">
      <div class="range-header flex items-center justify-between mb-0">
        <h2 class="card-title m-0">Capture Scale</h2>
        <span class="range-value neu-pressed px-2 py-1">x{{ scaleFixed.toFixed(1) }}</span>
      </div>
      <input type="range" class="neu-range mt-[10px] mb-[5px]" id="scale-range" v-model.number="selectedScale" min="1.0" max="5.0" step="0.1" :disabled="props.loading" />
      <div class="scale-labels flex justify-between px-1">
        <span>1x</span>
        <span>5x</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.card-title {
  font-size: 0.95em;
  color: var(--green-500);
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 1px;
}

.range-header {
  font-size: 0.9em;
}

.range-value {
  font-size: 0.85rem;
  color: var(--green-300);
  font-weight: bold;
}

.neu-range {
  appearance: none;
  -webkit-appearance: none;
  width: 100%;
  background: transparent;
}

.neu-range:focus {
  outline: none;
}

.neu-range::-webkit-slider-runnable-track {
  width: 100%;
  height: 12px;
  cursor: pointer;
  background: var(--bg-color);
  border-radius: 6px;
  box-shadow: inset 2px 2px 5px var(--shadow-dark), inset -2px -2px 5px var(--shadow-light);
}

.neu-range::-webkit-slider-thumb {
  height: 20px;
  width: 20px;
  border-radius: 50%;
  background: var(--bg-color);
  cursor: pointer;
  -webkit-appearance: none;
  margin-top: -4px;
  box-shadow: 3px 3px 6px var(--shadow-dark), -3px -3px 6px var(--shadow-light);
  border: 1px solid rgba(255, 255, 255, 0.1);
  transition: background 0.2s;
}

.neu-range:active::-webkit-slider-thumb {
  background: var(--green-50);
}

.scale-labels {
  font-size: 0.75em;
  color: var(--text-muted);
}
</style>
