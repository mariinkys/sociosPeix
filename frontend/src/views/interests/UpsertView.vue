<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Form, FormField } from '@primevue/forms'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Textarea from 'primevue/textarea'
import Button from 'primevue/button'
import Message from 'primevue/message'
import { useToast } from 'primevue/usetoast'
import { interestsService } from '@/services/interests.service'
import type { InterestPayload } from '@/types/interest.types'

const router = useRouter()
const route = useRoute()
const toast = useToast()

const interestId = computed(() => {
  const id = route.params.id
  return id ? Number(id) : null
})
const isEdit = computed(() => !!interestId.value)
const loading = ref(false)
const fetchLoading = ref(!!route.params.id)

const model = ref<InterestPayload>({
  name: '',
  description: null,
})

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  if (!values.name) {
    errors.name = [{ message: 'Name is required' }]
  } else if (String(values.name).length > 100) {
    errors.name = [{ message: 'Name must not exceed 100 characters' }]
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
        summary: 'Saved',
        detail: 'Interest updated successfully',
        life: 3000,
      })
    } else {
      await interestsService.create(model.value)
      toast.add({
        severity: 'success',
        summary: 'Created',
        detail: 'Interest created successfully',
        life: 3000,
      })
    }
    router.push('/interests')
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: isEdit.value ? 'Failed to update interest' : 'Failed to create interest',
      life: 3000,
    })
  } finally {
    loading.value = false
  }
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
      summary: 'Error',
      detail: 'Failed to load interest',
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
    <div class="flex items-center gap-3">
      <Button
        icon="pi pi-arrow-left"
        severity="secondary"
        text
        rounded
        aria-label="Go back"
        @click="router.push('/interests')"
      />
      <div>
        <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
          {{ isEdit ? 'Edit Interest' : 'New Interest' }}
        </h1>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
          {{
            isEdit
              ? "Update the interest's details below"
              : 'Fill in the details to create a new interest'
          }}
        </p>
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
          <FormField v-slot="$field" name="name" class="flex flex-col gap-1.5">
            <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
              Name <span class="text-red-500">*</span>
            </label>
            <InputText
              v-model="model.name"
              placeholder="Enter interest name"
              :invalid="$field?.invalid"
              fluid
            />
            <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
              {{ $field.error?.message }}
            </Message>
          </FormField>

          <FormField v-slot="$field" name="description" class="flex flex-col gap-1.5">
            <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
              Description
            </label>
            <Textarea
              v-model="model.description"
              placeholder="Enter a description"
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
              label="Cancel"
              severity="secondary"
              outlined
              @click="router.push('/interests')"
            />
            <Button
              type="submit"
              :label="isEdit ? 'Save Changes' : 'Create Interest'"
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
