<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

let file_list = ref<String[]>(await invoke("get_dir_list"));
</script>

<template>
  <div class="overflow-auto scroll-auto divide-y divide-solid">
    <div v-for="file in file_list">
      <div class="m-2 px-2">{{ file }}</div>
    </div>
  </div>
</template>