import defaultI18n from "./i18n";

// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false, // SSR must be turned off
  css: ["@/assets/css/global.css"],
  modules: [
    "@nuxtjs/tailwindcss",
    "@element-plus/nuxt",
    "@nuxtjs/i18n",
    "@nuxtjs/google-fonts",
  ],
  vite: {
    define: {
      "process.env.DEBUG": false,
    },
  },
  i18n: defaultI18n,
  elementPlus: {
    themes: ["dark"],
  },
  googleFonts: {
    download: true,
    families: {
      Roboto: true,
      "Josefin+Sans": true,
      Lato: [100, 300],
      Raleway: {
        wght: [100, 400],
        ital: [100],
      },
    },
  },
  tailwindcss: {
    exposeConfig: true,
  },
});
