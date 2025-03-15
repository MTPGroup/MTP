<script setup lang="ts">
import { Icon } from '@iconify/vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { ref } from 'vue'
import { tauriService } from '~/services/tauri'
import { useMyUserInfoStore } from '~/stores/userSettings'
import { useToast } from '../toast'

const { toast } = useToast()
const userStore = useMyUserInfoStore()
const username = useState(() => userStore.userName)
const apiKey = useState(() => userStore.apiKey)
const showApiKey = ref(false)

const saveUsername = async () => {
  console.log('Saving username:', username.value)
  await tauriService.setStore('username', username.value)
  toast({
    title: '设置已保存',
    description: '用户名已更新成功',
  })
  // 通知主窗口更新用户名
  try {
    const mainWindow = await WebviewWindow.getByLabel('main')
    if (mainWindow) {
      // 使用Tauri的emit方法发送事件
      await mainWindow.emit('user-settings-updated', {
        type: 'username',
        value: username.value,
      })
    }
  } catch (error) {
    console.error('Failed to notify main window:', error)
  }
}

const saveApiKey = async () => {
  console.log('Saving API Key')
  await tauriService.setStore('api_key', apiKey.value)
  toast({
    title: '设置已保存',
    description: 'API Key已更新成功',
  })

  try {
    const mainWindow = await WebviewWindow.getByLabel('main')
    if (mainWindow) {
      await mainWindow.emit('user-settings-updated', {
        type: 'apiKey',
        value: apiKey.value,
      })
    }
  } catch (error) {
    console.error('Failed to notify main window:', error)
  }
}

const toggleApiKeyVisibility = () => {
  showApiKey.value = !showApiKey.value
}

const copyApiKey = async () => {
  await navigator.clipboard.writeText(apiKey.value)
  toast({
    title: '复制成功',
    description: 'API Key已复制到剪贴板',
    duration: 2000,
  })
}

// 主题设置
const themeOptions = [
  {
    id: 'light',
    name: '白天模式',
    icon: 'ph:sun-bold',
    description: '明亮的界面风格',
  },
  {
    id: 'dark',
    name: '夜间模式',
    icon: 'ph:moon-bold',
    description: '暗色背景，保护眼睛',
  },
  {
    id: 'system',
    name: '跟随系统',
    icon: 'ph:desktop-bold',
    description: '自动适应系统主题',
  },
]

const currentTheme = useState('theme', () => 'system')

const setTheme = (theme: string) => {
  currentTheme.value = theme
  // 保存主题设置到配置文件或localStorage
  localStorage.setItem('theme', theme)
  // 应用主题
  document.documentElement.setAttribute(
    'data-theme',
    theme === 'system'
      ? window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light'
      : theme
  )
}
</script>

<template>
  <div class="flex h-full p-6 w-full flex-col">
    <h2 class="flex text-2xl font-bold mb-6">通用设置</h2>

    <Separator />

    <ScrollArea class="flex flex-col flex-1 space-y-12 py-6">
      <!-- 个人资料设置 -->
      <div class="space-y-6">
        <h3 class="text-lg font-medium px-2">个人资料</h3>

        <Card>
          <CardContent class="flex items-center gap-6 px-4 py-6">
            <AvatarUpload size="lg" />

            <div class="space-y-4 flex-1">
              <div class="space-y-2">
                <Label for="username">用户名</Label>
                <Input id="username" v-model="username" />
              </div>

              <Button @click="saveUsername">保存</Button>
            </div>
          </CardContent>
        </Card>
      </div>

      <!-- API Key设置 -->
      <div class="space-y-6">
        <h3 class="text-lg font-medium px-2">API设置</h3>

        <Card>
          <CardContent class="px-4 py-6">
            <div class="space-y-4">
              <div class="space-y-2">
                <Label for="apiKey" class="flex justify-between items-center">
                  <span>API Key</span>
                  <div class="flex gap-2">
                    <Button
                      variant="outline"
                      size="sm"
                      @click="toggleApiKeyVisibility"
                      class="h-8 px-2"
                    >
                      <Icon
                        :icon="showApiKey ? 'ph:eye-slash' : 'ph:eye'"
                        class="w-4 h-4 mr-1"
                      />
                      {{ showApiKey ? '隐藏' : '显示' }}
                    </Button>
                    <Button
                      v-if="apiKey"
                      variant="outline"
                      size="sm"
                      @click="copyApiKey"
                      class="h-8 px-2"
                    >
                      <Icon icon="ph:copy" class="w-4 h-4 mr-1" />
                      复制
                    </Button>
                  </div>
                </Label>
                <div class="flex gap-2">
                  <Input
                    id="apiKey"
                    v-model="apiKey"
                    :type="showApiKey ? 'text' : 'password'"
                    placeholder="请输入您的API Key"
                  />
                </div>
                <p class="text-sm text-muted-foreground">
                  API Key用于访问高级功能，请妥善保管
                </p>
              </div>

              <Button @click="saveApiKey">保存</Button>
            </div>
          </CardContent>
        </Card>
      </div>

      <!-- 主题设置 -->
      <div class="space-y-6 pt-6">
        <h3 class="text-lg font-medium px-2">外观设置</h3>

        <Card>
          <CardContent class="grid grid-cols-1 md:grid-cols-3 gap-4 px-2 pt-6">
            <!-- 主题选择卡片 -->
            <div
              v-for="theme in themeOptions"
              :key="theme.id"
              @click="setTheme(theme.id)"
              class="relative overflow-hidden rounded-lg border-2 cursor-pointer transition-all hover:scale-[1.02]"
              :class="[
                currentTheme === theme.id
                  ? 'border-primary shadow-md'
                  : 'border-border hover:border-primary/50',
              ]"
            >
              <!-- 主题预览 -->
              <div class="h-32 w-full relative">
                <!-- 白天模式预览 -->
                <div
                  v-if="theme.id === 'light'"
                  class="absolute inset-0 bg-gradient-to-br from-blue-50 to-blue-200"
                >
                  <div
                    class="absolute right-4 top-4 h-8 w-8 bg-yellow-300 rounded-full"
                  ></div>
                  <div
                    class="absolute left-4 top-16 h-6 w-20 bg-white rounded-md"
                  ></div>
                  <div
                    class="absolute left-4 bottom-4 h-6 w-32 bg-white/80 rounded-md"
                  ></div>
                </div>

                <!-- 夜间模式预览 -->
                <div
                  v-if="theme.id === 'dark'"
                  class="absolute inset-0 bg-gradient-to-br from-gray-800 to-gray-950"
                >
                  <div
                    class="absolute right-4 top-4 h-8 w-8 bg-blue-300 rounded-full"
                  ></div>
                  <div
                    class="absolute left-4 top-16 h-6 w-20 bg-gray-700 rounded-md"
                  ></div>
                  <div
                    class="absolute left-4 bottom-4 h-6 w-32 bg-gray-700/80 rounded-md"
                  ></div>
                </div>

                <!-- 跟随系统预览 -->
                <div v-if="theme.id === 'system'" class="absolute inset-0">
                  <div
                    class="absolute left-0 w-1/2 h-full bg-gradient-to-r from-blue-50 to-white"
                  >
                    <div
                      class="absolute right-2 top-4 h-6 w-6 bg-yellow-300 rounded-full"
                    ></div>
                    <div
                      class="absolute left-2 top-16 h-4 w-12 bg-white rounded-md"
                    ></div>
                  </div>
                  <div
                    class="absolute right-0 w-1/2 h-full bg-gradient-to-r from-gray-800 to-gray-950"
                  >
                    <div
                      class="absolute right-2 top-4 h-6 w-6 bg-blue-300 rounded-full"
                    ></div>
                    <div
                      class="absolute left-2 top-16 h-4 w-12 bg-gray-700 rounded-md"
                    ></div>
                  </div>
                </div>

                <!-- 选中标记 -->
                <div
                  v-if="currentTheme === theme.id"
                  class="absolute top-2 right-2 bg-primary text-white rounded-full p-1"
                >
                  <Icon icon="ph:check-bold" class="w-4 h-4" />
                </div>
              </div>

              <!-- 主题信息 -->
              <div class="px-4 py-3 bg-card">
                <div class="flex items-center gap-2">
                  <Icon :icon="theme.icon" class="w-5 h-5" />
                  <h4 class="font-medium">{{ theme.name }}</h4>
                </div>
                <p class="text-sm text-muted-foreground mt-1">
                  {{ theme.description }}
                </p>
              </div>
            </div>
          </CardContent>
        </Card>
      </div>
    </ScrollArea>
  </div>
</template>
