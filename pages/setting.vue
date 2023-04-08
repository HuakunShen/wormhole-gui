<template>
  <NuxtLayout>
    <h2>{{ $t("theme") }}</h2>
    <el-checkbox v-model="darkChecked" :label="$t('darkTheme')" size="large" />
    <h2>{{ $t("language") }}</h2>
    <Locale />
    <h2>Receive Directory</h2>
    <el-button @click="chooseDir" type="primary" text bg
      >Choose Default Directory</el-button
    >
  </NuxtLayout>
</template>
<script setup lang="ts">
import { useDark, useToggle } from "@vueuse/core";
import { open } from "@tauri-apps/api/dialog";

const isDark = useDark();

const darkChecked = ref(isDark.value);
const toggleDark = useToggle(isDark);

async function chooseDir() {
  const selected = await open({
    directory: true,
  });
  console.log(selected);
}

watch(darkChecked, (newTheme) => {
  toggleDark(newTheme);
});
</script>
