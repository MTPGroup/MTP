import { defineStore } from 'pinia'
import type { ConversationWithStudent } from '~/models/conversation'
import { tauriDbService } from '~/services/tauri-database'

export const useConversationStore = defineStore('conversation', () => {
  const conversations = useState<ConversationWithStudent[]>(() => [])
  const activeConversationId = useState<string | null>(() => null)
  const loading = useState(() => false)
  const streaming = useState(() => false)

  const activeConversation = computed(() => {
    if (!activeConversationId.value) return null
    return (
      conversations.value.find(
        (conv) => conv.id === activeConversationId.value
      ) || null
    )
  })

  const setActiveConversation = (id: string) => {
    activeConversationId.value = id
    navigateTo(`/chat/${id}`)
  }

  const closeConversation = () => {
    activeConversationId.value = null
  }

  async function loadConversations() {
    try {
      loading.value = true
      // const response = await $fetch<ExtendedConversation[]>(
      //   '/api/conversations'
      // )
      const response = await tauriDbService.getConversations()

      conversations.value = response
      for (const conversation of conversations.value) {
        const data = await tauriDbService.getMessagesByConversationId(conversation.id)
        if (data.length > 0)
          conversation.lastMessage = data[data.length - 1].content
      }
      sortConversations()
    } catch (error) {
      console.error('Failed to load conversations:', error)
    } finally {
      loading.value = false
    }
  }

  async function createConversation() {
    try {
      loading.value = true
      const newConversation = await tauriDbService.createConversation({
        title: 'New Chat',
        studentName: 'AI',
      })
      const conversationWithDateObjects = {
        ...newConversation,
        createdAt: newConversation.createdAt,
        updatedAt: newConversation.updatedAt,
      }
      conversations.value.unshift({
        ...conversationWithDateObjects,
        student_name: conversationWithDateObjects.studentName
      })
      activeConversationId.value = newConversation.id
    } catch (error) {
      console.error('Failed to create conversation:', error)
    } finally {
      loading.value = false
    }
  }

  async function deleteConversation(id: string) {
    try {
      loading.value = true
      await tauriDbService.deleteConversation(id)
      const index = conversations.value.findIndex((conv) => conv.id === id)
      if (index !== -1) {
        conversations.value.splice(index, 1)
      }
      if (activeConversationId.value === id) {
        activeConversationId.value = null
      }
    } catch (error) {
      console.error('Failed to delete conversation:', error)
    } finally {
      loading.value = false
    }
  }

  async function updateConversation(id: string, data: { title?: string }) {
    try {
      loading.value = true
      const updatedConversation = await tauriDbService.updateConversation(id, data)
      const index = conversations.value.findIndex((conv) => conv.id === id)
      if (index !== -1) {
        const updatedConversationWithDateObjects = {
          ...updatedConversation,
          createdAt: updatedConversation.createdAt,
          updatedAt: updatedConversation.updatedAt,
        }
        conversations.value[index] = {
          ...conversations.value[index],
          ...updatedConversationWithDateObjects,
        }
      }
    } catch (error) {
      console.error('Failed to update conversation:', error)
    } finally {
      loading.value = false
    }
  }

  function sortConversations() {
    conversations.value.sort(
      (a: ConversationWithStudent, b: ConversationWithStudent) => {
        // 优先按照是否有最后消息排序（有消息的在前）
        if (a.lastMessage && !b.lastMessage) return -1
        if (!a.lastMessage && b.lastMessage) return 1

        // 如果都有或都没有消息，按更新时间排序（最新的在前）
        return new Date(b.updatedAt).getTime() - new Date(a.updatedAt).getTime()
      }
    )
  }

  return {
    conversations,
    activeConversationId,
    activeConversation,
    setActiveConversation,
    closeConversation,
    sortConversations,
    loading,
    streaming,
    loadConversations,
    createConversation,
    deleteConversation,
    updateConversation,
  }
})
