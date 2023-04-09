<template>
  <NuxtLayout>
    <el-input v-model="path" placeholder="File Path">
      <template #prepend>
        <el-button @click="chooseFile">Send File</el-button>
      </template>
      <template #append>
        <el-button @click="chooseDirectory">Send Directory</el-button>
      </template>
    </el-input>
    <br /><br />
    <el-row :gutter="10">
      <el-col :span="12"
        ><el-button style="width: 100%" @click="send" type="primary" text bg
          >Send</el-button
        >
      </el-col>
      <el-col :span="12"
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
import { CopyDocument, Search } from "@element-plus/icons-vue";
import { writeText } from "@tauri-apps/api/clipboard";

const path = ref("");
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

function send() {
  ElMessage({
    type: "success",
    message: `Sending File`,
  });
}

async function chooseFile() {
  const selected = await open({
    multiple: true,
  });
  if (Array.isArray(selected)) {
    path.value = selected[0];
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    path.value = selected;
  }
}
async function chooseDirectory() {
  const selected = await open({
    directory: true,
  });
  if (Array.isArray(selected)) {
    path.value = selected[0];
  } else if (selected === null) {
    // user cancelled the selection
  } else {
    path.value = selected;
  }
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
