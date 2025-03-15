<script setup lang="ts">
import { Icon } from '@iconify/vue'
import { listen } from '@tauri-apps/api/event'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'
import { tauriService } from '~/services/tauri'

const colorMode = useColorMode()
const userStore = useMyUserInfoStore()
const avatar = computed(() => userStore.userAvatar)
const avatarAlt = computed(() => userStore.userName.charAt(0))

onMounted(async () => {
  await listen('user-settings-updated', (event) => {
    const { type, value } = event.payload as { type: string; value: string }
    if (type === 'username') {
      userStore.setUserName(value)
    } else if (type === 'avatar') {
      userStore.setUserAvatar(value)
    } else if (type === 'theme') {
      colorMode.preference = value
    }
  })
})

const toggleMode = async () => {
  colorMode.preference = colorMode.preference === 'dark' ? 'light' : 'dark'
  await tauriService.setStore('theme', colorMode.preference)
  try {
    const settingsWindow = await WebviewWindow.getByLabel('settings')
    if (settingsWindow) {
      await settingsWindow.emit('user-settings-updated', {
        type: 'theme',
        value: colorMode.preference,
      })
    }
    const aboutWindow = await WebviewWindow.getByLabel('about')
    if (aboutWindow) {
      await aboutWindow.emit('user-settings-updated', {
        type: 'theme',
        value: colorMode.preference,
      })
    }
  } catch (error) {
    console.error('Failed to notify main window:', error)
  }
}

const openAboutWindow = async () => {
  try {
    const existingWindow = await WebviewWindow.getByLabel('about')
    if (existingWindow) {
      await existingWindow.show()
      return
    }
    // 创建一个新窗口
    const win = new WebviewWindow('about', {
      title: '关于',
      url: `/about`,
      width: 512,
      height: 844,
      center: true,
      resizable: false,
      decorations: false,
      transparent: false,
    })
  } catch (error) {
    console.error('Failed to open about window:', error)
  }
}

const openSettingsWindow = async () => {
  try {
    const existingWindow = await WebviewWindow.getByLabel('settings')
    if (existingWindow) {
      await existingWindow.show()
      return
    }
    // 创建一个新窗口
    new WebviewWindow('settings', {
      title: '设置',
      url: `/settings`,
      width: 833,
      height: 840,
      center: true,
      resizable: false,
      decorations: false,
      transparent: false,
    })
  } catch (error) {
    console.error('Failed to open settings window:', error)
  }
}

const closeWindow = async () => {
  const existingWindow = await WebviewWindow.getByLabel('main')
  if (existingWindow) {
    await existingWindow.close()
  }
}
</script>

<template>
  <!-- 图标栏 -->
  <div
    class="flex flex-col w-16 h-full p-2 items-center justify-start bg-theme-300"
  >
    <div class="flex flex-col items-center justify-start flex-grow">
      <p class="pt-2 pb-4">MTP</p>
      <IAvatar />
      <AppNavbar />
    </div>
    <div class="flex items-center justify-center">
      <DropdownMenu>
        <DropdownMenuTrigger>
          <div class="flex flex-col items-center justify-center py-2">
            <IButton class="rounded-sm hover:bg-theme-400">
              <Icon
                icon="solar:hamburger-menu-broken"
                class="size-6 text-black"
              />
            </IButton>
          </div>
        </DropdownMenuTrigger>
        <DropdownMenuContent
          side="right"
          align="end"
          :align-offset="10"
          class="w-72"
        >
          <div class="space-y-1 p-2">
            <div class="flex items-center gap-2 p-2 rounded-md">
              <Avatar class="h-8 w-8">
                <AvatarImage :src="avatar" />
                <AvatarFallback>{{ avatarAlt }}</AvatarFallback>
              </Avatar>
              <div class="flex flex-col">
                <p class="text-sm font-medium">{{ userStore.userName }}</p>
                <p class="text-xs text-muted-foreground">在线</p>
              </div>
            </div>

            <Separator />

            <!-- 菜单项 -->
            <DropdownMenuItem @click="openSettingsWindow">
              <Icon icon="lucide:settings" class="mr-2 h-4 w-4" />
              <span>设置</span>
            </DropdownMenuItem>

            <DropdownMenuItem @click="toggleMode">
              <Icon
                :icon="
                  colorMode.value === 'dark' ? 'lucide:sun' : 'lucide:moon'
                "
                class="mr-2 h-4 w-4"
              />
              <span>{{
                colorMode.value === 'dark' ? '浅色模式' : '深色模式'
              }}</span>
            </DropdownMenuItem>

            <DropdownMenuItem @click="openAboutWindow">
              <Icon icon="lucide:info" class="mr-2 h-4 w-4" />
              <span>关于</span>
            </DropdownMenuItem>

            <Separator class="my-1" />

            <DropdownMenuItem
              class="text-red-500 focus:text-red-500 focus:bg-red-50"
              @click="closeWindow"
            >
              <Icon icon="lucide:log-out" class="mr-2 h-4 w-4" />
              <span>退出</span>
            </DropdownMenuItem>
          </div>
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  </div>
</template>
