<script setup lang="ts">
import { computed } from "vue";
import { useToast, type ToastType } from "@/composables/useToast";

const { toasts, position, remove } = useToast();

const transitionName = computed(() => {
  return position.value.includes("right") ? "slide-right" : "slide-left";
});

const getIcon = (type: ToastType) => {
  switch (type) {
    case "success":
      return "✔";
    case "warning":
      return "⚠";
    case "error":
      return "✖";
    default:
      return "ℹ";
  }
};
</script>

<template>
  <div class="toast-container" :class="position">
    <TransitionGroup :name="transitionName">
      <div v-for="toast in toasts" :key="toast.id" class="toast-item" :class="`toast-${toast.type}`" role="alert">
        <div class="toast-icon">{{ getIcon(toast.type) }}</div>
        <div class="toast-content">
          <h4 class="toast-title">{{ toast.title }}</h4>
          <p v-if="toast.description" class="toast-desc">{{ toast.description }}</p>
        </div>
        <button @click="remove(toast.id)" class="close-btn" aria-label="Close">×</button>
      </div>
    </TransitionGroup>
  </div>
</template>

<style scoped>
.toast-container {
  position: fixed;
  z-index: 9999;
  display: flex;
  gap: 12px;
  pointer-events: none;
  padding: 20px;
}

.top-right {
  top: 0;
  right: 0;
  flex-direction: column;
}
.bottom-right {
  bottom: 0;
  right: 0;
  flex-direction: column-reverse;
}
.top-left {
  top: 0;
  left: 0;
  flex-direction: column;
}
.bottom-left {
  bottom: 0;
  left: 0;
  flex-direction: column-reverse;
}

.toast-item {
  pointer-events: auto;
  min-width: 300px;
  max-width: 400px;
  padding: 16px;
  border-radius: 12px;
  display: flex;
  align-items: flex-start;
  gap: 12px;
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
}

.toast-success {
  border: 1px solid #4caf50;
  background: rgba(76, 175, 80, 0.15);
}
.toast-info {
  border: 1px solid #2196f3;
  background: rgba(33, 150, 243, 0.15);
}
.toast-warning {
  border: 1px solid #ff9800;
  background: rgba(255, 152, 0, 0.15);
}
.toast-error {
  border: 1px solid #f44336;
  background: rgba(244, 67, 54, 0.15);
}

.toast-title {
  margin: 0;
  font-size: 0.95rem;
  font-weight: 700;
  color: var(--text-main, #333);
}
.toast-desc {
  margin: 4px 0 0;
  font-size: 0.85rem;
  color: var(--text-muted, #666);
}
.toast-icon {
  font-size: 1.2rem;
}
.close-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  line-height: 0.5;
  padding: 0;
  cursor: pointer;
  opacity: 0.5;
  color: inherit;
  margin-left: auto;
}
.close-btn:hover {
  opacity: 1;
}

.toast-success .toast-title,
.toast-success .toast-icon {
  color: #2e7d32;
}
.toast-info .toast-title,
.toast-info .toast-icon {
  color: #0288d1;
}
.toast-warning .toast-title,
.toast-warning .toast-icon {
  color: #ed6c02;
}
.toast-error .toast-title,
.toast-error .toast-icon {
  color: #d32f2f;
}

.slide-right-enter-active,
.slide-right-leave-active,
.slide-left-enter-active,
.slide-left-leave-active {
  transition: all 0.4s cubic-bezier(0.5, 0, 0.2, 1);
}
.slide-right-enter-from,
.slide-right-leave-to {
  opacity: 0;
  transform: translateX(50px);
}
.slide-left-enter-from,
.slide-left-leave-to {
  opacity: 0;
  transform: translateX(-50px);
}
</style>
