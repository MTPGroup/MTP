import { invoke } from '@tauri-apps/api/core'
import type {
  Conversation,
  ConversationWithStudent,
} from '~/models/conversation'
import type { Message } from '~/models/message'

export class TauriService {
  async setStore(key: string, value: any): Promise<void> {
    await invoke('set_store', { key, value })
  }

  async getConversations(): Promise<ConversationWithStudent[]> {
    const conversations = await invoke<ConversationWithStudent[]>(
      'get_conversations'
    )
    return conversations
  }

  async getConversationById(
    id: string
  ): Promise<ConversationWithStudent | null> {
    const conversation = await invoke<ConversationWithStudent | null>(
      'get_conversation_by_id',
      { id }
    )
    if (!conversation) return null

    return conversation
  }

  async createConversation(data: {
    title?: string
    studentName: string
  }): Promise<Conversation> {
    return await invoke<Conversation>('create_conversation', { data })
  }

  async updateConversation(
    id: string,
    data: { title?: string }
  ): Promise<Conversation> {
    return await invoke<Conversation>('update_conversation', { id, data })
  }

  async deleteConversation(id: string): Promise<Conversation> {
    return await invoke<Conversation>('delete_conversation', { id })
  }

  // 添加消息相关方法
  async getMessagesByConversationId(
    conversationId: string
  ): Promise<Message[]> {
    return await invoke<Message[]>('get_messages_by_conversation_id', {
      conversationId,
    })
  }

  async getMessagesByConversationIdWithPagination(
    conversationId: string,
    page: number,
    pageSize: number
  ): Promise<Message[]> {
    return await invoke<Message[]>(
      'get_messages_by_conversation_id_with_pagination',
      {
        conversationId,
        page,
        pageSize,
      }
    )
  }

  async createMessage(data: {
    conversationId: string
    role: string
    content: string
    name?: string
    index?: number
  }): Promise<Message> {
    return await invoke<Message>('create_message', { data })
  }

  // 添加 AI 聊天功能
  async chatWithLLM(
    message: Message,
    conversationId: string
  ): Promise<Message> {
    return await invoke<Message>('chat_with_llm', {
      message,
      conversationId: conversationId,
    })
  }
}

export const tauriService = new TauriService()
