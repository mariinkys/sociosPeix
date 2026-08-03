<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import Button from 'primevue/button'
import SplitButton from 'primevue/splitbutton'
import type { MenuItem } from 'primevue/menuitem'
import TodayEmailsCard from '@/components/email/TodayEmailsCard.vue'
import TodayBirthdayCard from '@/components/member/TodayBirthdayCard.vue'
import FormOcrDialog from '@/components/formOcr/FormOcrDialog.vue'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()

const ocrDialogVisible = ref(false)

function downloadBlankForm() {
  const link = document.createElement('a')
  link.href = '/registrationForm.pdf'
  link.download = 'registrationForm.pdf'
  link.click()
}

const scanFormMenuItems: MenuItem[] = [
  {
    label: t('members.actions.scanForm'),
    icon: 'pi pi-qrcode',
    command: () => {
      ocrDialogVisible.value = true
    },
  },
  {
    label: t('members.actions.downloadBlankForm'),
    icon: 'pi pi-download',
    command: () => {
      downloadBlankForm()
    },
  },
]
</script>

<template>
  <div class="p-6 space-y-6">
    <FormOcrDialog v-model:visible="ocrDialogVisible" />

    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
          {{ t('home.title') }}
        </h1>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
          {{ t('home.description') }}
        </p>
      </div>

      <div class="flex flex-col gap-2 sm:flex-row sm:items-center">
        <SplitButton
          :label="t('members.actions.scanForm')"
          icon="pi pi-qrcode"
          data-tour="members-scanForm"
          severity="secondary"
          class="w-full sm:w-auto"
          :model="scanFormMenuItems"
          :pt="{
            root: { class: 'flex w-full sm:w-auto' },
            pcButton: { root: { class: 'flex-1' } },
            menuButton: { root: { class: 'shrink-0' } },
          }"
          @click="ocrDialogVisible = true"
        />

        <Button
          :label="t('members.actions.createNew')"
          data-tour="members-add"
          icon="pi pi-plus"
          class="w-full sm:w-auto"
          @click="router.push('/members/new')"
        />
      </div>
    </div>

    <TodayBirthdayCard data-tour="home-birthdays" />

    <TodayEmailsCard data-tour="home-emails-today" />
  </div>
</template>
