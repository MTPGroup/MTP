<script lang="ts" setup>
import {Icon} from '@iconify/vue'
import {getCurrentWindow} from '@tauri-apps/api/window'

// const closeWindow = async () => {
//   const aboutWindow = getCurrentWindow()
//   console.info(aboutWindow)
//   await aboutWindow.hide()
// }
</script>

<template>
  <div>
    <!-- Dragger bar -->
    <Toaster/>
    <div
        data-tauri-drag-region
        class="h-7 bg-transparent select-none flex justify-end fixed top-0 left-0 right-0 z-50"
    >
    </div>
    <slot/>
  </div>
</template>
