<script setup lang="ts">
import { useSoundSettings } from "@/composables/useSoundSettings";
import BaseDialog from "@/components/base/BaseDialog.vue";

const show = defineModel<boolean>({ default: false });
const { volume, muted, playCameraSound } = useSoundSettings();

const testSound = () => {
  playCameraSound();
};
</script>

<template>
  <BaseDialog v-model="show" title="Settings" :show-header="true" :show-footer="true">
    <template #content>
      <div class="settings-content flex flex-col gap-[30px] py-[10px]">
        <div class="setting-item flex flex-col gap-[10px]">
          <label for="volume-control">Volume ({{ volume }}%)</label>
          <div class="volume-controls flex items-center gap-4">
            <input id="volume-control" class="neu-range flex-1" type="range" v-model.number="volume" min="0" max="100" :disabled="muted" />

            <button @click="testSound" :disabled="muted" class="test-btn neu-btn flex items-center justify-center w-10 h-10 p-0" title="Test sound">
              <Icon icon="material-symbols:volume-down-rounded" class="text-2xl" />
            </button>
          </div>
        </div>

        <div class="setting-item checkbox-row neu-flat flex flex-row items-center justify-between p-4">
          <label for="mute-control" style="cursor: pointer">Mute Effects</label>
          <input id="mute-control" class="neu-checkbox flex items-center justify-center" type="checkbox" v-model="muted" />
        </div>
      </div>
    </template>

    <template #footer>
      <div class="w-full flex items-center justify-end">
        <button class="btn-close neu-btn py-2.5 px-6" @click="show = false">Close</button>
      </div>
    </template>
  </BaseDialog>
</template>

<style scoped>
label {
  font-weight: 600;
  color: var(--text-main);
}


.neu-range {
  appearance: none;
  -webkit-appearance: none;
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
}

.neu-checkbox {
  appearance: none;
  width: 28px;
  height: 28px;
  background: var(--bg-color);
  box-shadow: 5px 5px 10px var(--shadow-dark), -5px -5px 10px var(--shadow-light);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
}

.neu-checkbox:checked {
  box-shadow: inset 2px 2px 5px var(--shadow-dark), inset -2px -2px 5px var(--shadow-light);
}

.neu-checkbox:checked::after {
  content: "✔";
  color: var(--green-200);
  font-size: 1.2rem;
  font-weight: bold;
}

.test-btn {
  border-radius: 50%;
}

.test-btn:active {
  color: var(--green-200);
}

.btn-close {
  color: var(--text-main);
}

.version-label {
  font-size: 0.75rem;
  color: var(--text-muted);
}
</style>
