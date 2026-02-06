<script setup lang="ts">
import { useTemplateRef, watch, onMounted } from "vue";

const props = defineProps<{
  title: string;
  showHeader: boolean;
  showFooter: boolean;
  closeOnBackdrop?: boolean;
}>();

const open = defineModel<boolean>({ default: false });
const dialog = useTemplateRef<HTMLDialogElement>("REF_DIALOG");

watch(open, (newValue) => {
  if (!dialog.value) {
    return;
  }
  if (newValue && !dialog.value.open) {
    dialog.value.showModal();
  }
  if (!newValue && dialog.value.open) {
    dialog.value.close();
  }
});

const onBackdropClick = (e: MouseEvent) => {
  if (props.closeOnBackdrop && e.target === dialog.value) {
    dialog.value?.close();
  }
};

onMounted(() => {
  if (open.value && !dialog.value?.open) {
    dialog.value?.showModal();
  }
});
</script>

<template>
  <Transition name="dialog-fade">
    <dialog v-show="open" ref="REF_DIALOG" class="base-dialog neu-flat" @close="() => (open = false)" @click="onBackdropClick">
      <div class="dialog-container">
        <header v-if="showHeader" class="dialog-header">
          <slot name="header">
            <h3 class="dialog-title">{{ title }}</h3>
          </slot>

          <button class="close-btn neu-btn" @click="dialog?.close()" type="button">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="20"
              height="20"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <line x1="18" y1="6" x2="6" y2="18"></line>
              <line x1="6" y1="6" x2="18" y2="18"></line>
            </svg>
          </button>
        </header>

        <div class="dialog-content">
          <slot name="content" />
        </div>

        <footer v-if="showFooter" class="dialog-footer">
          <slot name="footer" />
        </footer>
      </div>
    </dialog>
  </Transition>
</template>

<style scoped>
.base-dialog {
  padding: 0;
  border: none;
  background: var(--bg-color);
  color: var(--text-main);
  box-shadow: 20px 20px 60px var(--shadow-dark), -20px -20px 60px var(--shadow-light);

  width: 90%;
  max-width: 500px;
  margin: auto;
  overflow: visible;
}

.base-dialog::backdrop {
  background-color: rgba(240, 240, 240, 0.4);
  backdrop-filter: blur(8px);
}

.dialog-fade-enter-active,
.dialog-fade-leave-active {
  transition: opacity 0.3s ease, transform 0.3s ease;
}

.dialog-fade-enter-from,
.dialog-fade-leave-to {
  opacity: 0;
  transform: scale(0.9);
}

.dialog-container {
  display: flex;
  flex-direction: column;
  max-height: 85vh;
  position: relative;
  background: var(--bg-color);
  border-radius: 20px;
}

.dialog-header {
  padding: 20px 24px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  border-bottom: 2px solid rgba(0, 0, 0, 0.02);
}

.dialog-title {
  font-size: 1.25em;
  font-weight: 700;
  margin: 0;
  color: var(--text-main);
}

.close-btn {
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 0;
  border-radius: 8px;
  color: var(--text-muted);
}
.close-btn:hover {
  color: var(--green-500);
}

.dialog-content {
  padding: 24px;
  overflow-y: auto;
}

.dialog-footer {
  padding: 20px 24px;
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  border-top: 2px solid rgba(0, 0, 0, 0.02);
}
</style>
