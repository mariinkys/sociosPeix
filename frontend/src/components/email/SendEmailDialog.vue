<script setup lang="ts">
import { ref, computed } from 'vue'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import Editor from 'primevue/editor'
import Message from 'primevue/message'
import { useToast } from 'primevue/usetoast'
import { emailsService } from '@/services/emails.service'
import { applyTemplate, wrapEmailBody, TEMPLATE_OPTIONS } from '@/utils/emailTemplates'
import type { EmailTemplate } from '@/utils/emailTemplates'

const props = defineProps<{ memberId: string }>()
const emit = defineEmits<{ sent: [] }>()

const toast = useToast()

const visible = defineModel<boolean>('visible', { required: true })

const step = ref<'compose' | 'preview'>('compose')
const sendLoading = ref(false)

const template = ref<EmailTemplate>('none')
const subject = ref('')
const htmlBody = ref('')
const subjectError = ref('')
const bodyError = ref('')
const attachments = ref<File[]>([])
const attachmentError = ref('')
const fileInput = ref<HTMLInputElement | null>(null)

const MAX_ATTACHMENT_BYTES = 5 * 1024 * 1024 // 5 MB

const totalAttachmentSize = computed(() => attachments.value.reduce((sum, f) => sum + f.size, 0))

function formatSize(bytes: number): string {
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(0)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

const renderedHtml = computed(() => wrapEmailBody(applyTemplate(htmlBody.value, template.value)))

function reset() {
  step.value = 'compose'
  template.value = 'none'
  subject.value = ''
  htmlBody.value = ''
  subjectError.value = ''
  bodyError.value = ''
  attachments.value = []
  attachmentError.value = ''
}

function validate(): boolean {
  subjectError.value = ''
  bodyError.value = ''
  let valid = true
  if (!subject.value.trim()) {
    subjectError.value = 'Subject is required'
    valid = false
  }
  if (!htmlBody.value.replace(/<[^>]*>/g, '').trim()) {
    bodyError.value = 'Body is required'
    valid = false
  }
  if (attachmentError.value) valid = false
  return valid
}

function goToPreview() {
  if (validate()) step.value = 'preview'
}

function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement
  if (!input.files) return

  const incoming = Array.from(input.files)
  const combined = [...attachments.value, ...incoming]
  const total = combined.reduce((sum, f) => sum + f.size, 0)

  if (total > MAX_ATTACHMENT_BYTES) {
    attachmentError.value = `Total size exceeds the 5 MB limit (${formatSize(total)} selected)`
    input.value = ''
    return
  }

  attachmentError.value = ''
  attachments.value = combined
  input.value = ''
}

function removeAttachment(index: number) {
  attachments.value = attachments.value.filter((_, i) => i !== index)
  attachmentError.value = ''
}

async function onSend() {
  sendLoading.value = true
  try {
    await emailsService.sendToMember(
      props.memberId,
      { subject: subject.value, htmlBody: renderedHtml.value },
      attachments.value.length ? attachments.value : undefined,
    )
    toast.add({
      severity: 'success',
      summary: 'Sent',
      detail: 'Email sent successfully.',
      life: 3000,
    })
    visible.value = false
    emit('sent')
  } catch {
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to send email.', life: 3000 })
  } finally {
    sendLoading.value = false
  }
}
</script>

<template>
  <Dialog
    v-model:visible="visible"
    :header="step === 'compose' ? 'Send Email' : `Preview — ${subject}`"
    :style="{ width: '900px' }"
    :breakpoints="{ '1024px': '90vw', '768px': '95vw' }"
    modal
    :draggable="false"
    @hide="reset"
  >
    <div v-if="step === 'compose'" class="space-y-5 py-2">
      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">Template</label>
        <div class="flex gap-2">
          <button
            v-for="opt in TEMPLATE_OPTIONS"
            :key="opt.value"
            type="button"
            class="flex-1 flex flex-col items-center gap-1.5 py-3 px-2 rounded-lg border text-xs font-medium transition-colors"
            :class="
              template === opt.value
                ? 'border-primary-500 bg-primary-50 dark:bg-primary-950 text-primary-700 dark:text-primary-300'
                : 'border-surface-200 dark:border-surface-700 text-surface-500 dark:text-surface-400 hover:border-surface-400 dark:hover:border-surface-500'
            "
            @click="template = opt.value"
          >
            <i :class="`pi ${opt.icon} text-base`"></i>
            {{ opt.label }}
          </button>
        </div>
      </div>

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

      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          Body <span class="text-red-500">*</span>
        </label>
        <Editor v-model="htmlBody" editor-style="height: 260px" @text-change="bodyError = ''" />
        <Message v-if="bodyError" severity="error" size="small" variant="simple">
          {{ bodyError }}
        </Message>
      </div>

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
          <span class="text-xs" :class="attachmentError ? 'text-red-500' : 'text-surface-400'">
            <template v-if="attachments.length === 0">No files selected — max 5 MB total</template>
            <template v-else>
              {{ attachments.length }} file(s) — {{ formatSize(totalAttachmentSize) }} / 5 MB
            </template>
          </span>
          <input ref="fileInput" type="file" multiple class="hidden" @change="onFileChange" />
        </div>

        <Message v-if="attachmentError" severity="error" size="small" variant="simple">
          {{ attachmentError }}
        </Message>

        <div v-if="attachments.length > 0" class="flex flex-wrap gap-2 mt-1">
          <div
            v-for="(file, i) in attachments"
            :key="i"
            class="flex items-center gap-1.5 px-2 py-1 rounded-full bg-surface-100 dark:bg-surface-800 text-xs text-surface-700 dark:text-surface-300"
          >
            <i class="pi pi-file text-xs"></i>
            <span class="max-w-32 truncate">{{ file.name }}</span>
            <span class="text-surface-400">({{ formatSize(file.size) }})</span>
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

    <div v-else class="py-2 space-y-3">
      <div class="flex items-center gap-2 text-sm text-surface-500 dark:text-surface-400">
        <i class="pi pi-info-circle"></i>
        <span>This is how the email will look to the recipient.</span>
      </div>
      <iframe
        :srcdoc="renderedHtml"
        class="w-full rounded-lg border border-surface-200 dark:border-surface-700"
        style="height: 520px"
        sandbox="allow-same-origin"
        title="Email preview"
      ></iframe>
    </div>

    <template #footer>
      <div v-if="step === 'compose'" class="flex items-center justify-end gap-2">
        <Button label="Cancel" severity="secondary" outlined @click="visible = false" />
        <Button label="Preview" icon="pi pi-eye" severity="secondary" @click="goToPreview" />
      </div>
      <div v-else class="flex items-center justify-between w-full">
        <Button
          label="Back to edit"
          icon="pi pi-arrow-left"
          severity="secondary"
          outlined
          @click="step = 'compose'"
        />
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
</template>
