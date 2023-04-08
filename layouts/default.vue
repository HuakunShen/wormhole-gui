<template>
  <el-container class="layout-container-demo" style="height: 100vh">
    <el-aside width="200px">
      <el-scrollbar>
        <el-menu
          :default-openeds="['1', '3']"
          style="height: 100vh"
          :router="true"
        >
          <el-menu-item index="/" route="/">
            <template #title>
              <el-icon><HomeFilled /></el-icon>{{ $t("home") }}
            </template>
          </el-menu-item>
          <el-menu-item index="/setting" route="/setting">
            <template #title>
              <el-icon><setting /></el-icon>{{ $t("setting") }}
            </template>
          </el-menu-item>
          <el-menu-item index="/about" route="/about">
            <template #title>
              <el-icon><InfoFilled /></el-icon>{{ $t("about") }}
            </template>
          </el-menu-item>
        </el-menu>
      </el-scrollbar>
    </el-aside>
    <el-container>
      <el-main>
        <slot />
      </el-main>
    </el-container>
  </el-container>
</template>
<script lang="ts" setup>
import { Setting, HomeFilled, InfoFilled } from "@element-plus/icons-vue";
import { useDark, useToggle } from "@vueuse/core";
const { locale } = useI18n();

const isDark = useDark();
const toggleDark = useToggle(isDark);
toggleDark(isDark.value);

watch(locale, (newLocale) => {
  console.log(locale.value);
});
</script>

<style>
body {
  margin: 0;
}
</style>
