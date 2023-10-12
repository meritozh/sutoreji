<script setup lang="ts">
import Finder from "./pages/Finder.vue";
import Launch from "./pages/Launch.vue";

import DragRegion from "./components/DragRegion.vue";

import { ref } from "vue";

const showLaunch = ref(false);
</script>

<template>
  <template v-if="showLaunch">
    <Launch></Launch>
  </template>
  
  <template v-else>
    <Finder></Finder>
  </template>

  <DragRegion data-tauri-drag-region class="absolute top-0"></DragRegion>
</template>