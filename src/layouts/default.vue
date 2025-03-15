<script lang="ts" setup>
import { Icon } from '@iconify/vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
</script>

<template>
  <!-- Dragger bar -->
  <!--<div
    class="h-7 white flex fixed top-0 left-0 right-0 justify-center items-center app-region-drag"
  ></div>-->
  <div
    data-tauri-drag-region
    class="h-7 bg-transparent select-none flex justify-end fixed top-0 left-0 right-0 z-50"
  >
    <div class="titlebar-button" @click="appWindow.minimize()">
      <Icon icon="mdi:window-minimize" class="size-4" />
    </div>
    <div class="titlebar-button" @click="appWindow.toggleMaximize()">
      <Icon icon="mdi:window-maximize" class="size-4" />
    </div>
    <div class="titlebar-button" @click="appWindow.close()">
      <Icon icon="mdi:window-close" class="size-4" />
    </div>
  </div>

  <!-- Main Page -->
  <div class="h-screen w-screen flex">
    <Toaster />
    <!-- Page -->
    <!-- Sidebar -->
    <AppSidebar />
    <slot />
  </div>
</template>
