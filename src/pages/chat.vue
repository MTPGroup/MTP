<script setup lang="ts">
import { Icon } from '@iconify/vue'

const dialogOpen = useState(() => false)

const conversationStore = useConversationStore()
const activeId = computed(() => conversationStore.activeConversationId)
const currentConversations = computed(() => conversationStore.conversations)
const newTitle = useState(() => '')
const newStudentName = useState(() => '')

const createNewChat = () => {
  // conversationStore.createConversation()
  dialogOpen.value = false
}

definePageMeta({
  middleware: ['conversation', 'user-settings'],
})
</script>

<template>
  <ResizablePanelGroup direction="horizontal">
    <ResizablePanel :default-size="28" class="max-w-80 min-w-40">
      <div class="flex flex-col h-full bg-theme-200 pt-7">
        <!-- Chat Sidebar top -->
        <div class="flex items-center justify-between px-2 pb-3">
          <ISearchbar />

          <div class="pr-2">
            <Dialog v-model:open="dialogOpen">
              <DialogTrigger as-child>
                <Button
                  variant="secondary"
                  size="icon"
                  class="size-7 bg-theme-400"
                >
                  <Icon
                    icon="tdesign:chat-bubble-add"
                    class="size-7 text-muted-foreground"
                  />
                </Button>
              </DialogTrigger>
              <DialogContent class="sm:max-w-[425px]">
                <DialogHeader>
                  <DialogTitle>创建新对话</DialogTitle>
                </DialogHeader>
                <div class="grid gap-4 py-4">
                  <div class="grid grid-cols-4 items-center gap-4">
                    <Label for="title" class="text-right"> 标题 </Label>
                    <Input id="title" :value="newTitle" class="col-span-3" />
                  </div>
                  <div class="grid grid-cols-4 items-center gap-4">
                    <Label for="username" class="text-right"> 名称 </Label>
                    <Input
                      id="username"
                      :value="newStudentName"
                      class="col-span-3"
                    />
                  </div>
                </div>
                <DialogFooter>
                  <Button type="submit" @click="createNewChat"> 确认 </Button>
                </DialogFooter>
              </DialogContent>
            </Dialog>
          </div>
        </div>
        <!-- Chat Sidebar main -->
        <ScrollArea class="flex-1 bg-theme-200">
          <div class="flex flex-col">
            <div
              v-for="(conversation, index) in currentConversations"
              :key="conversation.id"
            >
              <ChatListItem :conversation="conversation" />
              <Separator v-if="currentConversations.length !== index" />
            </div>
          </div>
        </ScrollArea>
      </div>
    </ResizablePanel>
    <ResizableHandle />
    <ResizablePanel :default-size="72">
      <NuxtPage v-if="activeId" />
      <div
        v-else
        class="flex w-full h-full items-center justify-center bg-theme-100"
      >
        <Icon icon="icon-park-solid:peach" class="size-24 text-icon" />
      </div>
    </ResizablePanel>
  </ResizablePanelGroup>
</template>
