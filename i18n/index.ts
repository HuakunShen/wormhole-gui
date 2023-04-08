// NuxtI18nOptions<unknown> | undefined
import { NuxtI18nOptions } from "@nuxtjs/i18n";

export default {
  locales: ["en", "cn"],
  defaultLocale: "en",
  vueI18n: {
    locale: "en",
    messages: {
      en: {
        home: "Home",
        setting: "Setting",
        about: "About",
        send: "Send",
        receive: "Receive",
        receiveCode: "Receive Code",
        theme: "Theme",
        darkTheme: "Dark Theme",
        sourceCode: "Source Code",
        author: "Author",
        language: "Language",
      },
      cn: {
        home: "首页",
        setting: "设置",
        about: "关于",
        send: "发送",
        receive: "接收",
        receiveCode: "接收码",
        theme: "主题",
        darkTheme: "深色",
        sourceCode: "源码",
        author: "作者",
        language: "语言",
      },
    },
  },
} as NuxtI18nOptions;
