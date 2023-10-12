<script setup lang="ts">
import ToolItem from "./ToolItem.vue";
import DragRegion from "./DragRegion.vue";
</script>

<template>
  <div>
    <div class="px-4 h-16 w-full flex justify-between items-center ">
      <div class="join join-horizontal bg-base-300 z-10">
        <ToolItem>1</ToolItem>
        <ToolItem>2</ToolItem>
      </div>
      <div class="join join-horizontal bg-base-300 z-10">
        <ToolItem>1</ToolItem>
        <ToolItem>2</ToolItem>
        <ToolItem>3</ToolItem>
      </div>
      
      <DragRegion class="absolute top-0"></DragRegion>
    </div>
  </div>
</template>