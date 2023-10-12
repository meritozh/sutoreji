<script setup lang="ts">
import ToolItem from "../components/ToolItem.vue";
import FileList from "../components/FileList.vue";
import ToolBar from "../layouts/ToolBar.vue";
import SideBar from "../layouts/SideBar.vue";

</script>

<template>
  <SideBar>
    <template v-slot:drawer-side>
      <ul class="menu px-4 pt-0 w-48 min-h-full text-base-content">
        <li><a>All</a></li>
        <li><a>Document</a></li>
        <li><a>Movie</a></li>
        <li><a>Comic</a></li>
      </ul>
    </template>
    <template v-slot:drawer-content>
      <ToolBar>
        <template v-slot:left-group>
          <ToolItem>1</ToolItem>
          <ToolItem>2</ToolItem>
        </template>
        <template v-slot:right-group>
          <ToolItem>3</ToolItem>
          <ToolItem>4</ToolItem>
          <ToolItem>5</ToolItem>
        </template>
      </ToolBar>

      <Suspense>
        <FileList />

        <template #fallback>
          <div>Loading...</div>
        </template>
      </Suspense>
    </template>
  </SideBar>
</template>