<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";

const props = defineProps<{
  loading?: boolean;
}>();

const savePath = defineModel<string>("savePath", { required: true });

const selectSavePath = async () => {
  const path = await open({
    title: "Select location to save capture",
    multiple: false,
    directory: true,
    defaultPath: savePath.value || undefined,
  });

  if (path != null) {
    savePath.value = Array.isArray(path) ? path[0] : path;
  }
};
</script>

<template>
  <div class="card save-card neu-flat flex flex-row flex-wrap items-center gap-[15px] p-5">
    <h2 class="card-title m-0">DESTINATION FOLDER</h2>

    <div class="path-info neu-pressed p-3 flex-1">
      <p v-if="savePath" class="path-display m-0" title="{{ savePath }}">
        {{ savePath }}
      </p>
      <p v-else class="path-display text-muted m-0">No path selected...</p>
    </div>

    <button @click="selectSavePath" :disabled="props.loading" class="select-btn gap-2 flex items-center neu-btn py-3 px-5">
      <Icon icon="material-symbols:folder" />
      <span> {{ savePath ? "Change" : "Select" }} </span>
    </button>
  </div>
</template>

<style scoped>
.card-title {
  font-size: 1rem;
  color: var(--green-500);
  font-weight: 700;
  width: 100%;
}

.path-info {
  overflow: hidden;
  white-space: nowrap;
  text-overflow: ellipsis;
  min-width: 0;
}

.path-display {
  font-size: 0.9em;
  color: var(--text-main);
  overflow: hidden;
  text-overflow: ellipsis;
}
.text-muted {
  color: var(--text-muted);
  font-style: italic;
}

.select-btn {
  font-size: 0.9rem;
  color: var(--text-main);
  background: var(--bg-color);
  white-space: nowrap;
}

.select-btn:hover {
  color: var(--green-200);
}
</style>
