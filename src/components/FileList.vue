<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

let file_list = ref<String[]>(await invoke("get_dir_list", { path: "./" }));
</script>

<template>
  <div class="divide-y divide-dashed">
    <div v-for="file in file_list">
      <div class="m-2">{{ file }}</div>
    </div>
  </div>
</template>