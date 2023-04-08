<template>
  <NuxtLayout>
    <el-row :gutter="10">
      <el-col :span="8"
        ><el-button style="width: 100%" @click="sendFile" type="primary" text bg
          >Send File</el-button
        >
      </el-col>
      <el-col :span="8"
        ><el-button
          style="width: 100%"
          @click="sendDirectory"
          type="primary"
          text
          bg
          >Send Directory</el-button
        ></el-col
      >
      <el-col :span="8"
        ><el-button type="primary" text bg style="width: 100%" @click="sendText"
          >Send Text</el-button
        ></el-col
      >
    </el-row>
    <br />
    <div v-if="receiveCode.length">
      <strong>{{ $t("receiveCode") }}: </strong>
      <span>{{ receiveCode }}</span>
      <el-button
        style="margin-left: 0.5em"
        @click="saveCodeToClipboard"
        :icon="CopyDocument"
        circle
      />
    </div>
  </NuxtLayout>
</template>
<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { ElInput } from "element-plus";
import { CopyDocument } from "@element-plus/icons-vue";
import { writeText } from "@tauri-apps/api/clipboard";

const textarea = ref("");
const receiveCode = ref("1-2-3");

function saveCodeToClipboard() {
  writeText(receiveCode.value).then(() => {
    ElMessage({
      message: `Copied Receive Code: ${receiveCode.value}`,
      grouping: true,
      type: "success",
    });
  });
}

async function sendFile() {
  const selected = await open({
    multiple: true,
  });
  console.log(selected);
}
async function sendDirectory() {
  const selected = await open({
    directory: true,
  });
  console.log(selected);
}

const sendText = () => {
  ElMessageBox({
    title: "Text To Send",
    message: () =>
      h(ElInput, {
        modelValue: textarea.value,
        type: "textarea",
        rows: "5",
        "onUpdate:modelValue": (val: string) => {
          textarea.value = val;
        },
      }),
  })
    .then(() => {
      ElMessage({
        type: "success",
        message: `Sending Message`,
      });
    })
    .catch(() => {
      ElMessage({
        type: "info",
        message: "Canceled",
      });
    });
};
</script>
