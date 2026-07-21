<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import RichTextEditor from '@/components/richTextEditor/RichTextEditor.vue'
import Message from 'primevue/message'
import { useToast } from 'primevue/usetoast'
import { emailsService } from '@/services/emails.service'
import { applyTemplate, wrapEmailBody, getTemplateOptions } from '@/utils/emailTemplates'
import InterestsSelect from '@/components/interest/MultiSelect.vue'
import type { EmailTemplate } from '@/utils/emailTemplates'
import type { MultiEmailCheckResponse } from '@/types/email.types'

const props = defineProps<{
  mode: 'member' | 'interests' | 'all'
  memberId?: string
}>()

const emit = defineEmits<{ sent: [] }>()

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()

const visible = defineModel<boolean>('visible', { required: true })

const step = ref<'compose' | 'preview'>('compose')
const sendLoading = ref(false)

const template = ref<EmailTemplate>('none')
const subject = ref('')
const htmlBody = ref('')
const interestIds = ref<number[]>([])
const subjectError = ref('')
const bodyError = ref('')
const interestError = ref('')
const attachments = ref<File[]>([])
const attachmentError = ref('')
const fileInput = ref<HTMLInputElement | null>(null)

const checkLoading = ref(false)
const checkResult = ref<MultiEmailCheckResponse | null>(null)
const checkFailed = ref(false)

const MAX_ATTACHMENT_BYTES = 5 * 1024 * 1024

const totalAttachmentSize = computed(() => attachments.value.reduce((sum, f) => sum + f.size, 0))
const templateOptions = computed(() => {
  return getTemplateOptions()
})

const needsCapacityCheck = computed(() => props.mode !== 'member')

const sendDisabled = computed(() => {
  if (!needsCapacityCheck.value) return false
  if (checkLoading.value) return true
  return checkResult.value?.exceedsLimit ?? false
})

function formatSize(bytes: number): string {
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(0)} KB`
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`
}

const renderedHtml = computed(() => wrapEmailBody(applyTemplate(htmlBody.value, template.value)))

const dialogTitle = computed(() =>
  step.value === 'preview'
    ? t('email.sendEmailDialog.titles.preview', { subject: subject.value })
    : props.mode === 'interests'
      ? t('email.sendEmailDialog.titles.byInterest')
      : props.mode === 'all'
        ? t('email.sendEmailDialog.titles.toAll')
        : t('email.sendEmailDialog.titles.default'),
)

function reset() {
  step.value = 'compose'
  template.value = 'none'
  subject.value = ''
  htmlBody.value = ''
  interestIds.value = []
  subjectError.value = ''
  bodyError.value = ''
  interestError.value = ''
  attachments.value = []
  attachmentError.value = ''
  checkLoading.value = false
  checkResult.value = null
  checkFailed.value = false
}

function validate(): boolean {
  subjectError.value = ''
  bodyError.value = ''
  interestError.value = ''
  let valid = true
  if (!subject.value.trim()) {
    subjectError.value = t('email.sendEmailDialog.validation.subjectRequired')
    valid = false
  }
  if (!htmlBody.value.replace(/<[^>]*>/g, '').trim()) {
    bodyError.value = t('email.sendEmailDialog.validation.bodyRequired')
    valid = false
  }
  if (props.mode === 'interests' && interestIds.value.length === 0) {
    interestError.value = t('email.sendEmailDialog.validation.interestsRequired')
    valid = false
  }
  if (attachmentError.value) valid = false
  return valid
}

async function fetchCapacityCheck() {
  checkLoading.value = true
  checkFailed.value = false
  checkResult.value = null
  try {
    checkResult.value = await emailsService.checkMultiSend(
      props.mode === 'interests' ? interestIds.value : undefined,
    )
  } catch {
    checkFailed.value = true
  } finally {
    checkLoading.value = false
  }
}

function goToPreview() {
  if (!validate()) return
  step.value = 'preview'
  if (needsCapacityCheck.value) {
    fetchCapacityCheck()
  }
}

function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement
  if (!input.files) return
  const combined = [...attachments.value, ...Array.from(input.files)]
  const total = combined.reduce((sum, f) => sum + f.size, 0)
  if (total > MAX_ATTACHMENT_BYTES) {
    attachmentError.value = t('email.sendEmailDialog.validation.attachmentLimitExceeded', {
      size: formatSize(total),
    })
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
  if (sendDisabled.value) return

  sendLoading.value = true
  try {
    if (props.mode === 'interests') {
      await emailsService.sendToInterests(
        { subject: subject.value, htmlBody: renderedHtml.value, interestIds: interestIds.value },
        attachments.value.length ? attachments.value : undefined,
      )
    } else if (props.mode === 'all') {
      await emailsService.sendToAll(
        { subject: subject.value, htmlBody: renderedHtml.value },
        attachments.value.length ? attachments.value : undefined,
      )
    } else {
      await emailsService.sendToMember(
        props.memberId!,
        { subject: subject.value, htmlBody: renderedHtml.value },
        attachments.value.length ? attachments.value : undefined,
      )
    }
    toast.add({
      severity: 'success',
      summary: t('common.feedback.sent'),
      detail: t('email.sendEmailDialog.messages.sentDetail'),
      life: 3000,
    })
    visible.value = false
    emit('sent')
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('email.sendEmailDialog.messages.sendError'),
      life: 3000,
    })
  } finally {
    sendLoading.value = false
  }
}
</script>

<template>
  <Dialog
    v-model:visible="visible"
    :header="dialogTitle"
    :style="{ width: '900px' }"
    :breakpoints="{ '1024px': '90vw', '768px': '95vw' }"
    modal
    :draggable="false"
    @hide="reset"
  >
    <div v-show="step === 'compose'" class="space-y-5 py-2">
      <!-- unchanged compose step - template, interests, subject, body, attachments -->
      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">{{
          t('common.fields.template')
        }}</label>
        <div class="flex gap-2">
          <button
            v-for="opt in templateOptions"
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

      <div v-if="mode === 'interests'" class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('common.fields.interests') }}
          <span class="text-red-500">*</span>
        </label>
        <InterestsSelect
          v-model="interestIds"
          :invalid="!!interestError"
          @update:modelValue="interestError = ''"
        />
        <Message v-if="interestError" severity="error" size="small" variant="simple">
          {{ interestError }}
        </Message>
        <p class="text-xs text-surface-400">
          {{ t('email.sendEmailDialog.help.interests') }}
        </p>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('common.fields.subject') }}
          <span class="text-red-500">*</span>
        </label>
        <InputText
          v-model="subject"
          :placeholder="t('common.placeholders.enterEmailSubject')"
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
          {{ t('common.fields.body') }}
          <span class="text-red-500">*</span>
        </label>
        <RichTextEditor
          v-model="htmlBody"
          :placeholder="t('email.sendEmailDialog.bodyPlaceholder')"
          @update:modelValue="bodyError = ''"
        />
        <Message v-if="bodyError" severity="error" size="small" variant="simple">
          {{ bodyError }}
        </Message>
      </div>

      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">{{
          t('common.fields.attachments')
        }}</label>
        <div class="flex items-center gap-2">
          <Button
            :label="t('common.actions.addFiles')"
            icon="pi pi-paperclip"
            severity="secondary"
            outlined
            size="small"
            @click="fileInput?.click()"
          />
          <span class="text-xs" :class="attachmentError ? 'text-red-500' : 'text-surface-400'">
            <template v-if="attachments.length === 0">{{
              t('email.sendEmailDialog.attachments.empty')
            }}</template>
            <template v-else>
              {{
                t('email.sendEmailDialog.attachments.summary', {
                  count: attachments.length,
                  size: formatSize(totalAttachmentSize),
                })
              }}
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
              :aria-label="t('common.actions.removeAttachment')"
              @click="removeAttachment(i)"
            >
              <i class="pi pi-times text-xs"></i>
            </button>
          </div>
        </div>
      </div>
    </div>

    <div v-show="step === 'preview'" class="py-2 space-y-3">
      <div class="flex items-center gap-2 text-sm text-surface-500 dark:text-surface-400">
        <i class="pi pi-info-circle"></i>
        <span>{{ t('email.descriptions.preview') }}</span>
      </div>

      <!-- capacity check, only for 'all' / 'interests' -->
      <div
        v-if="needsCapacityCheck"
        class="flex items-center gap-3 rounded-lg px-3 py-2.5"
        :class="
          checkResult?.exceedsLimit
            ? 'bg-red-50 dark:bg-red-950'
            : checkFailed
              ? 'bg-amber-50 dark:bg-amber-950'
              : 'bg-surface-50 dark:bg-surface-800'
        "
      >
        <template v-if="checkLoading">
          <i class="pi pi-spinner pi-spin text-lg text-surface-400"></i>
          <span class="text-sm text-surface-500 dark:text-surface-400">
            {{ t('email.sendEmailDialog.capacityCheck.checking') }}
          </span>
        </template>

        <template v-else-if="checkFailed">
          <i class="pi pi-exclamation-triangle text-lg text-amber-500"></i>
          <div class="flex items-center justify-between flex-1">
            <span class="text-sm text-amber-700 dark:text-amber-400">
              {{ t('email.sendEmailDialog.capacityCheck.checkFailed') }}
            </span>
            <Button
              :label="t('common.actions.retry')"
              text
              size="small"
              @click="fetchCapacityCheck"
            />
          </div>
        </template>

        <template v-else-if="checkResult">
          <i
            class="pi text-lg"
            :class="
              checkResult.exceedsLimit
                ? 'pi-times-circle text-red-500'
                : 'pi-check-circle text-green-500'
            "
          ></i>
          <div>
            <p class="text-sm font-semibold text-surface-900 dark:text-surface-0">
              <template v-if="checkResult.exceedsLimit">
                {{
                  t('email.sendEmailDialog.capacityCheck.exceeds', {
                    total: checkResult.totalRecipients,
                    remaining: checkResult.remaining,
                  })
                }}
              </template>
              <template v-else>
                {{
                  t('email.sendEmailDialog.capacityCheck.ok', {
                    total: checkResult.totalRecipients,
                    remaining: checkResult.remaining,
                  })
                }}
              </template>
            </p>
            <p class="text-xs text-surface-400 mt-0.5">
              {{
                t('email.sendEmailDialog.capacityCheck.providerDetail', {
                  provider: checkResult.provider,
                  sent: checkResult.sentToday,
                  limit: checkResult.dailyLimit,
                })
              }}
            </p>
          </div>
        </template>
      </div>

      <iframe
        :srcdoc="renderedHtml"
        class="w-full rounded-lg border border-surface-200 dark:border-surface-700"
        style="height: 520px"
        sandbox="allow-same-origin"
        :title="t('email.sendEmailDialog.preview.iframeTitle')"
      ></iframe>
    </div>

    <template #footer>
      <div v-if="step === 'compose'" class="flex items-center justify-end gap-2">
        <Button
          :label="t('common.actions.cancel')"
          severity="secondary"
          outlined
          @click="visible = false"
        />
        <Button
          :label="t('common.actions.preview')"
          icon="pi pi-eye"
          severity="secondary"
          @click="goToPreview"
        />
      </div>
      <div v-else class="flex items-center justify-between w-full">
        <Button
          :label="t('email.sendEmailDialog.actions.backToEdit')"
          icon="pi pi-arrow-left"
          severity="secondary"
          outlined
          @click="step = 'compose'"
        />
        <Button
          :label="t('common.actions.send')"
          icon="pi pi-send"
          iconPos="right"
          :loading="sendLoading"
          :disabled="sendDisabled"
          @click="onSend"
        />
      </div>
    </template>
  </Dialog>
</template>
