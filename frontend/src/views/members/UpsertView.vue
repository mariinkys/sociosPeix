<!-- views/MemberUpsertView.vue -->
<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Form, FormField } from '@primevue/forms'
import { useI18n } from 'vue-i18n'
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
import MemberEmailsCard from '@/components/member/EmailsCard.vue'

const { t } = useI18n({ useScope: 'global' })
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
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('members.messages.loadError'),
      life: 3000,
    })
    router.push('/members')
  } finally {
    fetchLoading.value = false
  }
}

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  if (!values.name) {
    errors.name = [{ message: t('members.fields.name.required') }]
  } else if (String(values.name).length > 100) {
    errors.name = [{ message: t('members.fields.name.max') }]
  }

  if (!values.surname) {
    errors.surname = [{ message: t('members.fields.surname.required') }]
  } else if (String(values.surname).length > 100) {
    errors.surname = [{ message: t('members.fields.surname.max') }]
  }

  if (values.secondSurname && String(values.secondSurname).length > 100) {
    errors.secondSurname = [{ message: t('members.fields.secondSurname.max') }]
  }

  if (!values.email) {
    errors.email = [{ message: t('members.fields.email.required') }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(String(values.email))) {
    errors.email = [{ message: t('members.fields.email.invalid') }]
  }

  if (values.phone && String(values.phone).length > 30) {
    errors.phone = [{ message: t('members.fields.phone.max') }]
  }

  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return

  loading.value = true
  try {
    if (isEdit.value) {
      const response = await membersService.update(
        memberId.value!,
        model.value as MemberUpdatePayload,
      )
      toast.add({
        severity: 'success',
        summary: t('common.feedback.saved'),
        detail: t('members.messages.updated'),
        life: 3000,
      })
      router.push('/members/' + response.id + '/edit')
    } else {
      const response = await membersService.create(model.value)
      toast.add({
        severity: 'success',
        summary: t('common.feedback.created'),
        detail: t('members.messages.created'),
        life: 3000,
      })
      router.push('/members/' + response.id + '/edit')
    }
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: isEdit.value ? t('members.messages.updateError') : t('members.messages.createError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function confirmDelete() {
  confirm.require({
    message: t('members.deleteDialog.messageGeneric'),
    header: t('members.deleteDialog.title'),
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
        await membersService.delete(memberId.value!)
        toast.add({
          severity: 'success',
          summary: t('common.feedback.deleted'),
          detail: t('members.deleteDialog.deletedSuccess'),
          life: 3000,
        })
        router.push('/members')
      } catch {
        toast.add({
          severity: 'error',
          summary: t('common.feedback.error'),
          detail: t('members.deleteDialog.error'),
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
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
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
            :aria-label="t('common.actions.back')"
          />
          <div>
            <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
              {{ isEdit ? t('members.titles.edit') : t('members.titles.create') }}
            </h1>
            <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
              {{ isEdit ? t('members.descriptions.edit') : t('members.descriptions.create') }}
            </p>
          </div>
        </div>

        <div class="flex items-center gap-2 shrink-0">
          <Button
            :label="t('common.actions.cancel')"
            severity="secondary"
            outlined
            @click="router.push('/members')"
          />
          <Button
            type="submit"
            :label="isEdit ? t('common.actions.saveChanges') : t('members.actions.create')"
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
            :aria-label="t('members.actions.delete')"
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
                  {{ t('members.sections.personalDetails') }}
                </h2>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <FormField v-slot="$field" name="name" class="flex flex-col gap-1.5">
                    <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                      {{ t('members.fields.name.label') }} <span class="text-red-500">*</span>
                    </label>
                    <InputText
                      v-model="model.name"
                      :placeholder="t('members.fields.name.placeholder')"
                      :invalid="$field?.invalid"
                      fluid
                    />
                    <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                      {{ $field.error?.message }}
                    </Message>
                  </FormField>

                  <FormField v-slot="$field" name="surname" class="flex flex-col gap-1.5">
                    <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                      {{ t('members.fields.surname.label') }} <span class="text-red-500">*</span>
                    </label>
                    <InputText
                      v-model="model.surname"
                      :placeholder="t('members.fields.surname.placeholder')"
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
                    {{ t('members.fields.secondSurname.label') }}
                  </label>
                  <InputText
                    v-model="model.secondSurname"
                    :placeholder="t('members.fields.secondSurname.placeholder')"
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
                      {{ t('members.fields.birthdate.label') }}
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
                      {{ t('members.fields.gender.label') }}
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
                  {{ t('members.sections.contact') }}
                </h2>

                <div class="grid grid-cols-1 sm:grid-cols-2 gap-4">
                  <FormField v-slot="$field" name="email" class="flex flex-col gap-1.5">
                    <div class="flex items-center justify-between w-full">
                      <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                        {{ t('members.fields.email.label') }} <span class="text-red-500">*</span>
                      </label>
                      <i
                        v-if="isEdit"
                        v-tooltip.top="t('members.fields.email.editWarning')"
                        class="pi pi-exclamation-triangle text-amber-500 text-xs cursor-default"
                      ></i>
                    </div>
                    <InputText
                      v-model="model.email"
                      type="email"
                      :placeholder="t('members.fields.email.placeholder')"
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
                      {{ t('members.fields.phone.label') }}
                    </label>
                    <InputText
                      v-model="model.phone"
                      type="tel"
                      :placeholder="t('members.fields.phone.placeholder')"
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
                    {{ t('members.fields.country.label') }}
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
                  {{ t('members.sections.notes') }}
                </h2>

                <FormField v-slot="$field" name="notes" class="flex flex-col gap-1.5">
                  <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                    {{ t('members.fields.notes.label') }}
                  </label>
                  <Textarea
                    v-model="model.notes"
                    :placeholder="t('members.fields.notes.placeholder')"
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
        <div class="lg:col-span-1 space-y-6">
          <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
            <template #content>
              <div class="p-2 space-y-4">
                <h2
                  class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
                >
                  {{ t('members.sections.interests') }}
                </h2>
                <div class="flex flex-col gap-1.5">
                  <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                    {{ t('members.fields.interests.label') }}
                  </label>
                  <InterestsSelect v-model="model.interestIds" />
                  <p class="text-xs text-surface-400 dark:text-surface-500">
                    {{ t('members.fields.interests.help') }}
                  </p>
                </div>
              </div>
            </template>
          </Card>

          <MemberEmailsCard v-if="isEdit && memberId" :memberId="memberId" />
        </div>
      </div>
    </Form>
  </div>
</template>
