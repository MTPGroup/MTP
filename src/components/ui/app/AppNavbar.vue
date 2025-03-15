<script lang="ts" setup>
import { Icon } from '@iconify/vue'

const route = useRoute()
const selected = computed(() => {
  return route.path
})
</script>

<template>
  <div class="pt-4 pb-2">
    <IButton
      :class="`${
        selected.startsWith('/chat') ? 'bg-theme-400' : ''
      } hover:secondary rounded-sm`"
    >
      <NuxtLink to="/chat">
        <Icon
          :icon="
            selected.startsWith('/chat')
              ? 'mingcute:chat-2-fill'
              : 'mingcute:chat-2-line'
          "
          :class="`${
            selected.startsWith('/chat') ? 'text-theme-600' : 'text-primary'
          } size-6`"
          flip="horizontal"
        />
      </NuxtLink>
    </IButton>
  </div>
</template>
