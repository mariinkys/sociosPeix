<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useRoute } from 'vue-router'

const { t } = useI18n({ useScope: 'global' })
const route = useRoute()

interface StatNavItem {
  path: string
  labelKey: string
  icon: string
}

const navItems: StatNavItem[] = [
  {
    path: '/statistics/interests-popularity',
    labelKey: 'statistics.interestPopularity.title',
    icon: 'pi pi-chart-bar',
  },
]

function isActive(path: string): boolean {
  return route.path === path
}
</script>

<template>
  <div class="p-6 space-y-6">
    <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
      {{ t('statistics.pageTitle') }}
    </h1>

    <div class="flex flex-col md:flex-row gap-6 md:min-h-[calc(100vh-9rem)]">
      <nav
        class="flex flex-row md:flex-col gap-1 overflow-x-auto md:overflow-x-visible md:w-56 shrink-0 border-b md:border-b-0 md:border-r border-surface-200 dark:border-surface-700 pb-2 md:pb-0 md:pr-4"
      >
        <RouterLink
          v-for="item in navItems"
          :key="item.path"
          :to="item.path"
          class="flex items-center gap-2 px-3 py-2 rounded-lg text-sm whitespace-nowrap transition-colors"
          :class="
            isActive(item.path)
              ? 'bg-primary-50 dark:bg-primary-950 text-primary-700 dark:text-primary-300 font-medium'
              : 'text-surface-600 dark:text-surface-400 hover:bg-surface-100 dark:hover:bg-surface-800'
          "
        >
          <i :class="item.icon"></i>
          {{ t(item.labelKey) }}
        </RouterLink>
      </nav>

      <div class="flex-1 min-w-0">
        <RouterView />
      </div>
    </div>
  </div>
</template>
