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
        selected.startsWith('/chat') ? 'bg-secondary' : ''
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
            selected.startsWith('/chat') ? 'text-pink-200' : 'text-primary'
          } size-6`"
          flip="horizontal"
        />
      </NuxtLink>
    </IButton>
  </div>
  <!-- <div class="py-2">
    <IButton
      :class="`${
        selected === '/contact' ? 'bg-secondary' : ''
      } hover:bg-secondary rounded-sm`"
    >
      <NuxtLink to="/contact">
        <Icon
          :icon="selected === '/contact' ? 'bi:person-fill' : 'bi:person'"
          :class="`${
            selected === '/contact' ? 'text-pink-200' : 'text-primary'
          } size-6`"
          flip="horizontal"
        />
      </NuxtLink>
    </IButton>
  </div> -->
</template>
