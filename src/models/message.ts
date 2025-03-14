export interface Message {
  role: 'user' | 'assistant' | 'system' | 'tool'
  content: string
}
