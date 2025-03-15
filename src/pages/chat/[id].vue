<script lang="ts" setup>
import { Icon } from '@iconify/vue'
import type { Message } from '~/models/message'
import { tauriService } from '~/services/tauri'

const dialogOpen = useState(() => false)
const conversationStore = useConversationStore()
const route = useRoute()
const messages = useState<Message[]>(() => [])
const isLoading = useState(() => false)
const isLoadingResponse = useState(() => false)
const inputText = useState(() => '')
const newTitle = useState(() => '')
const { scrollToBottom, isNearBottom } = useScrollToBottom()

const changeTitle = async () => {
  await tauriService.updateConversation(route.params.id.toString(), {
    title: newTitle.value,
  })
  newTitle.value = ''
  dialogOpen.value = false
}

// 监听路由参数变化，加载对应对话
watch(
  () => route.params.id,
  async (newId) => {
    if (newId) {
      await loadConversation(newId.toString())
      scrollToBottom(false)
    }
  },
  { immediate: true }
)

// 加载对话历史
async function loadConversation(conversationId: string) {
  isLoading.value = true
  try {
    // 通过API获取对话历史
    const response = await tauriService.getMessagesByConversationId(
      conversationId
    )
    if (response) {
      messages.value = response
    }
  } catch (error) {
    console.error('加载对话历史失败:', error)
  } finally {
    isLoading.value = false
  }
}

// 监听消息变化，自动滚动到底部
watch(
  () => messages.value.length,
  () => {
    // 只有当用户接近底部自动滚动

    if (isNearBottom()) {
      nextTick(() => {
        scrollToBottom(true)
      })
    }
  }
)

// 发送消息
async function sendMessage(text: string) {
  if (!text.trim() || !route.params.id) return

  const conversationId = route.params.id.toString()
  const humanMessage: Message = { role: 'user', content: text }

  // 先添加人类消息到UI
  messages.value.push(humanMessage)
  inputText.value = ''

  // 添加临时的加载消息
  const loadingMessageIndex = messages.value.length
  messages.value.push({ role: 'assistant', content: '...' })
  isLoadingResponse.value = true
  // 滚动到底部
  nextTick(() => {
    scrollToBottom(true)
  })

  try {
    // 发送消息到API并获取回复
    const data = await tauriService.chatWithLLM(
      { role: 'user', content: text },
      conversationId
    )

    if (data) {
      // 添加AI回复到UI
      messages.value[loadingMessageIndex] = {
        role: 'assistant',
        content: data.content,
      }
      conversationStore.conversations.forEach((conversation) => {
        if (conversation.id === conversationId) {
          conversation.lastMessage = data.content
        }
      })
      conversationStore.sortConversations()
      nextTick(() => {
        scrollToBottom(true)
      })
    }
  } catch (error) {
    messages.value.pop()
    messages.value.push({
      role: 'assistant',
      content: '服务器繁忙，请稍后重试(x)',
    })
    console.error('发送消息失败:', error)
  } finally {
    isLoadingResponse.value = false
  }
}
</script>

<template>
  <div class="flex flex-col h-full bg-blue-100">
    <!-- 头部信息 -->
    <div
      class="flex h-[68px] items-start justify-between px-4 backdrop-blur-lg bg-blue-300 pt-7"
    >
      <p class="text-lg">
        {{ conversationStore.activeConversation?.title || '聊天' }}
      </p>
      <!-- 按钮组 -->
      <div class="flex items-center justify-center">
        <Dialog v-model:open="dialogOpen">
          <DialogTrigger as-child>
            <IButton>
              <Icon icon="ic:edit" class="size-6 text-muted-foreground" />
            </IButton>
          </DialogTrigger>
          <DialogContent class="sm:max-w-[425px]">
            <DialogHeader>
              <DialogTitle>更改对话标题</DialogTitle>
            </DialogHeader>
            <div class="grid gap-4 py-4">
              <div class="grid grid-cols-4 items-center gap-4">
                <Label for="title" class="text-right"> 新标题 </Label>
                <Input id="title" :value="newTitle" class="col-span-3" />
              </div>
            </div>
            <DialogFooter>
              <Button type="submit" @click="changeTitle"> 确认</Button>
            </DialogFooter>
          </DialogContent>
        </Dialog>
        <IButton @click="conversationStore.closeConversation()">
          <Icon icon="ci:exit" class="size-6 text-muted-foreground" />
        </IButton>
        <IButton>
          <Icon
            icon="ic:round-more-horiz"
            class="size-6 text-muted-foreground"
          />
        </IButton>
      </div>
    </div>

    <div class="flex flex-1">
      <ResizablePanelGroup direction="vertical">
        <!-- 聊天内容区域 -->
        <ResizablePanel :default-size="62">
          <ScrollArea class="h-full px-4 pb-2 py-1">
            <div v-if="isLoading" class="flex justify-center p-4">
              <CircleSpinner :size="32" color="#93c5fd" :thickness="3" />
            </div>
            <div
              v-else-if="messages.length === 0"
              class="flex justify-center items-center h-full"
            >
              <p class="text-muted-foreground">没有消息记录，开始聊天吧</p>
            </div>
            <TransitionGroup
              v-else
              name="message-transition"
              id="scroll-area-viewport"
              tag="div"
              class="flex flex-col gap-4"
            >
              <ChatContent
                v-for="(message, i) in messages"
                :key="i"
                :content="message.content || ''"
                :isUser="message.role === 'user'"
              />
            </TransitionGroup>
          </ScrollArea>
        </ResizablePanel>

        <ResizableHandle />

        <!-- 输入区域 -->
        <ResizablePanel :default-size="38" class="min-h-[148px] max-h-80">
          <ChatInputArea
            v-model="inputText"
            @send="sendMessage"
            :is-loading-response="isLoadingResponse"
          />
        </ResizablePanel>
      </ResizablePanelGroup>
    </div>
  </div>
</template>

<style scoped>
.message-transition-enter-active,
.message-transition-leave-active {
  transition: all 0.35s ease;
}

.message-transition-enter-from {
  opacity: 0;
  transform: translateY(20px);
}

.message-transition-leave-to {
  opacity: 0;
}
</style>
