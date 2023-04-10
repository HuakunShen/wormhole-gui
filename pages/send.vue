<template>
  <NuxtLayout>
    <div class="flex flex-col h-full justify-end">
      <div class="grid grid-cols-1 gap-3">
        <el-progress v-if="progress" :percentage="progress" />
        <el-input
          v-model="path"
          autosize
          type="textarea"
          :placeholder="$t('filePath')"
          style="widht: 50%"
        />
        <el-row :gutter="10">
          <el-col :span="12"
            ><div class="grid-content ep-bg-purple" />
            <el-button style="width: 100%" @click="chooseFile">{{
              $t("chooseFile")
            }}</el-button></el-col
          >
          <el-col :span="12"
            ><div class="grid-content ep-bg-purple" />
            <el-button style="width: 100%" @click="chooseDirectory">{{
              $t("chooseFolder")
            }}</el-button></el-col
          >
        </el-row>
        <el-row :gutter="10">
          <el-col :span="12"
            ><el-button
              style="width: 100%"
              @click="send"
              type="primary"
              text
              bg
              >{{ $t("send") }}</el-button
            >
          </el-col>
          <el-col :span="12"
            ><el-button
              type="primary"
              text
              bg
              style="width: 100%"
              @click="sendText"
              >{{ $t("sendText") }}</el-button
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
        <small class="">{{ $t("dropFileToUpload") }}</small>
      </div>
    </div>
  </NuxtLayout>
</template>
<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { invoke } from "@tauri-apps/api/tauri";
import { ElInput } from "element-plus";
import { CopyDocument } from "@element-plus/icons-vue";
import { writeText } from "@tauri-apps/api/clipboard";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

let fileDropHoverUnlisten: UnlistenFn;
let fileDropUnlisten: UnlistenFn;
let fileDropCancelledUnlisten: UnlistenFn;
let wormholeCodeUnlisten: UnlistenFn;
let sendProgressUnlisten: UnlistenFn;

const fileDropHoverFlag = ref(false);
const progress = ref<undefined | number>(undefined);
const path = ref("");
const textarea = ref("");
const receiveCode = ref<string>("");

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
  wormholeCodeUnlisten = await listen<{ code: string }>(
    "wormhole://receive-code",
    (event) => {
      console.log(event);
      receiveCode.value = event.payload.code;
      writeText(receiveCode.value).then(() => {
        ElMessage({
          message: `Receive Code Saved to Clipboard`,
          type: "success",
        });
      });
    }
  );
  sendProgressUnlisten = await listen<{ sent: number; total: number }>(
    "wormhole://progress",
    (event) => {
      console.log(event);
      const { sent, total } = event.payload;
      progress.value = (sent / total) * 100;
    }
  );
});

onUnmounted(() => {
  fileDropHoverUnlisten();
  fileDropUnlisten();
  fileDropCancelledUnlisten();
  wormholeCodeUnlisten();
  sendProgressUnlisten();
});

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
  if (path.value.length === 0) {
    ElMessage({
      type: "error",
      message: `Please select a file`,
    });
    return;
  }

  invoke("send", { filepath: path.value })
    .then(() => {
      receiveCode.value = "";
      ElMessage({
        type: "success",
        message: `Finished`,
      });
    })
    .catch((err) => {
      console.log(err);
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
