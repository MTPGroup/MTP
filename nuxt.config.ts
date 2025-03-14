// https://nuxt.com/docs/api/configuration/nuxt-config
import path from 'path'

export default defineNuxtConfig({
    compatibilityDate: '2025-03-07',
    appConfig: {
        nuxt: 'hanasaki.mtp.app',
    },
    ssr: false,
    devtools: { enabled: true },
    devServer: { host: process.env.TAURI_DEV_HOST || 'localhost' },
    srcDir: 'src/',
    css: ['~/assets/css/tailwind.css'],
    vite: {
        clearScreen: false,
        envPrefix: ['VITE_', 'TAURI_'],
        server: {
            // Tauri requires a consistent port
            strictPort: true,
        },
    },
    components: {
        dirs: ['src/components'],
    },

    modules: [
        '@nuxtjs/tailwindcss',
        'shadcn-nuxt',
        '@pinia/nuxt',
        'pinia-plugin-persistedstate/nuxt',
    ],
    shadcn: {
        prefix: '',
        componentDir: 'src/components/ui',
    },
})
