<script setup lang="ts">
const userStore = useMyUserInfoStore()
</script>

<template>
  <div class="relative cursor-pointer">
    <Avatar>
      <AvatarImage :src="userStore.userAvatar" alt="avatar" />
      <AvatarFallback>avatar</AvatarFallback>
    </Avatar>
    <span :class="`absolute bottom-0.5 size-4 border-2 border-transparent rounded-full bg-clip-padding -right-1 ${true ? 'bg-emerald-400' : 'bg-rose-400'
        }`" />
  </div>
</template>
