<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import Button from 'primevue/button'
import TodayEmailsCard from '@/components/email/TodayEmailsCard.vue'
import TodayBirthdayCard from '@/components/member/TodayBirthdayCard.vue'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
</script>

<template>
  <div class="p-6 space-y-6">
    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
          {{ t('home.title') }}
        </h1>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
          {{ t('home.description') }}
        </p>
      </div>
      <Button
        :label="t('members.actions.createNew')"
        data-tour="members-add"
        icon="pi pi-plus"
        class="shrink-0"
        @click="router.push('/members/new')"
      />
    </div>

    <TodayBirthdayCard data-tour="home-birthdays" />

    <TodayEmailsCard data-tour="home-emails-today" />
  </div>
</template>
