<script lang="ts" setup>
import hljs from 'highlight.js'
import MarkdownIt from 'markdown-it'
import { computed } from 'vue'
// 可选: 导入highlight.js的样式
import 'highlight.js/styles/github.css'

const props = defineProps({
  content: {
    type: String,
    required: true,
  },
  isUser: {
    type: Boolean,
    default: false,
  },
})

const userStore = useMyUserInfoStore()

const isLoading = computed(() => props.content === '...')

// 创建markdown-it实例并配置
const md = new MarkdownIt({
  html: false, // 禁用HTML标签以防止XSS
  linkify: true, // 自动转换URL文本为链接
  typographer: true, // 启用一些语言中性的替换和引号
  highlight: function (str: string, lang: string) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return hljs.highlight(str, { language: lang }).value
      } catch (__) {}
    }
    return '' // 使用默认转义
  },
})

// 渲染markdown内容
const renderedContent = computed(() => {
  if (isLoading.value || !props.content) return ''
  return md.render(props.content)
})

// 自动获取学生头像或使用默认头像
const conversationStore = useConversationStore()
const studentAvatar = computed(() => {
  if (!props.isUser) {
    return conversationStore.activeConversation?.student?.avatars
      ? JSON.parse(conversationStore.activeConversation.student.avatars)[0]
      : ''
  }
  return userStore.userAvatar // 用户的默认头像
})
</script>

<template>
  <div
    class="flex w-full gap-3 rounded-lg"
    :class="{
      'justify-end': isUser,
    }"
  >
    <!-- 学生的头像显示在左侧 -->
    <div v-if="!isUser" class="flex-shrink-0">
      <div
        class="size-10 rounded-full bg-theme-200 overflow-hidden flex items-center justify-center"
      >
        <Avatar>
          <AvatarImage :src="studentAvatar" />
          <AvatarFallback>Avatar</AvatarFallback>
        </Avatar>
      </div>
    </div>

    <!-- 消息内容 -->
    <div
      class="py-2 px-3 rounded-lg max-w-[85%] transition-all duration-300 markdown-content"
      :class="{
        'animate-pulse': isLoading,
        'bg-white text-gray-900': !isUser,
        'bg-theme-700 text-white': isUser,
        'opacity-70': isLoading,
      }"
    >
      <p class="whitespace-pre-wrap break-words" v-if="isLoading">
        <span class="inline-block animate-bounce">.</span>
        <span class="inline-block animate-bounce" style="animation-delay: 0.2s"
          >.</span
        >
        <span class="inline-block animate-bounce" style="animation-delay: 0.4s"
          >.</span
        >
      </p>
      <!-- 使用v-html渲染Markdown -->
      <div v-else v-html="renderedContent" class="markdown-body"></div>
    </div>

    <!-- 用户消息的头像显示在右侧 -->
    <div v-if="isUser" class="flex-shrink-0">
      <div
        class="size-10 rounded-full bg-theme-100 overflow-hidden flex items-center justify-center"
      >
        <Avatar>
          <AvatarImage :src="studentAvatar" />
          <AvatarFallback>Avatar</AvatarFallback>
        </Avatar>
      </div>
    </div>
  </div>
</template>
