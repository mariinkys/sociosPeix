<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import TodayEmailsCard from '@/components/email/TodayEmailsCard.vue'
import TodayBirthdayCard from '@/components/member/TodayBirthdayCard.vue'

const { t } = useI18n({ useScope: 'global' })
</script>

<template>
  <div class="p-6 space-y-6">
    <div>
      <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
        {{ t('home.page.title') }}
      </h1>
      <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
        {{ t('home.page.description') }}
      </p>
    </div>

    <TodayBirthdayCard />

    <TodayEmailsCard />
  </div>
</template>
