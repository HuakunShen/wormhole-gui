<template>
  <div class="flex items-center justify-center w-full">
    <label for="dropzone-file" :class="onHoverClasses">
      <div class="flex flex-col items-center justify-center pt-5 pb-6">
        <svg
          aria-hidden="true"
          class="w-10 h-10 mb-3 text-gray-400"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
          xmlns="http://www.w3.org/2000/svg"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"
          ></path>
        </svg>
        <p class="mb-2 text-sm text-gray-500 dark:text-gray-400">
          <span class="font-semibold">{{ $t('dragNDrop') }}</span>
        </p>
      </div>
    </label>
  </div>
</template>
<script setup lang="ts">
const props = defineProps<{
  fileDragOnHover: boolean;
}>();
const onHoverClasses = computed(() => {
  let baseClassNames =
    "flex-col items-center justify-center w-full border-2 border-dashed rounded-lg bg-transparent border-zinc-300 dark:border-zinc-700 hover:invert";
  const resultClassNames =
    (props.fileDragOnHover ? "invert " : "") + baseClassNames;
  return resultClassNames;
});
</script>
