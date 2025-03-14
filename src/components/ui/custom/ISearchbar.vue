<script setup lang="ts">
import { Icon } from '@iconify/vue'

const conversationStore = useConversationStore()
const selectedItem = ref(null)

const items = computed(() =>
  conversationStore.conversations.map((conversation) => ({
    label: conversation.studentName,
    value: conversation.id,
  }))
)

// 处理选中事件
function handleSelect(item: any) {
  if (item && item.value) {
    // 设置选中的对话
    conversationStore.setActiveConversation(item.value)
  }
}
</script>

<template>
  <Combobox
    by="label"
    v-model="selectedItem"
    @update:model-value="handleSelect"
  >
    <ComboboxAnchor>
      <div class="relative flex w-full px-2">
        <ComboboxInput
          class="pl-7 h-7 bg-secondary"
          :display-value="(val: any) => val?.label ?? ''"
          placeholder="搜索联系人..."
        />
        <span class="absolute inset-y-0 flex items-center justify-center px-2">
          <Icon icon="ic:round-search" class="size-4 text-muted-foreground" />
        </span>
      </div>
    </ComboboxAnchor>

    <ComboboxList class="max-h-80">
      <ComboboxEmpty> 该对话不存在 </ComboboxEmpty>

      <ComboboxGroup>
        <ComboboxItem v-for="item in items" :key="item.value" :value="item">
          {{ item.label }}

          <ComboboxItemIndicator>
            <Icon icon="ic:round-check" class="size-4 text-primary" />
          </ComboboxItemIndicator>
        </ComboboxItem>
      </ComboboxGroup>
    </ComboboxList>
  </Combobox>
</template>
