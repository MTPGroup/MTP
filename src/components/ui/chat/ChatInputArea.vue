<script setup lang="ts">
import { Icon } from '@iconify/vue'
import { useVModel } from '@vueuse/core'

const props = defineProps<{
  modelValue: string
  isLoadingResponse: boolean
}>()

const emits = defineEmits<{
  (e: 'update:modelValue', value: string): void
  (e: 'send', text: string): void
}>()

const inputText = useVModel(props, 'modelValue', emits)

function handleSend() {
  if (inputText.value.trim()) {
    emits('send', inputText.value)
  }
}

function handleKeyDown(e: KeyboardEvent) {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    handleSend()
  }
}
</script>

<template>
  <div
    class="flex flex-col h-full border-t border-theme-300/30 bg-theme-400/50 backdrop-blur-lg supports-[backdrop-filter]:bg-theme-400/10"
  >
    <div class="flex items-center justify-between px-2 pt-1">
      <div class="flex">
        <IButton>
          <Icon
            icon="mingcute:emoji-line"
            class="size-6 text-muted-foreground"
          />
        </IButton>
        <IButton>
          <Icon
            icon="mingcute:pic-2-line"
            class="size-6 text-muted-foreground"
          />
        </IButton>
      </div>
      <div class="flex">
        <IButton>
          <Icon
            icon="mingcute:history-anticlockwise-line"
            class="size-6 text-muted-foreground"
          />
        </IButton>
      </div>
    </div>
    <div class="flex flex-col h-full">
      <Textarea
        v-model="inputText"
        class="flex h-full shadow-none rounded-none resize-none border-none focus:ring-0 focus:border-none focus-visible:ring-0 focus-visible:ring-offset-0"
        @keydown="handleKeyDown"
      ></Textarea>
      <div class="flex items-center justify-end p-4">
        <Button
          v-if="isLoadingResponse"
          class="h-7 bg-theme-700 disable"
          @click="handleSend"
        >
          <Icon
            icon="mingcute:loading-line"
            class="size-6 text-white animate-spin"
          />
          ...
        </Button>
        <Button
          v-else
          class="h-7 bg-theme-600 hover:bg-theme-700"
          @click="handleSend"
          >发送</Button
        >
      </div>
    </div>
  </div>
</template>
