<script setup lang="ts">
import type { Monitor } from "@/types";

const props = defineProps<{
  monitors: Monitor[];
  loading?: boolean;
}>();

const selectedMonitorId = defineModel<number | null>("selectedMonitorId");

function selectMonitor(id: number) {
  if (props.loading) return;
  selectedMonitorId.value = id;
}
</script>

<template>
  <div class="card settings-card neu-flat flex flex-col gap-5 p-6 w-full h-full">
    <div class="section-monitor flex flex-col h-full w-full">
      <h2 class="card-title mb-4">Select Screen</h2>

      <div v-if="monitors.length === 0" class="no-monitors p-5 flex-1 flex items-center justify-center">Loading monitors...</div>

      <div v-else class="monitors-grid gap-5 py-2 flex-1 overflow-y-auto pr-2">
        <button
          v-for="(monitor, index) in monitors"
          :key="monitor.id"
          type="button"
          class="monitor-btn flex flex-col items-center p-4"
          :class="selectedMonitorId === monitor.id ? 'neu-pressed active' : 'neu-flat'"
          @click="selectMonitor(monitor.id)"
          :disabled="loading"
        >
          <div class="monitor-preview-container flex flex-1 items-center justify-center mb-3 w-full">
            <div class="monitor-preview" :style="{ aspectRatio: `${monitor.width}/${monitor.height}` }">
              <div v-if="monitor.is_primary" class="primary-badge px-[6px] py-[2px]">PRIMARY</div>
            </div>
          </div>

          <div class="monitor-info flex flex-col gap-1 w-full mt-auto">
            <span class="monitor-name" :title="monitor.name">Monitor {{ index + 1 }}</span>
            <span class="monitor-res">{{ monitor.width }} x {{ monitor.height }}</span>
          </div>

          <div class="status-indicator"></div>
        </button>
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
  align-items: stretch;
}

.monitor-btn {
  border: 2px solid transparent;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  height: 100%;
  color: var(--text-main);
  text-align: center;
}

.monitor-btn:hover:not(:disabled) {
  transform: translateY(-2px);
}

.monitor-btn.active {
  border-color: var(--green-main);
  color: var(--green-400);
}

.monitor-preview-container {
  height: 50%;
  min-height: 60px;
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
  background-color: var(--green-main);
  border-color: var(--green-main);
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
  font-size: clamp(0.75rem, 1vw, 0.9rem);
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
</style>
