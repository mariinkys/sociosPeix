<!-- views/MemberUpsertView.vue -->
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
import ConfirmDialog from 'primevue/confirmdialog'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { membersService } from '@/services/members.service'
import type { MemberCreatePayload, MemberUpdatePayload } from '@/types/member.types'
import CountrySelect from '@/components/country/SelectorComponent.vue'
import GenderSelect from '@/components/gender/SelectorComponent.vue'
import InterestsSelect from '@/components/interest/MultiSelect.vue'

const router = useRouter()
const route = useRoute()
const toast = useToast()
const confirm = useConfirm()

const memberId = computed(() => route.params.id as string | undefined)
const isEdit = computed(() => !!memberId.value)
const loading = ref(false)
const fetchLoading = ref(!!route.params.id)
const deleteLoading = ref(false)

const model = ref<MemberCreatePayload>({
  name: '',
  surname: '',
  secondSurname: null,
  email: '',
  birthdate: null,
  phone: null,
  notes: null,
  genderId: null,
  countryId: null,
  interestIds: [],
})

async function fetchMember() {
  if (!isEdit.value) return
  fetchLoading.value = true
  try {
    const member = await membersService.getById(memberId.value!)
    model.value = {
      name: member.name,
      surname: member.surname,
      secondSurname: member.secondSurname,
      email: member.email,
      birthdate: member.birthdate,
      phone: member.phone,
      notes: member.notes,
      genderId: member.genderId,
      countryId: member.countryId,
      interestIds: member.interests.map((i) => i.id),
    }
  } catch {
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to load member', life: 3000 })
    router.push('/members')
  } finally {
    fetchLoading.value = false
  }
}

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  if (!values.name) {
    errors.name = [{ message: 'Name is required' }]
  } else if (String(values.name).length > 100) {
    errors.name = [{ message: 'Name must not exceed 100 characters' }]
  }

  if (!values.surname) {
    errors.surname = [{ message: 'Surname is required' }]
  } else if (String(values.surname).length > 100) {
    errors.surname = [{ message: 'Surname must not exceed 100 characters' }]
  }

  if (values.secondSurname && String(values.secondSurname).length > 100) {
    errors.secondSurname = [{ message: 'Second surname must not exceed 100 characters' }]
  }

  if (!values.email) {
    errors.email = [{ message: 'Email is required' }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(String(values.email))) {
    errors.email = [{ message: 'Email must be valid' }]
  }

  if (values.phone && String(values.phone).length > 30) {
    errors.phone = [{ message: 'Phone must not exceed 30 characters' }]
  }

  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return

  loading.value = true
  try {
    if (isEdit.value) {
      await membersService.update(memberId.value!, model.value as MemberUpdatePayload)
      toast.add({
        severity: 'success',
        summary: 'Saved',
        detail: 'Member updated successfully',
        life: 3000,
      })
    } else {
      await membersService.create(model.value)
      toast.add({
        severity: 'success',
        summary: 'Created',
        detail: 'Member created successfully',
        life: 3000,
      })
    }
    router.push('/members')
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: isEdit.value ? 'Failed to update member' : 'Failed to create member',
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function confirmDelete() {
  confirm.require({
    message: `Are you sure you want to delete this member? This action cannot be undone.`,
    header: 'Delete Member',
    icon: 'pi pi-exclamation-triangle',
    rejectProps: { label: 'Cancel', severity: 'secondary', outlined: true },
    acceptProps: { label: 'Delete', severity: 'danger' },
    accept: async () => {
      deleteLoading.value = true
      try {
        await membersService.delete(memberId.value!)
        toast.add({
          severity: 'success',
          summary: 'Deleted',
          detail: 'Member deleted successfully.',
          life: 3000,
        })
        router.push('/members')
      } catch {
        toast.add({
          severity: 'error',
          summary: 'Error',
          detail: 'Failed to delete member. Please try again.',
          life: 3000,
        })
      } finally {
        deleteLoading.value = false
      }
    },
  })
}

onMounted(async () => {
  fetchMember()
})
</script>

<template>
  <div class="p-6 space-y-6">
    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400" />
    </div>

    <Form
      v-else
      v-slot="$form"
      :initialValues="model"
      :resolver
      :validateOnBlur="true"
      :validateOnValueUpdate="true"
      class="space-y-6"
      @submit="onSubmit"
    >
      <ConfirmDialog />

      <div class="flex items-center justify-between gap-3 flex-wrap">
        <div class="flex items-center gap-3">
          <Button
            icon="pi pi-arrow-left"
            severity="secondary"
            text
            rounded
            @click="router.push('/members')"
            aria-label="Go back"
          />
          <div>
            <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
              {{ isEdit ? 'Edit Member' : 'New Member' }}
            </h1>
            <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
              {{
                isEdit
                  ? "Update the member's details below"
                  : 'Fill in the details to create a new member'
              }}
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <Button label="Cancel" severity="secondary" outlined @click="router.push('/members')" />
          <Button
            type="submit"
            :label="isEdit ? 'Save Changes' : 'Create Member'"
            :icon="isEdit ? 'pi pi-check' : 'pi pi-user-plus'"
            iconPos="right"
            :loading="loading"
            :disabled="!$form.valid"
          />
          <Button
            v-if="isEdit"
            icon="pi pi-trash"
            severity="danger"
            outlined
            :loading="deleteLoading"
            aria-label="Delete member"
            @click="confirmDelete"
          />
        </div>
      </div>

      <!-- Two-column grid -->
      <div class="grid grid-cols-1 lg:grid-cols-3 gap-6 items-start">
        <!-- Left Side-->
        <Card class="lg:col-span-2 border border-surface-200 dark:border-surface-700 shadow-sm">
          <template #content>
            <div class="p-2 space-y-6">
              <div class="space-y-4">
                <h2
                  class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
                >
                  Personal Details
                </h2>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <FormField v-slot="$field" name="name" class="flex flex-col gap-1.5">
                    <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                      Name <span class="text-red-500">*</span>
                    </label>
                    <InputText
                      v-model="model.name"
                      placeholder="Enter name"
                      :invalid="$field?.invalid"
                      fluid
                    />
                    <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                      {{ $field.error?.message }}
                    </Message>
                  </FormField>

                  <FormField v-slot="$field" name="surname" class="flex flex-col gap-1.5">
                    <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                      Surname <span class="text-red-500">*</span>
                    </label>
                    <InputText
                      v-model="model.surname"
                      placeholder="Enter surname"
                      :invalid="$field?.invalid"
                      fluid
                    />
                    <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                      {{ $field.error?.message }}
                    </Message>
                  </FormField>
                </div>

                <FormField v-slot="$field" name="secondSurname" class="flex flex-col gap-1.5">
                  <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                    Second Surname
                  </label>
                  <InputText
                    v-model="model.secondSurname"
                    placeholder="Enter second surname"
                    :invalid="$field?.invalid"
                    fluid
                  />
                  <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                    {{ $field.error?.message }}
                  </Message>
                </FormField>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <FormField v-slot="$field" name="birthdate" class="flex flex-col gap-1.5">
                    <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                      Birthdate
                    </label>
                    <InputText
                      v-model="model.birthdate"
                      type="date"
                      :invalid="$field?.invalid"
                      fluid
                    />
                    <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                      {{ $field.error?.message }}
                    </Message>
                  </FormField>

                  <FormField v-slot="$field" name="genderId" class="flex flex-col gap-1.5">
                    <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                      Gender
                    </label>
                    <GenderSelect v-model="model.genderId" :invalid="$field?.invalid" />
                    <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                      {{ $field.error?.message }}
                    </Message>
                  </FormField>
                </div>
              </div>

              <div class="border-t border-surface-100 dark:border-surface-800" />

              <div class="space-y-4">
                <h2
                  class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
                >
                  Contact
                </h2>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <FormField v-slot="$field" name="email" class="flex flex-col gap-1.5">
                    <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                      Email <span class="text-red-500">*</span>
                    </label>
                    <InputText
                      v-model="model.email"
                      type="email"
                      placeholder="Enter email"
                      autocomplete="email"
                      :invalid="$field?.invalid"
                      fluid
                    />
                    <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                      {{ $field.error?.message }}
                    </Message>
                  </FormField>

                  <FormField v-slot="$field" name="phone" class="flex flex-col gap-1.5">
                    <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                      Phone
                    </label>
                    <InputText
                      v-model="model.phone"
                      type="tel"
                      placeholder="Enter phone"
                      :invalid="$field?.invalid"
                      fluid
                    />
                    <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                      {{ $field.error?.message }}
                    </Message>
                  </FormField>
                </div>

                <FormField v-slot="$field" name="countryId" class="flex flex-col gap-1.5">
                  <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                    Country
                  </label>
                  <CountrySelect v-model="model.countryId" :invalid="$field?.invalid" />
                  <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                    {{ $field.error?.message }}
                  </Message>
                </FormField>
              </div>

              <div class="border-t border-surface-100 dark:border-surface-800" />

              <div class="space-y-4">
                <h2
                  class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
                >
                  Notes
                </h2>

                <FormField v-slot="$field" name="notes" class="flex flex-col gap-1.5">
                  <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                    Notes
                  </label>
                  <Textarea
                    v-model="model.notes"
                    placeholder="Any additional notes"
                    :invalid="$field?.invalid"
                    rows="4"
                    fluid
                  />
                  <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                    {{ $field.error?.message }}
                  </Message>
                </FormField>
              </div>
            </div>
          </template>
        </Card>

        <!-- Right Side -->
        <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
          <template #content>
            <div class="p-2 space-y-4">
              <h2
                class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
              >
                Interests
              </h2>
              <div class="flex flex-col gap-1.5">
                <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                  Interests
                </label>
                <InterestsSelect v-model="model.interestIds" />
                <p class="text-xs text-surface-400 dark:text-surface-500">
                  Select one or more interests for this member
                </p>
              </div>
            </div>
          </template>
        </Card>
      </div>
    </Form>
  </div>
</template>
