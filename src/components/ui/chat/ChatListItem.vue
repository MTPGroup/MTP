<script setup lang="ts">
import type { ConversationWithStudent } from '~/models/conversation'

const props = defineProps<{
  conversation: ConversationWithStudent
}>()

const conversationStore = useConversationStore()

// 获取最后一条消息
const lastMessage = computed<string>(() => {
  const conversation = conversationStore.conversations.find(
    (conv) => conv.id === props.conversation.id
  )
  return conversation?.lastMessage ?? ''
})

const avatar = computed(() => {
  const avs = JSON.parse(props.conversation.student?.avatars || '[]')
  if (avs.length > 0) {
    return avs[0]
  } else {
    return ''
  }
})

const selected = computed(
  () => conversationStore.activeConversationId === props.conversation.id
)
</script>

<template>
  <div v-if="conversation" :class="`flex py-2 px-4 hover:bg-blue-200 ${selected ? 'bg-blue-200' : ''}`"
    @click="conversationStore.setActiveConversation(conversation.id)">
    <Avatar>
      <AvatarImage :src="avatar" />
      <AvatarFallback>{{ conversation.student?.name?.charAt(0) }}</AvatarFallback>
    </Avatar>
    <div class="flex flex-col ml-2 min-w-0 flex-1">
      <h4 class="font-semibold tracking-tight">
        {{ conversation.student?.name }}
      </h4>
      <div class="flex text-sm text-gray-500 truncate">
        {{ lastMessage }}
      </div>
    </div>
  </div>
</template>
