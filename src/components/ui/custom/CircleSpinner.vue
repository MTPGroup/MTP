<script lang="ts" setup>
defineProps({
  size: {
    type: Number,
    default: 24,
  },
  color: {
    type: String,
    default: 'currentColor',
  },
  thickness: {
    type: Number,
    default: 2,
  },
  duration: {
    type: Number,
    default: 1.2, // seconds
  },
})
</script>

<template>
  <div class="inline-flex items-center justify-center" aria-label="加载中">
    <svg
      :width="size"
      :height="size"
      viewBox="0 0 24 24"
      xmlns="http://www.w3.org/2000/svg"
      class="animate-spin-slow"
      :style="{
        '--duration': `${duration}s`,
        'animation-duration': `${duration}s`,
      }"
    >
      <circle
        class="animate-circle"
        cx="12"
        cy="12"
        r="10"
        fill="none"
        :stroke="color"
        :stroke-width="thickness"
        stroke-linecap="round"
        :style="{
          'animation-duration': `${duration * 2}s`,
          'stroke-dasharray': '62.83',
          'stroke-dashoffset': '0',
          'transform-origin': 'center',
        }"
      ></circle>
    </svg>
  </div>
</template>

<style scoped>
@keyframes rotate {
  100% {
    transform: rotate(360deg);
  }
}

@keyframes dash {
  0% {
    stroke-dasharray: 1, 150;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: -35;
  }
  100% {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: -124;
  }
}

@keyframes circle-dash {
  0% {
    stroke-dasharray: 1, 150;
    stroke-dashoffset: 0;
  }
  50% {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: -35;
  }
  100% {
    stroke-dasharray: 90, 150;
    stroke-dashoffset: -124;
  }
}

.animate-circle {
  animation: circle-dash ease-in-out infinite;
}
</style>
