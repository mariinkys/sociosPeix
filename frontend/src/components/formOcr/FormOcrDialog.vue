<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import Dialog from 'primevue/dialog'
import Button from 'primevue/button'
import Message from 'primevue/message'
import InputText from 'primevue/inputtext'
import { useToast } from 'primevue/usetoast'
import { formOcrService } from '@/services/formOcr.service'
import { membersService } from '@/services/members.service'
import GenderSelect from '@/components/gender/SelectorComponent.vue'
import CountrySelect from '@/components/country/SelectorComponent.vue'
import type { FormOcrPage } from '@/types/formOcr.types'
import type { MemberCreatePayload } from '@/types/member.types'

const emit = defineEmits<{ refresh: [] }>()

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()

const visible = defineModel<boolean>('visible', { required: true })

const step = ref<'upload' | 'review'>('upload')

const file = ref<File | null>(null)
const fileInput = ref<HTMLInputElement | null>(null)
const fileError = ref('')
const analyzeLoading = ref(false)

const pages = ref<FormOcrPage[]>([])
const currentIndex = ref(0)
const createLoading = ref(false)

const form = ref<{
  name: string
  surname: string
  secondSurname: string
  birthdate: string
  phone: string
  email: string
  genderId: number | null
  countryId: number | null
}>({
  name: '',
  surname: '',
  secondSurname: '',
  birthdate: '',
  phone: '',
  email: '',
  genderId: null,
  countryId: null,
})


function pageToForm(page: FormOcrPage) {
  form.value = {
    name: capitalizeFirstLetter(page.fields.NOMBRE.text),
    surname: capitalizeFirstLetter(page.fields.APELLIDO.text),
    secondSurname: capitalizeFirstLetter(page.fields.SEGUNDO_APELLIDO.text),
    birthdate: page.fields.FECHA_NACIMIENTO.text,
    phone: page.fields.TELEFONO.text,
    email: page.fields.CORREO_ELECTRONICO.text,
    genderId: null,
    countryId: null,
  }
}

function capitalizeFirstLetter(string: string) {
  return string.charAt(0).toUpperCase() + string.slice(1).toLowerCase();
}

function reset() {
  step.value = 'upload'
  file.value = null
  fileError.value = ''
  analyzeLoading.value = false
  pages.value = []
  currentIndex.value = 0
  createLoading.value = false
  form.value = {
    name: '',
    surname: '',
    secondSurname: '',
    birthdate: '',
    phone: '',
    email: '',
    genderId: null,
    countryId: null,
  }
}

function onFileChange(event: Event) {
  const input = event.target as HTMLInputElement
  if (!input.files?.length) return
  const selected = input.files[0]
  if (!selected) return
  if (selected.type !== 'application/pdf') {
    fileError.value = t('formOcr.dialog.validation.pdfOnly')
    input.value = ''
    return
  }
  fileError.value = ''
  file.value = selected
  input.value = ''
}

function removeFile() {
  file.value = null
  fileError.value = ''
}

async function onAnalyze() {
  if (!file.value) {
    fileError.value = t('formOcr.dialog.validation.fileRequired')
    return
  }
  analyzeLoading.value = true
  try {
    const result: FormOcrPage[] = await formOcrService.analyze(file.value as File)
    if (!result.length) {
      toast.add({
        severity: 'warn',
        summary: t('common.feedback.warn'),
        detail: t('formOcr.dialog.messages.noPages'),
        life: 3000,
      })
      return
    }
    pages.value = result
    currentIndex.value = 0
    pageToForm(result[0]!)
    step.value = 'review'
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('formOcr.dialog.messages.error'),
      life: 3000,
    })
  } finally {
    analyzeLoading.value = false
  }
}

async function onCreateMember() {
  createLoading.value = true
  const isLast = currentIndex.value === pages.value.length - 1

  const payload: MemberCreatePayload = {
    name: form.value.name,
    surname: form.value.surname,
    secondSurname: form.value.secondSurname || null,
    email: form.value.email,
    birthdate: isDateValid(form.value.birthdate) ? form.value.birthdate : null,
    phone: form.value.phone || null,
    genderId: form.value.genderId,
    countryId: form.value.countryId,
    notes: null,
    interestIds: [],
  }

  try {
    await membersService.create(payload)
    toast.add({
      severity: 'success',
      summary: t('common.feedback.success'),
      detail: t('formOcr.dialog.messages.memberCreated', {
        name: `${payload.name} ${payload.surname}`,
      }),
      life: 3000,
    })

    if (isLast) {
      visible.value = false
      emit('refresh')
    } else {
      currentIndex.value++
      pageToForm(pages.value[currentIndex.value]!)
    }
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('formOcr.dialog.messages.createError'),
      life: 3000,
    })
  } finally {
    createLoading.value = false
  }
}

function isDateValid(dateStr: string): boolean {
  return !Number.isNaN(new Date(dateStr).getTime());
}

function onSkip() {
  const isLast = currentIndex.value === pages.value.length - 1
  if (isLast) {
    visible.value = false
    emit('refresh')
  } else {
    currentIndex.value++
    pageToForm(pages.value[currentIndex.value]!)
  }
}
</script>

<template>
  <Dialog
    v-model:visible="visible"
    :header="
      step === 'upload'
        ? t('formOcr.dialog.title')
        : t('formOcr.dialog.reviewTitle', {
            current: currentIndex + 1,
            total: pages.length,
          })
    "
    :style="{ width: '520px' }"
    :breakpoints="{ '768px': '95vw' }"
    modal
    :draggable="false"
    @hide="reset"
  >
    <div v-if="step === 'upload'" class="space-y-4 py-2">
      <p class="text-sm text-surface-500 dark:text-surface-400">
        {{ t('formOcr.dialog.description') }}
      </p>

      <div class="flex flex-col gap-1.5">
        <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
          {{ t('formOcr.dialog.fields.file') }}
          <span class="text-red-500">*</span>
        </label>

        <div
          v-if="!file"
          class="flex flex-col items-center justify-center gap-2 rounded-lg border-2 border-dashed border-surface-200 dark:border-surface-700 py-8 px-4 cursor-pointer hover:border-primary-400 dark:hover:border-primary-500 transition-colors"
          @click="fileInput?.click()"
        >
          <i class="pi pi-file-pdf text-3xl text-surface-400"></i>
          <span class="text-sm text-surface-500 dark:text-surface-400">
            {{ t('formOcr.dialog.upload.prompt') }}
          </span>
          <span class="text-xs text-surface-400">
            {{ t('formOcr.dialog.upload.hint') }}
          </span>
        </div>

        <div
          v-else
          class="flex items-center gap-3 px-3 py-2.5 rounded-lg bg-surface-50 dark:bg-surface-800 border border-surface-200 dark:border-surface-700"
        >
          <i class="pi pi-file-pdf text-xl text-primary-500"></i>
          <span class="flex-1 text-sm text-surface-700 dark:text-surface-300 truncate">
            {{ file.name }}
          </span>
          <button
            class="text-surface-400 hover:text-red-500 transition-colors"
            :aria-label="t('common.actions.delete')"
            @click="removeFile"
          >
            <i class="pi pi-times text-sm"></i>
          </button>
        </div>

        <input
          ref="fileInput"
          type="file"
          accept="application/pdf"
          class="hidden"
          @change="onFileChange"
        />

        <Message v-if="fileError" severity="error" variant="simple">
          {{ fileError }}
        </Message>
      </div>
    </div>

    <div v-else class="space-y-4 py-2">
      <p class="text-sm text-surface-500 dark:text-surface-400">
        {{ t('formOcr.dialog.reviewDescription') }}
      </p>

      <!-- Progress indicator -->
      <div class="flex gap-1">
        <div
          v-for="(_, i) in pages"
          :key="i"
          class="h-1 flex-1 rounded-full transition-colors"
          :class="
            i < currentIndex
              ? 'bg-green-400'
              : i === currentIndex
                ? 'bg-primary-500'
                : 'bg-surface-200 dark:bg-surface-700'
          "
        />
      </div>

      <div class="grid grid-cols-1 sm:grid-cols-2 gap-3">
        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('common.fields.name') }}
          </label>
          <InputText v-model="form.name" fluid />
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('common.fields.surname') }}
          </label>
          <InputText v-model="form.surname" fluid />
        </div>

        <div class="flex flex-col gap-1.5 sm:col-span-2">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('common.fields.secondSurname') }}
          </label>
          <InputText v-model="form.secondSurname" fluid />
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('common.fields.birthdate') }}
          </label>
          <InputText v-model="form.birthdate" type="date" :placeholder="t('common.placeholders.dateFormat')" fluid />
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('common.fields.phone') }}
          </label>
          <InputText v-model="form.phone" fluid />
        </div>

        <div class="flex flex-col gap-1.5 sm:col-span-2">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('common.fields.email') }}
          </label>
          <InputText v-model="form.email" fluid />
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('common.fields.gender') }}
          </label>
          <GenderSelect v-model="form.genderId" :initial-name="pages[currentIndex]?.fields.GENERO.text" />
        </div>

        <div class="flex flex-col gap-1.5">
          <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
            {{ t('common.fields.country') }}
          </label>
          <CountrySelect v-model="form.countryId" :initial-name="pages[currentIndex]?.fields.PAIS.text" />
        </div>
      </div>
    </div>

    <template #footer>
      <div v-if="step === 'upload'" class="flex items-center justify-end gap-2">
        <Button
          :label="t('common.actions.cancel')"
          severity="secondary"
          outlined
          @click="visible = false"
        />
        <Button
          :label="t('formOcr.dialog.actions.analyze')"
          icon="pi pi-search"
          iconPos="right"
          :loading="analyzeLoading"
          :disabled="!file"
          @click="onAnalyze"
        />
      </div>

      <div v-else class="flex items-center justify-between w-full">
        <Button
          :label="t('formOcr.dialog.actions.skip')"
          severity="secondary"
          outlined
          :disabled="createLoading"
          @click="onSkip"
        />
        <Button
          :label="
            currentIndex === pages.length - 1
              ? t('formOcr.dialog.actions.createAndFinish')
              : t('formOcr.dialog.actions.createAndNext')
          "
          icon="pi pi-user-plus"
          iconPos="right"
          :loading="createLoading"
          @click="onCreateMember"
        />
      </div>
    </template>
  </Dialog>
</template>
