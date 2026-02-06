import "@/assets/styles/main.css";
import { createApp } from "vue";
import App from "./App.vue";
import baseComponents from "./baseComponents";

createApp(App).use(baseComponents).mount("#app");
