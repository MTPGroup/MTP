<script lang="ts" setup>
defineProps<{
  title: string
  isSelected: boolean
  onSelected: () => void
}>()
</script>

<template>
  <div
    :class="`flex items-center justify-start p-2 rounded-md hover:bg-secondary ${
      isSelected ? 'bg-theme-400' : ''
    }`"
    @click="onSelected()"
  >
    {{ title }}
  </div>
</template>
