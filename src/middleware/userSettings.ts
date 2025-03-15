import { useToast } from '~/components/ui/toast'

export default defineNuxtRouteMiddleware(async () => {
  const colorMode = useColorMode()
  const { toast } = useToast()
  const userStore = useMyUserInfoStore()

  await userStore.loadUserSettings()
  if (userStore.apiKey == '') {
    toast({
      title: 'API Key 不存在',
      description: '请先设置 API Key',
      variant: 'destructive',
      duration: 5000,
    })
  }
  colorMode.preference = userStore.theme
})
