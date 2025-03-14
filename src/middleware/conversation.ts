export default defineNuxtRouteMiddleware(async () => {
  const conversationStore = useConversationStore()

  if (conversationStore.conversations.length === 0) {
    await conversationStore.loadConversations()
  }
})
