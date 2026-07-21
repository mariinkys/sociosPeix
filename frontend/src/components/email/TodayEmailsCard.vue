c
<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'
import ToggleButton from 'primevue/togglebutton'
import Card from 'primevue/card'
import Tag from 'primevue/tag'
import { useToast } from 'primevue/usetoast'
import { emailsService } from '@/services/emails.service'
import type { EmailResponse } from '@/types/email.types'
import EmailPreviewDialog from '@/components/email/EmailPreviewDialog.vue'

const { t, locale } = useI18n({ useScope: 'global' })
const toast = useToast()

const emails = ref<EmailResponse[]>([])
const loading = ref(false)
const showTransactional = ref(false)

const previewVisible = ref(false)
const previewLoading = ref(false)
const previewEmail = ref<(EmailResponse & { body?: string }) | null>(null)

const visibleEmails = computed(() =>
  showTransactional.value
    ? emails.value
    : emails.value.filter((e) => e.category !== 'TRANSACTIONAL'),
)

async function fetchTodayEmails() {
  loading.value = true
  try {
    emails.value = await emailsService.getToday()
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('email.todayEmailsCard.errors.load'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

async function openPreview(email: EmailResponse) {
  previewVisible.value = true
  previewLoading.value = true
  previewEmail.value = email
  try {
    previewEmail.value = await emailsService.getById(email.id)
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('email.todayEmailsCard.errors.loadContent'),
      life: 3000,
    })
    previewVisible.value = false
  } finally {
    previewLoading.value = false
  }
}

onMounted(fetchTodayEmails)
</script>

<template>
  <Card v-bind="$attrs" class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <i class="pi pi-gift text-primary-500 dark:text-primary-400" />
            <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0">
              {{ t('email.titles.todayEmails') }}
            </h2>
            <span
              v-if="visibleEmails.length > 0"
              class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 text-xs font-semibold"
            >
              {{ visibleEmails.length }}
            </span>
          </div>

          <div class="flex items-center gap-1">
            <ToggleButton
              v-model="showTransactional"
              :onLabel="t('email.common.showingTransactional')"
              :offLabel="t('email.common.hidingTransactional')"
              data-tour="home-emails-today-transactional-button"
              onIcon="pi pi-eye"
              offIcon="pi pi-eye-slash"
            />
            <Button
              icon="pi pi-refresh"
              severity="secondary"
              text
              rounded
              aria-label="Refresh"
              :loading="loading"
              @click="fetchTodayEmails"
            />
          </div>
        </div>

        <DataTable
          :value="visibleEmails"
          :loading="loading"
          :rows="10"
          :rowsPerPageOptions="[5, 10, 25]"
          paginator
          paginatorTemplate="RowsPerPageDropdown FirstPageLink PrevPageLink CurrentPageReport NextPageLink LastPageLink"
          currentPageReportTemplate="{first} to {last} of {totalRecords}"
        >
          <Column field="subject" :header="t('common.fields.subject')" style="width: 40%">
            <template #body="{ data }: { data: EmailResponse }">
              <div class="flex items-center gap-2">
                <Tag
                  v-if="data.category === 'TRANSACTIONAL'"
                  :value="t('email.category.transactional')"
                  severity="info"
                  class="text-xs"
                />

                <span class="font-medium text-surface-900 dark:text-surface-0 text-sm">{{
                  data.subject
                }}</span>
              </div>
            </template>
          </Column>
          <Column field="provider" :header="t('common.fields.provider')" style="width: 10%">
            <template #body="{ data }: { data: EmailResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm capitalize">{{
                data.provider
              }}</span>
            </template>
          </Column>
          <Column
            field="recipientCount"
            :header="t('common.fields.sentTo')"
            sortable
            style="width: 10%"
          >
            <template #body="{ data }: { data: EmailResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm capitalize">{{
                data.recipientCount
              }}</span>
            </template>
          </Column>
          <Column field="createdAt" :header="t('common.fields.sentAt')" sortable style="width: 30%">
            <template #body="{ data }: { data: EmailResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm">
                {{
                  new Date(data.createdAt).toLocaleDateString(locale, {
                    day: 'numeric',
                    month: 'short',
                    year: 'numeric',
                  })
                }}
              </span>
            </template>
          </Column>
          <Column style="width: 10%">
            <template #body="{ data }: { data: EmailResponse }">
              <div class="flex justify-end">
                <Button
                  icon="pi pi-eye"
                  severity="secondary"
                  text
                  rounded
                  :aria-label="t('email.todayEmailsCard.viewEmail')"
                  @click="openPreview(data)"
                />
              </div>
            </template>
          </Column>
          <template #empty>
            <div class="flex flex-col items-center justify-center py-10 gap-2 text-surface-400">
              <i class="pi pi-envelope text-3xl"></i>
              <p class="text-sm">{{ t('email.todayEmailsCard.empty') }}</p>
            </div>
          </template>
        </DataTable>
      </div>
    </template>
  </Card>

  <EmailPreviewDialog
    v-model:visible="previewVisible"
    :email="previewEmail"
    :loading="previewLoading"
  />
</template>
