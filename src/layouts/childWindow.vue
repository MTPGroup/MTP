<script lang="ts" setup></script>

<template>
  <div>
    <!-- Dragger bar -->
    <Toaster />
    <div
      data-tauri-drag-region
      class="h-7 bg-transparent select-none flex justify-end fixed top-0 left-0 right-0 z-50"
    ></div>
    <slot />
  </div>
</template>
