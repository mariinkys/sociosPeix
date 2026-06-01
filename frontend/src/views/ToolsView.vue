<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import EmailQuotaCard from '@/components/email/EmailQuotaCard.vue'
import SendEmailByInterestCard from '@/components/email/SendEmailByInterestCard.vue'
import SendEmailToAllCard from '@/components/email/SendEmailsToAllCard.vue'
import VersionChip from '@/components/VersionChip.vue'

const { t } = useI18n({ useScope: 'global' })
</script>

<template>
  <div class="p-6 space-y-6">
    <div>
      <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
        {{ t('tools.title') }}
      </h1>
      <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
        {{ t('tools.description') }}
      </p>
    </div>

    <div class="flex flex-wrap gap-6 items-stretch">
      <EmailQuotaCard class="flex-1 min-w-72" />
      <SendEmailByInterestCard class="flex-1 min-w-72" />
      <SendEmailToAllCard class="flex-1 min-w-72" />
    </div>

    <div class="absolute bottom-6 left-1/2 -translate-x-1/2">
      <VersionChip />
    </div>
  </div>
</template>
