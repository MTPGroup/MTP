<script setup lang="ts">
import { useMyUserInfoStore } from '@/stores/userInfo'
import { Icon } from '@iconify/vue'
import { WebviewWindow } from '@tauri-apps/api/webviewWindow'

const props = defineProps({
  size: {
    type: String,
    default: 'md', // sm, md, lg
  },
})

const userStore = useMyUserInfoStore()
const avatarUrl = useState(() => userStore.userAvatar)
const isHovering = useState(() => false)
const fileInput = useState<HTMLInputElement | null>(() => null)

const sizeClasses = {
  sm: 'h-16 w-16',
  md: 'h-24 w-24',
  lg: 'h-32 w-32',
}

const handleFileSelect = (event: Event) => {
  const input = event.target as HTMLInputElement
  if (input.files && input.files[0]) {
    const file = input.files[0]

    // Validate file type
    if (!file.type.match('image.*')) {
      alert('Please select an image file')
      return
    }

    // Create FileReader to read the file
    const reader = new FileReader()
    reader.onload = async (e) => {
      const result = e.target?.result as string
      avatarUrl.value = result

      // 通知主窗口更新头像
      try {
        const mainWindow = await WebviewWindow.getByLabel('main')
        if (mainWindow) {
          // 使用Tauri的emit方法发送事件
          await mainWindow.emit('user-settings-updated', {
            type: 'avatar',
            value: avatarUrl.value,
          })
        }
      } catch (error) {
        console.error('Failed to notify main window:', error)
      }
    }
    reader.readAsDataURL(file)
  }
}

function triggerFileInput() {
  fileInput.value?.click()
}
</script>

<template>
  <div class="relative">
    <Avatar
      :class="[sizeClasses[props.size as keyof typeof sizeClasses], 'cursor-pointer']"
      @mouseenter="isHovering = true"
      @mouseleave="isHovering = false"
      @click="triggerFileInput"
    >
      <AvatarImage :src="avatarUrl" />
      <AvatarFallback>{{ userStore.userName.charAt(0) }}</AvatarFallback>

      <!-- Overlay with edit icon on hover -->
      <div
        v-show="isHovering"
        class="absolute inset-0 bg-black/30 rounded-full flex items-center justify-center"
        :class="[sizeClasses[props.size as keyof typeof sizeClasses], 'cursor-pointer']"
      >
        <Icon icon="lucide:camera" class="text-white h-6 w-6" />
      </div>
    </Avatar>

    <!-- Hidden file input -->
    <input
      ref="fileInput"
      type="file"
      accept="image/*"
      class="hidden"
      @change="handleFileSelect"
    />
  </div>
</template>
