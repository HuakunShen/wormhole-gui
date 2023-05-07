<template>
  <el-select
    v-model="locale"
    class="m-2"
    placeholder="Language"
    size="large"
    @change="langChange"
  >
    <el-option key="en" label="English" value="en" />
    <el-option key="cn" label="Chinese" value="cn" />
  </el-select>
</template>
<script setup lang="ts">
const { locale, setLocale, setLocaleCookie, locales } = useI18n();

function langChange(lang: string) {
  //   (e: any) => setLocale(e.target.value)
  setLocale(lang);
  setLocaleCookie(lang);
  console.log(lang);
}
</script>
