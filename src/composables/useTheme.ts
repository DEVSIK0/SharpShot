import { ref, watch } from "vue";

const THEME_KEY = "app_theme";

export function useTheme() {
  const isDark = ref(false);

  const applyTheme = (dark: boolean) => {
    if (dark) {
      document.documentElement.setAttribute("data-theme", "dark");
    } else {
      document.documentElement.setAttribute("data-theme", "light");
    }
  };

  const savedTheme = localStorage.getItem(THEME_KEY);
  if (savedTheme) {
    isDark.value = savedTheme === "dark";
  }
  applyTheme(isDark.value);

  const toggleTheme = () => {
    isDark.value = !isDark.value;
  };

  watch(isDark, (newVal) => {
    localStorage.setItem(THEME_KEY, newVal ? "dark" : "light");
    applyTheme(newVal);
  });

  return {
    isDark,
    toggleTheme,
  };
}
