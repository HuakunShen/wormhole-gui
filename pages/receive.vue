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
          >
            <template #append>
              <el-button @click="paste" :icon="ListIcon" />
            </template>
          </el-input>
        </el-col>
        <el-col :span="12">
          <el-button
            @click="receive"
            text
            bg
            style="width: 100%"
            type="primary"
          >
            {{ $t("receive") }}
          </el-button>
        </el-col>
      </el-row>
    </el-form>
  </NuxtLayout>
</template>
<script setup lang="ts">
import { List as ListIcon } from "@element-plus/icons-vue";
import { readText } from "@tauri-apps/api/clipboard";

const form = reactive({
  receiveCode: "",
});

function paste() {
  readText().then((value) => {
    if (value) {
      form.receiveCode = value;
    }
  });
}

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
