<script setup lang="ts">
import { computed } from "vue";
import type { Monitor } from "@/types";

const props = defineProps<{
  monitors: Monitor[];
  loading?: boolean;
}>();

const selectedMonitorId = defineModel<number | null>("selectedMonitorId");
const selectedScale = defineModel<number>("selectedScale", { default: 2.0 });

const scaleFixed = computed(() => {
  return parseFloat(parseFloat(String(selectedScale.value)).toFixed(1));
});

function selectMonitor(id: number) {
  if (props.loading) return;
  selectedMonitorId.value = id;
}
</script>

<template>
  <div class="card settings-card neu-flat flex flex-col gap-8 p-6">
    <div class="section-monitor">
      <h2 class="card-title mb-4">Select Screen</h2>

      <div v-if="monitors.length === 0" class="no-monitors p-5">Loading monitors...</div>

      <div v-else class="monitors-grid gap-5">
        <button
          v-for="(monitor, index) in monitors"
          :key="monitor.id"
          type="button"
          class="monitor-btn flex flex-col items-center p-4"
          :class="selectedMonitorId === monitor.id ? 'neu-pressed active' : 'neu-flat'"
          @click="selectMonitor(monitor.id)"
          :disabled="loading"
        >
          <div class="monitor-preview-container flex items-center justify-center mb-3 w-full">
            <div class="monitor-preview" :style="{ aspectRatio: `${monitor.width}/${monitor.height}` }">
              <div v-if="monitor.is_primary" class="primary-badge px-[6px] py-[2px]">PRIMARY</div>
            </div>
          </div>

          <div class="monitor-info flex flex-col gap-1 w-full">
            <span class="monitor-name" :title="monitor.name">Monitor {{ index + 1 }}</span>
            <span class="monitor-res">{{ monitor.width }} x {{ monitor.height }}</span>
          </div>

          <div class="status-indicator"></div>
        </button>
      </div>
    </div>

    <div class="divider"></div>

    <div class="section-scale">
      <div class="range-header flex items-center justify-between mb-[10px]">
        <h2 class="card-title mb-0">Capture Scale</h2>
        <span class="range-value neu-pressed px-3 py-1">x{{ scaleFixed.toFixed(1) }}</span>
      </div>
      <input type="range" class="neu-range mt-[15px] mb-[5px]" id="scale-range" v-model.number="selectedScale" min="1.0" max="5.0" step="0.1" :disabled="loading" />
      <div class="scale-labels flex justify-between px-1 mt-1">
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

.monitors-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
}

.monitor-btn {
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  min-height: 140px;
  color: var(--text-main);
  text-align: center;
}

.monitor-btn:hover:not(:disabled) {
  transform: translateY(-2px);
}

.monitor-btn.active {
  border-color: var(--green-200);
  color: var(--green-400);
}

.monitor-preview-container {
  height: 60px;
  width: 100%;
}

.monitor-preview {
  background-color: var(--text-muted);
  height: 100%;
  max-width: 100%;
  border-radius: 4px;
  position: relative;
  border: 2px solid var(--text-main);
  opacity: 0.3;
  transition: all 0.3s;
}

.monitor-btn.active .monitor-preview {
  background-color: var(--green-200);
  border-color: var(--green-400);
  opacity: 1;
}

.primary-badge {
  position: absolute;
  bottom: 4px;
  left: 50%;
  transform: translateX(-50%);
  background: var(--green-300);
  color: white;
  border-radius: 4px;
  font-size: 0.6em;
  font-weight: 700;
  box-shadow: 0 1px 2px rgba(0, 0, 0, 0.1);
  letter-spacing: 0.5px;
  z-index: 2;
  pointer-events: none;
}

.monitor-info {
  z-index: 1;
}

.monitor-name {
  font-weight: 600;
  font-size: 0.9em;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: 100%;
}

.monitor-res {
  font-size: 0.75em;
  opacity: 0.7;
}

.no-monitors {
  text-align: center;
  color: var(--text-muted);
  font-style: italic;
}

.divider {
  height: 2px;
  background: var(--bg-color);
  box-shadow: inset 1px 1px 2px var(--shadow-dark), inset -1px -1px 2px var(--shadow-light);
  border-radius: 2px;
  width: 100%;
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
  height: 24px;
  width: 24px;
  border-radius: 50%;
  background: var(--bg-color);
  cursor: pointer;
  -webkit-appearance: none;
  margin-top: -6px;
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
