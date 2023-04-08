<template>
  <NuxtLayout>
    <h2>{{ $t("theme") }}</h2>
    <el-checkbox v-model="darkChecked" :label="$t('darkTheme')" size="large" />
    <h2>{{ $t("language") }}</h2>
    <Locale />
  </NuxtLayout>
</template>
<script setup lang="ts">
import { useDark, useToggle } from "@vueuse/core";

const isDark = useDark();

const darkChecked = ref(isDark.value);
const toggleDark = useToggle(isDark);

watch(darkChecked, (newTheme) => {
  toggleDark(newTheme);
});
</script>
