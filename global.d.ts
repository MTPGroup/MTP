interface Window {
  electron: {
    openAboutWindow: () => void
    closeAboutWindow: () => void
    openSettingsWindow: () => void
    closeSettingsWindow: () => void
    closeWindow: () => void
    saveAvatar: (dataUrl: string) => Promise<string>
    sendToMainWindow: (channel: string, data: unknown) => void
    onMessage: (channel: string, callback: (data: unknown) => void) => void
  }
}
