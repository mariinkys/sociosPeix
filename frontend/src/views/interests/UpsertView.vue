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
    errors.name = [{ message: t('interests.form.name.required') }]
  } else if (String(values.name).length > 100) {
    errors.name = [{ message: t('interests.form.name.max') }]
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
        summary: t('common.saved'),
        detail: t('interests.toasts.updated'),
        life: 3000,
      })
    } else {
      await interestsService.create(model.value)
      toast.add({
        severity: 'success',
        summary: t('common.created'),
        detail: t('interests.toasts.created'),
        life: 3000,
      })
    }
    router.push('/interests')
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: isEdit.value ? t('interests.toasts.updateError') : t('interests.toasts.createError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function confirmDelete() {
  confirm.require({
    message: t('interests.deleteDialog.messageGeneric'),
    header: t('interests.deleteDialog.header'),
    icon: 'pi pi-exclamation-triangle',
    rejectProps: {
      label: t('interests.deleteDialog.reject'),
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: {
      label: t('interests.deleteDialog.accept'),
      severity: 'danger',
    },
    accept: async () => {
      deleteLoading.value = true
      try {
        await interestsService.delete(interestId.value!)
        toast.add({
          severity: 'success',
          summary: t('common.deleted'),
          detail: t('interests.deleteDialog.deletedSuccess'),
          life: 3000,
        })
        router.push('/interests')
      } catch {
        toast.add({
          severity: 'error',
          summary: t('common.error'),
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
      summary: t('common.error'),
      detail: t('interests.toasts.loadError'),
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
          :aria-label="t('interests.actions.goBack')"
          @click="router.push('/interests')"
        />
        <div>
          <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
            {{ isEdit ? t('interests.page.editTitle') : t('interests.page.newTitle') }}
          </h1>
          <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
            {{ isEdit ? t('interests.page.editDescription') : t('interests.page.newDescription') }}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2 shrink-0">
        <Button
          v-if="isEdit"
          icon="pi pi-trash"
          severity="danger"
          outlined
          :loading="deleteLoading"
          :aria-label="t('interests.actions.deleteInterest')"
          @click="confirmDelete"
        />
      </div>
    </div>

    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
    </div>

    <Card v-else class="border border-surface-200 dark:border-surface-700 shadow-sm">
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
              {{ t('interests.form.name.label') }} <span class="text-red-500">*</span>
            </label>
            <InputText
              v-model="model.name"
              :placeholder="t('interests.form.name.placeholder')"
              :invalid="$field?.invalid"
              fluid
            />
            <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
              {{ $field.error?.message }}
            </Message>
          </FormField>

          <FormField v-slot="$field" name="description" class="flex flex-col gap-1.5">
            <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
              {{ t('interests.form.description.label') }}
            </label>
            <Textarea
              v-model="model.description"
              :placeholder="t('interests.form.description.placeholder')"
              :invalid="$field?.invalid"
              rows="4"
              fluid
            />
            <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
              {{ $field.error?.message }}
            </Message>
          </FormField>

          <div class="flex items-center justify-end gap-3 pt-2">
            <Button
              :label="t('common.cancel')"
              severity="secondary"
              outlined
              @click="router.push('/interests')"
            />
            <Button
              type="submit"
              :label="isEdit ? t('common.saveChanges') : t('interests.form.submitCreate')"
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
