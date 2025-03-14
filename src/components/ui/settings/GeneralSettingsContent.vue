<script setup lang="ts">
import { useMyUserInfoStore } from '@/stores/userInfo'
import { Icon } from '@iconify/vue'
import { useToast } from '../toast'

const { toast } = useToast()
const userStore = useMyUserInfoStore()
const username = useState(() => userStore.userName)

const saveUsername = () => {
  console.log('Saving username:', username.value)
  userStore.setUserName(username.value)
  toast({
    title: '设置已保存',
    description: '用户名已更新成功',
  })
  if (window.electron) {
    window.electron.sendToMainWindow('settings-updated', {
      type: 'username',
      value: username.value,
    })
  }
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
