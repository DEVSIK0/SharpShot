import { App } from "vue";

import BaseDialog from "./components/base/BaseDialog.vue";
import { Icon } from "@iconify/vue";

declare module "@vue/runtime-core" {
  export interface GlobalComponents {
    BaseDialog: typeof BaseDialog;
    Icon: typeof Icon;
  }
}

export default {
  install(app: App) {
    app.component("BaseDialog", BaseDialog);
    app.component("Icon", Icon);
  },
};
