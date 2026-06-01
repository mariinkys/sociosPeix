<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import { wrapEmailBody } from '@/utils/emailTemplates'
import type { EmailResponse } from '@/types/email.types'

defineProps<{
  email: (EmailResponse & { body?: string }) | null
  loading: boolean
}>()

const { t } = useI18n({ useScope: 'global' })
const visible = defineModel<boolean>('visible', { required: true })
</script>

<template>
  <Dialog
    v-model:visible="visible"
    :header="email?.subject ?? t('email.titles.previewFallback')"
    :style="{ width: '720px' }"
    :breakpoints="{ '768px': '95vw' }"
    modal
    :draggable="false"
  >
    <div v-if="loading" class="flex items-center justify-center py-16">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
    </div>

    <div v-else-if="email" class="space-y-4 py-2">
      <div class="grid grid-cols-2 gap-3 text-sm">
        <div class="flex flex-col gap-0.5">
          <span class="text-xs font-medium text-surface-400 uppercase tracking-wide">{{
            t('common.fields.sentAt')
          }}</span>
          <span class="text-surface-700 dark:text-surface-300">
            {{
              new Date(email.createdAt).toLocaleDateString('es-ES', {
                day: 'numeric',
                month: 'long',
                year: 'numeric',
              })
            }}
          </span>
        </div>
        <div class="flex flex-col gap-0.5">
          <span class="text-xs font-medium text-surface-400 uppercase tracking-wide">{{
            t('common.fields.provider')
          }}</span>
          <span class="text-surface-700 dark:text-surface-300 capitalize">{{
            email.provider
          }}</span>
        </div>
      </div>

      <div class="border-t border-surface-100 dark:border-surface-800"></div>

      <div class="flex flex-col gap-2">
        <div class="flex items-center justify-between">
          <span class="text-xs font-medium text-surface-400 uppercase tracking-wide">{{
            t('common.fields.recipients')
          }}</span>
          <span
            class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 text-xs font-semibold"
          >
            {{ email.recipientCount }}
          </span>
        </div>
        <div class="flex flex-wrap gap-1.5">
          <span
            v-for="recipient in email.recipientEmails"
            :key="recipient"
            class="inline-flex items-center gap-1 px-2 py-1 rounded-full bg-surface-100 dark:bg-surface-800 text-xs text-surface-600 dark:text-surface-300"
          >
            <i class="pi pi-envelope text-xs text-surface-400"></i>
            {{ recipient }}
          </span>
        </div>
      </div>

      <div class="border-t border-surface-100 dark:border-surface-800"></div>

      <div class="flex flex-col gap-1.5">
        <span class="text-xs font-medium text-surface-400 uppercase tracking-wide">{{
          t('common.fields.body')
        }}</span>
        <iframe
          :srcdoc="wrapEmailBody(email.body ?? '')"
          class="w-full rounded-lg border border-surface-200 dark:border-surface-700"
          style="height: 400px"
          sandbox="allow-same-origin"
          title="Email body preview"
        ></iframe>
      </div>
    </div>

    <template #footer>
      <Button
        :label="t('common.actions.close')"
        severity="secondary"
        outlined
        @click="visible = false"
      />
    </template>
  </Dialog>
</template>
