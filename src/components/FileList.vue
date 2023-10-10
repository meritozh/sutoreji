<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

let file_list = ref<String[]>(await invoke("get_dir_list", { path: "./" }));
</script>

<template>
  <div v-for="file in file_list">
    <div class="btn">{{ file }}</div>
  </div>
</template>