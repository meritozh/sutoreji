<script setup lang="ts">
import FileList from "./components/FileList.vue";
</script>

<template>
  <Suspense>
    <FileList />

    <template #fallback>
      <div>Loading...</div>
    </template>
  </Suspense>
</template>