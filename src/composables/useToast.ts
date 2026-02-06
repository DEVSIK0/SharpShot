import { ref } from "vue";

export type ToastType = "info" | "success" | "warning" | "error";
export type ToastPosition = "top-left" | "top-right" | "bottom-left" | "bottom-right";

export interface Toast {
  id: number;
  title: string;
  description?: string;
  type: ToastType;
  duration: number;
}

const toasts = ref<Toast[]>([]);
const position = ref<ToastPosition>("bottom-right");

export function useToast(initialPosition?: ToastPosition) {
  if (initialPosition) {
    position.value = initialPosition;
  }

  const show = (options: { title: string; description?: string; type?: ToastType; duration?: number }) => {
    const id = Date.now() + Math.random();

    const toast: Toast = {
      id,
      title: options.title,
      description: options.description,
      type: options.type ?? "info",
      duration: options.duration ?? 4000,
    };

    toasts.value.push(toast);

    if (toast.duration > 0) {
      setTimeout(() => {
        remove(id);
      }, toast.duration);
    }
  };

  const remove = (id: number) => {
    const idx = toasts.value.findIndex((t) => t.id === id);
    if (idx !== -1) toasts.value.splice(idx, 1);
  };

  return {
    toasts,
    position,
    show,
    remove,
  };
}
