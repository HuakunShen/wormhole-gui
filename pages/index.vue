<template>
  <NuxtLayout>
    <el-button size="large" type="primary" link>Send</el-button>
    <el-button size="large" type="success" link>Receive</el-button>
  </NuxtLayout>
</template>
<script setup lang="ts">
const router = useRouter();

router.push({ path: "/send" });
</script>
