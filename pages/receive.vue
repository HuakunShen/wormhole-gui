<template>
  <NuxtLayout>
    <div class="grid grid-cols-1 gap-3">
      <div class="flex" v-if="progress">
        <el-progress class="grow" :percentage="progress" />
        <div class="flex-none">
          {{ `${progressSizeStr} [${mbps ?? 0}MB/s]` }}
        </div>
      </div>
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
        <br />
        <div v-for="(path, idx) in receivePaths" :key="idx" class="text item">
          <el-button
            class="mr-2"
            :icon="FolderOpened"
            circle
            @click="openPath(path)"
          />{{ path }}
        </div>
      </el-form>
    </div>
  </NuxtLayout>
</template>
<script setup lang="ts">
import { List as ListIcon } from "@element-plus/icons-vue";
import { readText } from "@tauri-apps/api/clipboard";
import { open } from "@tauri-apps/api/shell";
import { invoke } from "@tauri-apps/api/tauri";
import { downloadDir, dirname } from "@tauri-apps/api/path";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { FolderOpened } from "@element-plus/icons-vue";
import bytes from "bytes";

let sendProgressUnlisten: UnlistenFn;
const receivePaths = ref<string[]>([]);
const downloadDirPath = await downloadDir();
const progress = ref<undefined | number>(undefined);
const mbps = ref(0);
let receiveStartTime: undefined | number = undefined;
let progressSizeStr = "/";

onMounted(async () => {
  sendProgressUnlisten = await listen<{ sent: number; total: number }>(
    "wormhole://progress",
    (event) => {
      if (!receiveStartTime) {
        receiveStartTime = Date.now();
      }
      const { sent, total } = event.payload;
      progress.value = Math.round((sent / total) * 100);
      const timeUsed = Math.round((Date.now() - receiveStartTime) / 1000);
      const megabytesSent = sent / 1000000;
      progressSizeStr = `${bytes(sent)}/${bytes(total)}`;
      mbps.value = Math.round((megabytesSent * 100) / timeUsed) / 100;
    }
  );
});

onUnmounted(() => {
  sendProgressUnlisten();
});

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

async function openPath(path: string) {
  // open(path)
  const baseDir = await dirname(path);
  open(baseDir);
}

function receive() {
  const receiveCode = form.receiveCode.trim();

  if (receiveCode.length !== 0) {
    receiveStartTime = Date.now();
    invoke<string>("receive", {
      receiveCode: receiveCode,
      saveDir: downloadDirPath,
    })
      .then((savePath: string) => {
        receivePaths.value.push(savePath);
        ElNotification({
          title: "Success",
          message: "Received File",
          type: "success",
          position: "bottom-right",
        });
        receiveStartTime = undefined;
      })
      .catch((err) => {
        ElNotification({
          title: "Receive Code",
          message: err,
          type: "error",
          position: "bottom-right",
        });
        receiveStartTime = undefined;
      });
  }
}
</script>
