<template>
  <NuxtLayout>
    <h2>
      <p>{{ $t("send") }}</p>
    </h2>
    <el-button type="primary">{{ $t("send") }}</el-button>
    <el-form
      :model="form"
      label-width="120px"
      @submit.prevent
      @submit="receive"
    >
      <h2>{{ $t("receive") }}</h2>
      <el-input v-model="form.receiveCode" :placeholder="$t('receiveCode')" />
      <br /><br />
      <el-button type="success" @click="receive">{{ $t("receive") }}</el-button>
    </el-form>
  </NuxtLayout>
</template>
<script setup lang="ts">
const form = reactive({
  receiveCode: "",
});
const { locale } = useI18n();
onMounted(() => {
  console.log(locale.value);
});
function receive() {
  if (form.receiveCode.trim().length !== 0) {
    ElNotification({
      title: "Receive Code",
      message: form.receiveCode.trim(),
      type: "success",
    });
  }
}
</script>
