<template>
  <NuxtLayout>
    <el-descriptions class="margin-top" :column="1" :border="true">
      <el-descriptions-item>
        <template #label>
          <div class="cell-item">
            <el-icon>
              <user />
            </el-icon>
            {{ $t("theme") }}
          </div>
        </template>
        <el-switch
          v-model="darkChecked"
          :active-icon="Moon"
          :inactive-icon="Sunny"
        />
      </el-descriptions-item>

      <el-descriptions-item>
        <template #label>
          <div class="cell-item">
            <el-icon>
              <user />
            </el-icon>
            {{ $t("language") }}
          </div>
        </template>
        <Locale />
      </el-descriptions-item>

      <el-descriptions-item>
        <template #label>
          <div class="cell-item">
            <el-icon>
              <user />
            </el-icon>
            {{ $t("defaultReceiveDir") }}
          </div>
        </template>
        <!-- <el-button @click="chooseDir" type="primary" text bg
          >Choose Default Directory</el-button
        > -->
        <el-input
          v-model="defaultReceiveDir"
          placeholder="Please input"
          class="input-with-select"
        >
          <template #append>
            <el-button @click="chooseDir" :icon="Search" />
          </template>
        </el-input>
      </el-descriptions-item>
    </el-descriptions>
  </NuxtLayout>
</template>
<script setup lang="ts">
import { useDark, useToggle } from "@vueuse/core";
import { open } from "@tauri-apps/api/dialog";
import { Search, Sunny, Moon } from "@element-plus/icons-vue";

const isDark = useDark();
const defaultReceiveDir = ref("abc");
const darkChecked = ref(isDark.value);
const toggleDark = useToggle(isDark);

async function chooseDir() {
  const selected = await open({
    directory: true,
  });
  if (selected instanceof Array && selected.length > 0) {
    defaultReceiveDir.value = selected[0];
  } else if (selected) {
    defaultReceiveDir.value = selected as string;
  }
}

watch(darkChecked, (newTheme) => {
  toggleDark(newTheme);
});
</script>
