<!-- components/member/MemberEmailsCard.vue -->
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'
import Card from 'primevue/card'
import Dialog from 'primevue/dialog'
import InputText from 'primevue/inputtext'
import Editor from 'primevue/editor'
import Message from 'primevue/message'
import { useToast } from 'primevue/usetoast'
import { emailsService } from '@/services/emails.service'
import type { EmailResponse } from '@/types/email.types'
import type { DataTablePageEvent } from 'primevue/datatable'

const props = defineProps<{ memberId: string }>()

const toast = useToast()

const emails = ref<EmailResponse[]>([])
const loading = ref(false)
const totalElements = ref(0)
const page = ref(0)
const size = ref(5)

const dialogVisible = ref(false)
const sendLoading = ref(false)
const subject = ref('')
const htmlBody = ref('')
const subjectError = ref('')
const bodyError = ref('')

const previewVisible = ref(false)
const previewLoading = ref(false)
const previewEmail = ref<(EmailResponse & { body?: string }) | null>(null)

const attachments = ref<File[]>([])
const fileInput = ref<HTMLInputElement | null>(null)

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
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: 'Failed to load emails.',
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function onPage(event: DataTablePageEvent) {
  page.value = event.page
  size.value = event.rows
  fetchEmails()
}

function openDialog() {
  subject.value = ''
  htmlBody.value = ''
  subjectError.value = ''
  bodyError.value = ''
  attachments.value = []
  dialogVisible.value = true
}

function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement
  if (input.files) attachments.value = Array.from(input.files)
}

function removeAttachment(index: number) {
  attachments.value = attachments.value.filter((_, i) => i !== index)
}

function validate(): boolean {
  subjectError.value = ''
  bodyError.value = ''
  let valid = true
  if (!subject.value.trim()) {
    subjectError.value = 'Subject is required'
    valid = false
  }
  const stripped = htmlBody.value.replace(/<[^>]*>/g, '').trim()
  if (!stripped) {
    bodyError.value = 'Body is required'
    valid = false
  }
  return valid
}

async function onSend() {
  if (!validate()) return
  sendLoading.value = true
  try {
    await emailsService.sendToMember(
      props.memberId,
      { subject: subject.value, htmlBody: htmlBody.value },
      attachments.value.length ? attachments.value : undefined,
    )
    toast.add({
      severity: 'success',
      summary: 'Sent',
      detail: 'Email sent successfully.',
      life: 3000,
    })
    dialogVisible.value = false
    fetchEmails()
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: 'Failed to send email.',
      life: 3000,
    })
  } finally {
    sendLoading.value = false
  }
}

function wrapEmailBody(body: string): string {
  return `<!DOCTYPE html>
  <html>
    <head>
      <meta charset="utf-8" />
      <meta name="viewport" content="width=device-width, initial-scale=1" />
      <style>
        body {
          margin: 12px;
          font-family: sans-serif;
          font-size: 14px;
          line-height: 1.5;
          color: #1a1a1a;
          word-break: break-word;
        }
        img { max-width: 100%; height: auto; }
      </style>
    </head>
    <body>${body}</body>
  </html>`
}

async function openPreview(email: EmailResponse) {
  previewVisible.value = true
  previewLoading.value = true
  previewEmail.value = email
  try {
    const full = await emailsService.getById(email.id)
    previewEmail.value = full
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
            <i class="pi pi-envelope text-primary-500 dark:text-primary-400" />
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
            <Button label="Send Email" icon="pi pi-send" size="small" @click="openDialog" />
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
          <Column field="subject" header="Subject">
            <template #body="{ data }: { data: EmailResponse }">
              <span class="font-medium text-surface-900 dark:text-surface-0 text-sm">
                {{ data.subject }}
              </span>
            </template>
          </Column>

          <Column field="provider" header="Provider">
            <template #body="{ data }: { data: EmailResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm capitalize">
                {{ data.provider }}
              </span>
            </template>
          </Column>

          <Column field="createdAt" header="Sent At">
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

          <Column style="width: 4rem">
            <template #body="{ data }: { data: EmailResponse }">
              <Button
                icon="pi pi-eye"
                severity="secondary"
                text
                rounded
                size="small"
                aria-label="View email"
                @click="openPreview(data)"
              />
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

  <!-- Send Email Dialog -->
  <Dialog
    v-model:visible="dialogVisible"
    header="Send Email"
    :style="{ width: '900px' }"
    :breakpoints="{ '1024px': '90vw', '768px': '95vw' }"
    modal
    :draggable="false"
  >
    <div class="space-y-5 py-2">
      <!-- Subject -->
      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          Subject <span class="text-red-500">*</span>
        </label>
        <InputText
          v-model="subject"
          placeholder="Enter email subject"
          :invalid="!!subjectError"
          fluid
          @input="subjectError = ''"
        />
        <Message v-if="subjectError" severity="error" size="small" variant="simple">
          {{ subjectError }}
        </Message>
      </div>

      <!-- Body -->
      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          Body <span class="text-red-500">*</span>
        </label>
        <Editor v-model="htmlBody" editor-style="height: 260px" @text-change="bodyError = ''" />
        <Message v-if="bodyError" severity="error" size="small" variant="simple">
          {{ bodyError }}
        </Message>
      </div>

      <!-- Attachments -->
      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          Attachments
        </label>
        <div class="flex items-center gap-2">
          <Button
            label="Add files"
            icon="pi pi-paperclip"
            severity="secondary"
            outlined
            size="small"
            @click="fileInput?.click()"
          />
          <span class="text-xs text-surface-400">
            {{
              attachments.length === 0
                ? 'No files selected'
                : `${attachments.length} file(s) selected`
            }}
          </span>
          <input ref="fileInput" type="file" multiple class="hidden" @change="onFileChange" />
        </div>

        <div v-if="attachments.length > 0" class="flex flex-wrap gap-2 mt-1">
          <div
            v-for="(file, i) in attachments"
            :key="i"
            class="flex items-center gap-1.5 px-2 py-1 rounded-full bg-surface-100 dark:bg-surface-800 text-xs text-surface-700 dark:text-surface-300"
          >
            <i class="pi pi-file text-xs"></i>
            <span class="max-w-32 truncate">{{ file.name }}</span>
            <button
              class="ml-1 text-surface-400 hover:text-red-500 transition-colors"
              aria-label="Remove attachment"
              @click="removeAttachment(i)"
            >
              <i class="pi pi-times text-xs"></i>
            </button>
          </div>
        </div>
      </div>
    </div>

    <template #footer>
      <div class="flex items-center justify-end gap-2">
        <Button label="Cancel" severity="secondary" outlined @click="dialogVisible = false" />
        <Button
          label="Send"
          icon="pi pi-send"
          iconPos="right"
          :loading="sendLoading"
          @click="onSend"
        />
      </div>
    </template>
  </Dialog>

  <!-- Preview Email Dialog -->
  <Dialog
    v-model:visible="previewVisible"
    :header="previewEmail?.subject ?? 'Email'"
    :style="{ width: '720px' }"
    :breakpoints="{ '768px': '95vw' }"
    modal
    :draggable="false"
  >
    <div v-if="previewLoading" class="flex items-center justify-center py-16">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400" />
    </div>

    <div v-else-if="previewEmail" class="space-y-4 py-2">
      <div class="grid grid-cols-2 gap-3 text-sm">
        <div class="flex flex-col gap-0.5">
          <span class="text-xs font-medium text-surface-400 uppercase tracking-wide">Sent At</span>
          <span class="text-surface-700 dark:text-surface-300">
            {{
              new Date(previewEmail.createdAt).toLocaleDateString('es-ES', {
                day: 'numeric',
                month: 'long',
                year: 'numeric',
              })
            }}
          </span>
        </div>
        <div class="flex flex-col gap-0.5">
          <span class="text-xs font-medium text-surface-400 uppercase tracking-wide">Provider</span>
          <span class="text-surface-700 dark:text-surface-300 capitalize">
            {{ previewEmail.provider }}
          </span>
        </div>
      </div>

      <div class="border-t border-surface-100 dark:border-surface-800"></div>

      <div class="flex flex-col gap-1.5">
        <span class="text-xs font-medium text-surface-400 uppercase tracking-wide">Body</span>
        <iframe
          :srcdoc="wrapEmailBody(previewEmail.body)"
          class="w-full rounded-lg border border-surface-200 dark:border-surface-700"
          style="height: 400px"
          sandbox="allow-same-origin"
          title="Email body preview"
        ></iframe>
      </div>
    </div>

    <template #footer>
      <Button label="Close" severity="secondary" outlined @click="previewVisible = false" />
    </template>
  </Dialog>
</template>
