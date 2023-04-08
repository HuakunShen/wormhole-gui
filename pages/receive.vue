<template>
  <NuxtLayout>
    <el-form
      :model="form"
      label-width="120px"
      @submit.prevent
      @submit="receive"
    >
      <el-row :gutter="10">
        <el-col :span="12">
          <el-input
            class="w-52"
            v-model="form.receiveCode"
            :placeholder="$t('receiveCode')"
          />
        </el-col>
        <el-col :span="12">
          <el-button @click="receive" text bg style="width: 100%" type="primary">
            {{ $t("receive") }}
          </el-button>
        </el-col>
      </el-row>
    </el-form>
  </NuxtLayout>
</template>
<script setup lang="ts">
const form = reactive({
  receiveCode: "",
});
function receive() {
  if (form.receiveCode.trim().length !== 0) {
    ElNotification({
      title: "Receive Code",
      message: form.receiveCode.trim(),
      type: "success",
      position: "bottom-right",
    });
  }
}
</script>
