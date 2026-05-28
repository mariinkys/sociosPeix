<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import Button from 'primevue/button'

const router = useRouter()
const auth = useAuthStore()

const STORAGE_KEY = 'socios-peix-theme'
const isDark = ref(false)

const applyTheme = (dark: boolean) => {
  isDark.value = dark
  document.documentElement.classList.toggle('my-app-dark', dark)
  localStorage.setItem(STORAGE_KEY, dark ? 'dark' : 'light')
}

onMounted(() => {
  const saved = localStorage.getItem(STORAGE_KEY)
  if (saved) {
    applyTheme(saved === 'dark')
  } else {
    applyTheme(window.matchMedia('(prefers-color-scheme: dark)').matches)
  }
})

const toggleTheme = () => applyTheme(!isDark.value)

const logout = async () => {
  await auth.logout()
  router.push('/login')
}
</script>

<template>
  <header
    class="flex items-center justify-between border-b border-surface-200 dark:border-surface-700 bg-white dark:bg-surface-900 px-6 py-3 shadow-sm transition-colors duration-200"
  >
    <div class="flex items-center gap-2">
      <span class="text-base font-semibold text-surface-900 dark:text-surface-0"> SociosPeix </span>
    </div>

    <div class="flex items-center gap-2">
      <template v-if="auth.isAuthenticated">
        <span class="text-sm text-surface-500 dark:text-surface-400 hidden sm:inline">
          {{ auth.user?.email }}
        </span>

        <Button
          icon="pi pi-sign-out"
          severity="secondary"
          text
          size="small"
          @click="logout"
          class="!text-surface-700 dark:!text-surface-300"
        />
      </template>

      <Button
        :icon="isDark ? 'pi pi-sun' : 'pi pi-moon'"
        :aria-label="isDark ? 'Switch to light mode' : 'Switch to dark mode'"
        severity="secondary"
        text
        size="small"
        @click="toggleTheme"
        class="!text-surface-700 dark:!text-surface-300"
      />
    </div>
  </header>
</template>
