<script lang="ts" setup>
import { Icon } from '@iconify/vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

const selectItems = [{ name: '通用', value: 'General' }]
const selected = useState(() => 'General')

definePageMeta({
  layout: 'child-window',
  middleware: 'user-settings',
})
const closeSettingsWindow = async () => {
  console.info('close settings window')
  const existingWindow = await WebviewWindow.getByLabel('settings')
  if (existingWindow) {
    await existingWindow.hide()
  }
}
</script>

<template>
  <div class="h-screen w-screen flex flex-col">
    <div class="flex"></div>
    <ResizablePanelGroup direction="horizontal">
      <ResizablePanel :default-size="28" class="max-w-80 min-w-40">
        <div class="flex flex-col h-full bg-blue-100 p-2">
          <SettingsListItem
            v-for="item in selectItems"
            :key="item.value"
            :title="item.name"
            :isSelected="selected === item.value"
            @selected="selected = item.value"
          />
        </div>
      </ResizablePanel>
      <ResizableHandle />
      <ResizablePanel :default-size="72">
        <div class="relative flex h-full bg-blue-50">
          <div class="absolute top-0 right-0">
            <Button
              variant="ghost"
              size="icon"
              class="text-black hover:bg-blue-100"
              @click="closeSettingsWindow"
            >
              <Icon icon="lucide:x" class="h-4 w-4" />
            </Button>
          </div>
          <div class="flex h-full w-full" v-if="selected === 'General'">
            <GeneralSettingsContent />
          </div>
        </div>
      </ResizablePanel>
    </ResizablePanelGroup>
  </div>
</template>
