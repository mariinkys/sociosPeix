<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter, useRoute } from 'vue-router'
import { Form, FormField } from '@primevue/forms'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Button from 'primevue/button'
import Message from 'primevue/message'
import ConfirmDialog from 'primevue/confirmdialog'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { interestsService } from '@/services/interests.service'
import type { InterestPayload } from '@/types/interest.types'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()
const toast = useToast()
const confirm = useConfirm()

const interestId = computed(() => {
  const id = route.params.id
  return id ? Number(id) : null
})
const isEdit = computed(() => !!interestId.value)
const loading = ref(false)
const fetchLoading = ref(!!route.params.id)
const deleteLoading = ref(false)

const model = ref<InterestPayload>({
  name: '',
  description: null,
})

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  if (!values.name) {
    errors.name = [{ message: t('interests.fields.name.required') }]
  } else if (String(values.name).length > 100) {
    errors.name = [{ message: t('interests.fields.name.max') }]
  }

  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return

  loading.value = true
  try {
    if (isEdit.value) {
      await interestsService.update(interestId.value!, model.value)
      toast.add({
        severity: 'success',
        summary: t('common.feedback.saved'),
        detail: t('interests.messages.updated'),
        life: 3000,
      })
    } else {
      await interestsService.create(model.value)
      toast.add({
        severity: 'success',
        summary: t('common.feedback.created'),
        detail: t('interests.messages.created'),
        life: 3000,
      })
    }
    router.push('/interests')
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: isEdit.value
        ? t('interests.messages.updateError')
        : t('interests.messages.createError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function confirmDelete() {
  confirm.require({
    message: t('interests.deleteDialog.messageGeneric'),
    header: t('interests.deleteDialog.title'),
    icon: 'pi pi-exclamation-triangle',
    rejectProps: {
      label: t('common.actions.cancel'),
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: {
      label: t('common.actions.delete'),
      severity: 'danger',
    },
    accept: async () => {
      deleteLoading.value = true
      try {
        await interestsService.delete(interestId.value!)
        toast.add({
          severity: 'success',
          summary: t('common.feedback.deleted'),
          detail: t('interests.deleteDialog.deletedSuccess'),
          life: 3000,
        })
        router.push('/interests')
      } catch {
        toast.add({
          severity: 'error',
          summary: t('common.feedback.error'),
          detail: t('interests.deleteDialog.error'),
          life: 3000,
        })
      } finally {
        deleteLoading.value = false
      }
    },
  })
}

onMounted(async () => {
  if (!isEdit.value) return
  try {
    const interest = await interestsService.getById(interestId.value!)
    model.value = {
      name: interest.name,
      description: interest.description,
    }
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('interests.messages.loadError'),
      life: 3000,
    })
    router.push('/interests')
  } finally {
    fetchLoading.value = false
  }
})
</script>

<template>
  <div class="p-6 max-w-2xl mx-auto space-y-6">
    <div class="flex items-center justify-between gap-3 flex-wrap">
      <div class="flex items-center gap-3">
        <Button
          icon="pi pi-arrow-left"
          severity="secondary"
          text
          rounded
          :aria-label="t('common.actions.back')"
          @click="router.push('/interests')"
        />
        <div>
          <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
            {{ isEdit ? t('interests.titles.edit') : t('interests.titles.create') }}
          </h1>
          <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
            {{ isEdit ? t('interests.descriptions.edit') : t('interests.descriptions.create') }}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2 shrink-0">
        <Button
          v-if="isEdit"
          data-tour="interest-delete"
          icon="pi pi-trash"
          severity="danger"
          outlined
          :loading="deleteLoading"
          :aria-label="t('interests.actions.delete')"
          @click="confirmDelete"
        />
      </div>
    </div>

    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
    </div>

    <Card
      v-else
      class="border border-surface-200 dark:border-surface-700 shadow-sm"
      data-tour="interest-form"
    >
      <template #content>
        <Form
          v-slot="$form"
          :initialValues="model"
          :resolver
          :validateOnBlur="true"
          :validateOnValueUpdate="true"
          class="p-2 space-y-6"
          @submit="onSubmit"
        >
          <ConfirmDialog />

          <FormField v-slot="$field" name="name" class="flex flex-col gap-1.5">
            <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
              {{ t('interests.fields.name.label') }} <span class="text-red-500">*</span>
            </label>
            <InputText
              v-model="model.name"
              :placeholder="t('interests.fields.name.placeholder')"
              :invalid="$field?.invalid"
              fluid
            />
            <Message v-if="$field?.invalid" severity="error" variant="simple">
              {{ $field.error?.message }}
            </Message>
          </FormField>

          <FormField v-slot="$field" name="description" class="flex flex-col gap-1.5">
            <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
              {{ t('interests.fields.description.label') }}
            </label>
            <Textarea
              v-model="model.description"
              :placeholder="t('interests.fields.description.placeholder')"
              :invalid="$field?.invalid"
              rows="4"
              fluid
            />
            <Message v-if="$field?.invalid" severity="error" variant="simple">
              {{ $field.error?.message }}
            </Message>
          </FormField>

          <div class="flex items-center justify-end gap-3 pt-2">
            <Button
              :label="t('common.actions.cancel')"
              severity="secondary"
              outlined
              @click="router.push('/interests')"
            />
            <Button
              type="submit"
              data-tour="interest-save"
              :label="isEdit ? t('common.actions.saveChanges') : t('interests.actions.create')"
              :icon="isEdit ? 'pi pi-check' : 'pi pi-plus'"
              iconPos="right"
              :loading="loading"
              :disabled="!$form.valid"
            />
          </div>
        </Form>
      </template>
    </Card>
  </div>
</template>
