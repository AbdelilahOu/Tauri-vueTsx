import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import vueJsx from "@vitejs/plugin-vue-jsx";
import svgLoader from "vite-svg-loader";
import VueI18nPlugin from "@intlify/unplugin-vue-i18n/vite";
import Components from "unplugin-vue-components/vite";
import { ElementPlusResolver } from "unplugin-vue-components/resolvers";
// https://vitejs.dev/config/
export default defineConfig({
  base: "./",
  plugins: [
    vue(),
    vueJsx(),
    svgLoader(),
    VueI18nPlugin({
      include: "./src/locales/**",
    }),
    Components({
      extensions: ["vue", "md", "svg"],
      directoryAsNamespace: false,
      dts: true,
      globalNamespaces: ["global"],
      include: [/\.vue$/, /\.md$/],
      // resolvers: [
      //   (name) => {
      //     if (name === "MyCustom")
      //       return path
      //         .resolve(__dirname, "src/CustomResolved.vue")
      //         .replaceAll("\\", "/");
      //   },
      //   VantResolver(),
      //   IconsResolver({
      //     componentPrefix: "i",
      //   }),
      // ],
    }),
  ],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
    },
  },
});
