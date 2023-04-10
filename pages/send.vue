<template>
  <NuxtLayout>
    <div class="flex flex-col h-full justify-end">
      <div class="grid grid-cols-1 gap-3">
        <el-input
          v-model="path"
          autosize
          type="textarea"
          placeholder="File Path"
          style="widht: 50%"
        />
        <el-row :gutter="10">
          <el-col :span="12"
            ><div class="grid-content ep-bg-purple" />
            <el-button style="width: 100%" @click="chooseFiles"
              >Choose Files</el-button
            ></el-col
          >
          <el-col :span="12"
            ><div class="grid-content ep-bg-purple" />
            <el-button style="width: 100%" @click="chooseDirectory"
              >Choose Directory</el-button
            ></el-col
          >
        </el-row>
        <el-row :gutter="10">
          <el-col :span="12"
            ><el-button style="width: 100%" @click="send" type="primary" text bg
              >Send</el-button
            >
          </el-col>
          <el-col :span="12"
            ><el-button
              type="primary"
              text
              bg
              style="width: 100%"
              @click="sendText"
              >Send Text</el-button
            ></el-col
          >
        </el-row>
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

        <FiledropZone :file-drag-on-hover="fileDropHoverFlag" />
      </div>

      <div class="flex-grow flex justify-center items-end">
        <small class="">Drop a File to Upload</small>
      </div>
    </div>
  </NuxtLayout>
</template>
<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { ElInput } from "element-plus";
import { CopyDocument } from "@element-plus/icons-vue";
import { writeText } from "@tauri-apps/api/clipboard";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

let fileDropHoverUnlisten: UnlistenFn;
let fileDropUnlisten: UnlistenFn;
let fileDropCancelledUnlisten: UnlistenFn;
let fileDropHoverFlag = ref(false);

onMounted(async () => {
  fileDropHoverUnlisten = await listen<string[]>(
    "tauri://file-drop-hover",
    (event) => {
      fileDropHoverFlag.value = true;
    }
  );
  fileDropUnlisten = await listen<string[]>("tauri://file-drop", (event) => {
    console.log(event);
    const paths = event.payload;
    if (paths.length > 0) {
      path.value = paths[0];
    }
  });
  fileDropCancelledUnlisten = await listen<string[]>(
    "tauri://file-drop-cancelled",
    (event) => {
      console.log(event);
      fileDropHoverFlag.value = false;
    }
  );
});

onUnmounted(() => {
  console.log("unmounted");
  fileDropHoverUnlisten();
  fileDropUnlisten();
  fileDropCancelledUnlisten();
});

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

async function chooseFiles() {
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
