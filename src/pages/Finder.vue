<script setup lang="ts">
import ToolBar from "../components/ToolBar.vue";
import DragRegion from "../components/DragRegion.vue";
import SideBar from "../components/SideBar.vue";
import FileList from "../components/FileList.vue";

</script>

<template>
  <SideBar>
    <template v-slot:drawer-side>
      <DragRegion></DragRegion>
      <ul class="menu px-4 pt-0 w-48 min-h-full text-base-content">
        <li><a>All</a></li>
        <li><a>Document</a></li>
        <li><a>Movie</a></li>
        <li><a>Comic</a></li>
      </ul>
    </template>
    <template v-slot:drawer-content>
      <ToolBar></ToolBar>

      <Suspense>
        <FileList />

        <template #fallback>
          <div>Loading...</div>
        </template>
      </Suspense>
    </template>
  </SideBar>
</template>