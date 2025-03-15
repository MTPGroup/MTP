<script lang="ts" setup>
import { Icon } from '@iconify/vue'
import { listen } from '@tauri-apps/api/event'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

definePageMeta({
  layout: 'child-window',
})

const colorMode = useColorMode()

onMounted(async () => {
  await listen('user-settings-updated', (event) => {
    const { type, value } = event.payload as { type: string; value: string }
    if (type === 'theme') {
      colorMode.preference = value
    }
  })
})

const closeAboutWindow = async () => {
  console.info('close about window')
  const existingWindow = await WebviewWindow.getByLabel('about')
  if (existingWindow) {
    await existingWindow.hide()
  }
}
</script>

<template>
  <div
    class="flex flex-col items-center justify-center min-h-screen bg-gradient-to-b from-theme-100 to-theme-200"
  >
    <div
      class="w-full max-w-lg h-full bg-white rounded-md shadow-lg overflow-hidden"
    >
      <!-- 头部Logo区域 -->
      <div class="relative bg-theme-500 py-8 px-6 text-center">
        <div class="absolute top-0 right-0 p-2">
          <Button
            variant="ghost"
            size="icon"
            class="h-8 w-8 text-theme-50 hover:bg-theme-400 hover:text-white"
            @click="closeAboutWindow"
          >
            <Icon icon="lucide:x" class="h-4 w-4" />
          </Button>
        </div>

        <div class="mb-3">
          <div
            class="size-24 mx-auto bg-white rounded-full flex items-center justify-center shadow-md"
          >
            <img
              src="../assets/logo.png"
              alt="MTP Logo"
              class="size-20 mx-auto bg-white rounded-full flex items-center justify-center shadow-md"
            />
          </div>
        </div>
        <h1 class="text-2xl font-bold text-white">MomoTalk Plus</h1>
        <p class="text-theme-100 text-sm mt-1">版本 1.0.0</p>
      </div>

      <!-- 内容区域 -->
      <div class="p-6">
        <div class="space-y-6">
          <!-- 应用简介 -->
          <div>
            <h2 class="text-lg font-medium text-gray-900 border-b pb-2 mb-3">
              关于应用
            </h2>
            <p class="text-gray-600">
              MomoTalk Plus (MTP) 是一个专为sensei建立的交流平台，
              结合了最新的人工智能技术，与学生效率互动(大雾)。
            </p>
          </div>

          <!-- 技术栈展示 -->
          <div>
            <h2 class="text-lg font-medium text-gray-900 border-b pb-2 mb-3">
              技术栈
            </h2>
            <div class="grid grid-cols-3 gap-2 text-center">
              <div
                class="p-2 rounded-md bg-gray-50 hover:bg-theme-50 transition-colors"
              >
                <Icon icon="logos:nuxt-icon" class="size-10 mx-auto" />
                <p class="text-sm mt-1">Nuxt 3</p>
              </div>
              <div
                class="p-2 rounded-md bg-gray-50 hover:bg-theme-50 transition-colors"
              >
                <Icon icon="logos:rust" class="size-10 mx-auto" />
                <p class="text-sm mt-1">Rust</p>
              </div>
              <div
                class="p-2 rounded-md bg-gray-50 hover:bg-theme-50 transition-colors"
              >
                <Icon icon="logos:typescript-icon" class="size-10 mx-auto" />
                <p class="text-sm mt-1">TypeScript</p>
              </div>
              <div
                class="p-2 rounded-md bg-gray-50 hover:bg-theme-50 transition-colors"
              >
                <Icon icon="logos:tailwindcss-icon" class="size-10 mx-auto" />
                <p class="text-sm mt-1">Tailwind CSS</p>
              </div>
              <div
                class="p-2 rounded-md bg-gray-50 hover:bg-theme-50 transition-colors"
              >
                <Icon icon="logos:tauri" class="size-10 mx-auto" />
                <p class="text-sm mt-1">Tauri</p>
              </div>
              <div
                class="p-2 rounded-md bg-gray-50 hover:bg-theme-50 transition-colors"
              >
                <Icon icon="simple-icons:openai" class="size-10 mx-auto" />
                <p class="text-sm mt-1">AI Integration</p>
              </div>
            </div>
          </div>

          <!-- 开发团队 -->
          <div>
            <h2 class="text-lg font-medium text-gray-900 border-b pb-2 mb-3">
              开发团队
            </h2>
            <div class="flex items-center space-x-3 my-3">
              <Avatar class="size-12 shadow-md border border-theme-300">
                <AvatarImage
                  src="https://cravatar.cn/avatar/2652ab380725cbe9e65da5222a7a9efe"
                />
                <AvatarFallback>S</AvatarFallback>
              </Avatar>
              <div>
                <p class="font-medium text-black">hanasaki</p>
                <p class="text-sm text-gray-500">主要开发者</p>
              </div>
            </div>
          </div>

          <!-- 链接 -->
          <div class="flex justify-center space-x-4 pt-2">
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger>
                  <NuxtLink
                    to="https://github.com/MTPGroup/MTP"
                    target="_blank"
                  >
                    <Button variant="outline" class="group">
                      <Icon
                        icon="lucide:github"
                        class="mr-2 h-4 w-4 group-hover:text-black"
                      />
                      <span>GitHub</span>
                    </Button>
                  </NuxtLink>
                </TooltipTrigger>
                <TooltipContent>
                  <p>https://github.com/hanasa2023/MTP</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger>
                  <Button variant="outline" class="group">
                    <Icon
                      icon="lucide:book-open"
                      class="mr-2 h-4 w-4 group-hover:text-theme-500"
                    />
                    <span>文档</span>
                  </Button>
                </TooltipTrigger>
                <TooltipContent>
                  <p>暂无文档</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div>
        </div>
      </div>

      <!-- 底部 -->
      <div class="bg-gray-50 py-3 px-6 text-center border-t">
        <p class="text-xs text-gray-500">
          © {{ new Date().getFullYear() }} MomoTalk Plus. 保留所有权利。
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.bg-gradient-to-b {
  background-size: 100% 100%;
  background-position: 0px 0px;
}
</style>
