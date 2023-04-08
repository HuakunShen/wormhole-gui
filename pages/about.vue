<template>
  <NuxtLayout>
    <div>
      <h3>About</h3>
      <p>
        This is a GUI for
        <el-link @click="open(wormholeLink)">Magic Wormhole</el-link>, based on
        <el-link @click="open(wormholeRSLink)">magic-wormhole.rs</el-link>.
      </p>
    </div>
    <div>
      <h3>Source Code</h3>
      <el-link style="margin-right: 0.5rem" @click="open(sourceCodeLink)">{{
        sourceCodeLink
      }}</el-link>
      <el-button
        @click="copyLink(sourceCodeLink)"
        :icon="CopyDocument"
        circle
      />
    </div>
    <div>
      <h3>Author</h3>
      <el-link style="margin-right: 0.5rem" @click="open(authorLink)"
        >Huakun</el-link
      >
      <el-button @click="copyLink(authorLink)" :icon="CopyDocument" circle />
    </div>
  </NuxtLayout>
</template>
<script setup lang="ts">
import { writeText } from "@tauri-apps/api/clipboard";
import { CopyDocument } from "@element-plus/icons-vue";
import { open } from "@tauri-apps/api/shell";

const authorLink = ref("https://github.com/HuakunShen");
const sourceCodeLink = ref("https://github.com/HuakunShen/wormhole-gui");
const wormholeLink = "https://github.com/magic-wormhole/magic-wormhole";
const wormholeRSLink = "https://github.com/magic-wormhole/magic-wormhole.rs";

function copyLink(link: string) {
  writeText(link).then(() => {
    ElMessage({
      message: "Copied Link to Clipboard",
      grouping: true,
      type: "success",
    });
  });
}
</script>
