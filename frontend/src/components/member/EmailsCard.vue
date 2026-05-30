<script setup lang="ts">
import { ref, onMounted } from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'
import Card from 'primevue/card'
import { useToast } from 'primevue/usetoast'
import { emailsService } from '@/services/emails.service'
import SendEmailDialog from '@/components/email/SendEmailDialog.vue'
import EmailPreviewDialog from '@/components/email/EmailPreviewDialog.vue'
import type { EmailResponse } from '@/types/email.types'
import type { DataTablePageEvent } from 'primevue/datatable'

const props = defineProps<{ memberId: string }>()

const toast = useToast()

const emails = ref<EmailResponse[]>([])
const loading = ref(false)
const totalElements = ref(0)
const page = ref(0)
const size = ref(5)

const sendDialogVisible = ref(false)

const previewVisible = ref(false)
const previewLoading = ref(false)
const previewEmail = ref<(EmailResponse & { body?: string }) | null>(null)

async function fetchEmails() {
  loading.value = true
  try {
    const data = await emailsService.getByMember(props.memberId, {
      page: page.value,
      size: size.value,
    })
    emails.value = data.content
    totalElements.value = data.totalElements
  } catch {
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to load emails.', life: 3000 })
  } finally {
    loading.value = false
  }
}

function onPage(event: DataTablePageEvent) {
  page.value = event.page
  size.value = event.rows
  fetchEmails()
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
      summary: 'Error',
      detail: 'Failed to load email content.',
      life: 3000,
    })
    previewVisible.value = false
  } finally {
    previewLoading.value = false
  }
}

onMounted(fetchEmails)
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <i class="pi pi-envelope text-primary-500 dark:text-primary-400"></i>
            <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0">Emails</h2>
            <span
              v-if="totalElements > 0"
              class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 text-xs font-semibold"
            >
              {{ totalElements }}
            </span>
          </div>
          <div class="flex items-center gap-2">
            <Button
              icon="pi pi-refresh"
              severity="secondary"
              text
              rounded
              size="small"
              aria-label="Refresh emails"
              :loading="loading"
              @click="fetchEmails"
            />
            <Button
              label="Send Email"
              icon="pi pi-send"
              size="small"
              @click="sendDialogVisible = true"
            />
          </div>
        </div>

        <DataTable
          :value="emails"
          :loading="loading"
          :rows="size"
          :totalRecords="totalElements"
          :rowsPerPageOptions="[5, 10, 25]"
          lazy
          paginator
          size="small"
          paginatorTemplate="FirstPageLink PrevPageLink CurrentPageReport NextPageLink LastPageLink"
          currentPageReportTemplate="{first} to {last} of {totalRecords}"
          @page="onPage"
        >
          <Column field="subject" header="Subject" style="width: 50%">
            <template #body="{ data }: { data: EmailResponse }">
              <span class="font-medium text-surface-900 dark:text-surface-0 text-sm">{{
                data.subject
              }}</span>
            </template>
          </Column>
          <Column field="provider" header="Provider" style="width: 15%">
            <template #body="{ data }: { data: EmailResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm capitalize">{{
                data.provider
              }}</span>
            </template>
          </Column>
          <Column field="createdAt" header="Sent At" style="width: 25%">
            <template #body="{ data }: { data: EmailResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm">
                {{
                  new Date(data.createdAt).toLocaleDateString('es-ES', {
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
                  size="small"
                  aria-label="View email"
                  @click="openPreview(data)"
                />
              </div>
            </template>
          </Column>
          <template #empty>
            <div class="flex flex-col items-center justify-center py-10 gap-2 text-surface-400">
              <i class="pi pi-envelope text-3xl"></i>
              <p class="text-sm">No emails sent yet</p>
            </div>
          </template>
        </DataTable>
      </div>
    </template>
  </Card>

  <SendEmailDialog
    mode="member"
    :member-id="memberId"
    v-model:visible="sendDialogVisible"
    @sent="fetchEmails"
  />

  <EmailPreviewDialog
    v-model:visible="previewVisible"
    :email="previewEmail"
    :loading="previewLoading"
  />
</template>
