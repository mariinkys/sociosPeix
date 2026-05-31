<script setup lang="ts">
import EmailQuotaCard from '@/components/email/EmailQuotaCard.vue'
import SendEmailByInterestCard from '@/components/email/SendEmailByInterestCard.vue'
import SendEmailToAllCard from '@/components/email/SendEmailsToAllCard.vue'
</script>

<template>
  <div class="p-6 space-y-6">
    <div>
      <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">Tools</h1>
      <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">Let's get things done</p>
    </div>

    <div class="flex flex-wrap gap-6 items-stretch">
      <EmailQuotaCard class="flex-1 min-w-72" />
      <SendEmailByInterestCard class="flex-1 min-w-72" />
      <SendEmailToAllCard class="flex-1 min-w-72" />
    </div>
  </div>
</template>
