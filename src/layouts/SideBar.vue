<script setup lang="ts">
import DragRegion from "../components/DragRegion.vue";
</script>

<template>
  <div class="drawer drawer-open">
    <input id="sidebar" type="checkbox" class="drawer-toggle" />
    <div class="drawer-content">
      <slot name="drawer-content"></slot>
    </div>
    <div class="drawer-side border-r bg-amber-300">
      <DragRegion></DragRegion>
      <slot name="drawer-side"></slot>
    </div>
  </div>
</template>