import messages from "@intlify/unplugin-vue-i18n/messages";
import { MotionPlugin } from "@vueuse/motion";
import { createI18n } from "vue-i18n";
import { createPinia } from "pinia";
import { createApp } from "vue";
import router from "./router";
import App from "./App.vue";
import "./assets/main.css";

const locale = localStorage.getItem("locale");

createApp(App)
  .use(createPinia())
  .use(MotionPlugin)
  .use(
    createI18n({
      legacy: false,
      globalInjection: false,
      locale: locale ? JSON.parse(locale).key : "fr",
      fallbackLocale: "fr",
      availableLocales: ["en", "fr", "ar", "de"],
      messages: messages,
    })
  )
  .use(router)
  .mount("#app");
