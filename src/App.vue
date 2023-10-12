<script setup lang="ts">
import { ref } from "vue";
import Finder from "./pages/Finder.vue";
import Launch from "./pages/Launch.vue";

const showLaunch = ref(false);
</script>

<template>
  <template v-if="showLaunch">
    <Launch></Launch>
  </template>
  
  <template v-else>
    <Finder></Finder>
  </template>
</template>