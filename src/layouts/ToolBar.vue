<template>
  <div class="border-b border-base-300">
    <div class="px-4 h-16 w-full flex justify-between items-center">
      <div class="join join-horizontal bg-base-300 z-10">
        <slot name="left-group"></slot>
      </div>
      <div class="join join-horizontal bg-base-300 z-10">
        <slot name="right-group"></slot>
      </div>
    </div>
  </div>
</template>