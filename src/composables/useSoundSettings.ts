import { ref, watch } from "vue";
import cameraSoundPath from "@/assets/sfx/camera.mp3";
import { useToast } from "@/composables/useToast";

const VOLUME_KEY = "settings_volume";
const MUTED_KEY = "settings_muted";

const volume = ref(10);
const muted = ref(false);

const { show } = useToast();
let debounceTimeout: any;

const saveWithDebounce = () => {
  if (debounceTimeout) clearTimeout(debounceTimeout);
  debounceTimeout = setTimeout(() => {
    localStorage.setItem(VOLUME_KEY, volume.value.toString());
    localStorage.setItem(MUTED_KEY, muted.value.toString());
    show({ title: "Settings Saved", description: "Audio preferences updated.", type: "success", duration: 2000 });
  }, 1000);
};

const savedVolume = localStorage.getItem(VOLUME_KEY);
if (savedVolume != null) {
  volume.value = parseInt(savedVolume, 10);
}

const savedMuted = localStorage.getItem(MUTED_KEY);
if (savedMuted != null) {
  muted.value = savedMuted === "true";
}

watch([volume, muted], () => {
  saveWithDebounce();
});

export const useSoundSettings = () => {
  const playCameraSound = () => {
    if (muted.value) return;

    const audio = new Audio(cameraSoundPath);
    audio.volume = volume.value / 100;
    audio.play().catch((e) => console.error("Error playing sound:", e));
  };

  return {
    volume,
    muted,
    playCameraSound,
  };
};
