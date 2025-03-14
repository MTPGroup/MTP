import { nextTick } from 'vue'

export function useScrollToBottom() {
  // 获取RadixVue滚动视口的函数
  function getViewportElement() {
    return document.querySelector('#scroll-area-viewport')?.parentElement
      ?.parentElement
    // document.querySelector('.radix-scroll-area-viewport') ||
    // document.querySelector('[data-radix-scroll-area-viewport]')
  }

  // 滚动到底部
  function scrollToBottom(smooth: boolean = false) {
    nextTick(() => {
      const viewport = getViewportElement()
      if (viewport) {
        if (smooth) {
          viewport.scrollTo({
            top: viewport.scrollHeight,
            behavior: 'smooth',
          })
        } else {
          viewport.scrollTop = viewport.scrollHeight
        }
        return true
      }
      return false
    })
  }

  // 检查是否在底部附近
  function isNearBottom(threshold = 100) {
    const viewport = getViewportElement()
    if (!viewport) return true

    const { scrollHeight, scrollTop, clientHeight } = viewport
    return scrollHeight - scrollTop - clientHeight < threshold
  }

  return {
    scrollToBottom,
    isNearBottom,
  }
}
