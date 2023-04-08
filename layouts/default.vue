<template>
  <el-container class="layout-container-demo">
    <el-aside width="130px">
      <el-scrollbar>
        <el-menu
          :default-openeds="['1', '3']"
          style="height: 100vh"
          :router="true"
        >
          <!-- <el-menu-item index="/" route="/">
            <template #title>
              <el-icon><HomeFilled /></el-icon>{{ $t("home") }}
            </template>
          </el-menu-item> -->
          <el-menu-item index="/send" route="/send">
            <template #title>
              <el-icon><Promotion /></el-icon>{{ $t("send") }}
            </template>
          </el-menu-item>
          <el-menu-item index="/receive" route="/receive">
            <template #title>
              <el-icon><Download /></el-icon>{{ $t("receive") }}
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
import {
  Setting,
  HomeFilled,
  InfoFilled,
  Promotion,
  Download,
} from "@element-plus/icons-vue";
import { useDark, useToggle } from "@vueuse/core";
const { locale } = useI18n();

const isDark = useDark();
const toggleDark = useToggle(isDark);
toggleDark(isDark.value);
</script>

<style>
body {
  margin: 0;
  height: 100vh;
}
</style>
