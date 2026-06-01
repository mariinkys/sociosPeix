<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Form, FormField } from '@primevue/forms'
import { useI18n } from 'vue-i18n'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import Message from 'primevue/message'
import ConfirmDialog from 'primevue/confirmdialog'
import Select from 'primevue/select'
import { useConfirm } from 'primevue/useconfirm'
import { useToast } from 'primevue/usetoast'
import { usersService } from '@/services/users.service'
import { useAuthStore } from '@/stores/auth'
import type { RegisterPayload } from '@/types/auth.types'
import type { UserUpdatePayload, UserRole, UpdatePasswordPayload } from '@/types/user.types'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()
const toast = useToast()
const confirm = useConfirm()
const authStore = useAuthStore()

const userId = computed(() => route.params.id as string | undefined)
const isEdit = computed(() => !!userId.value)
const isSelf = computed(() => authStore.user?.id === userId.value)
const loading = ref(false)
const fetchLoading = ref(!!route.params.id)
const deleteLoading = ref(false)
const roleLoading = ref(false)
const passwordLoading = ref(false)
const currentRole = ref<UserRole>('USER')
const selectedRole = ref<UserRole>('USER')

const roleOptions = computed(() => [
  { label: t('users.roles.USER'), value: 'USER' as UserRole },
  { label: t('users.roles.ADMIN'), value: 'ADMIN' as UserRole },
])

const roleChanged = computed(() => selectedRole.value !== currentRole.value)

// Main form
const model = ref<RegisterPayload>({
  name: '',
  email: '',
  password: '',
})

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  if (!values.name) {
    errors.name = [{ message: t('users.fields.name.required') }]
  } else if (String(values.name).length > 100) {
    errors.name = [{ message: t('users.fields.name.max') }]
  }

  if (!values.email) {
    errors.email = [{ message: t('users.fields.email.required') }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(String(values.email))) {
    errors.email = [{ message: t('users.fields.email.invalid') }]
  }

  if (!isEdit.value && !values.password) {
    errors.password = [{ message: t('users.fields.password.required') }]
  } else if (values.password && String(values.password).length < 8) {
    errors.password = [{ message: t('users.fields.password.min') }]
  }

  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return
  loading.value = true
  try {
    if (isEdit.value) {
      const payload: UserUpdatePayload = { name: model.value.name, email: model.value.email }
      await usersService.update(userId.value!, payload)
      toast.add({
        severity: 'success',
        summary: t('common.feedback.saved'),
        detail: t('users.messages.updated'),
        life: 3000,
      })
    } else {
      await usersService.create(model.value)
      toast.add({
        severity: 'success',
        summary: t('common.feedback.created'),
        detail: t('users.messages.created'),
        life: 3000,
      })
    }
    router.push('/users')
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: isEdit.value ? t('users.messages.updateError') : t('users.messages.createError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

// Password card
const passwordModel = ref({
  currentPassword: '',
  newPassword: '',
  confirmPassword: '',
})
const passwordError = ref('')

const passwordResolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}

  if (!authStore.isAdmin && !values.currentPassword) {
    errors.currentPassword = [{ message: t('users.passwordCard.fields.currentPassword.required') }]
  }

  if (!values.newPassword) {
    errors.newPassword = [{ message: t('users.passwordCard.fields.newPassword.required') }]
  } else if (String(values.newPassword).length < 8) {
    errors.newPassword = [{ message: t('users.passwordCard.fields.newPassword.min') }]
  }

  if (!values.confirmPassword) {
    errors.confirmPassword = [{ message: t('users.passwordCard.fields.confirmPassword.required') }]
  } else if (values.newPassword !== values.confirmPassword) {
    errors.confirmPassword = [{ message: t('users.passwordCard.fields.confirmPassword.mismatch') }]
  }

  return { errors }
}

async function onPasswordSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return
  passwordLoading.value = true
  passwordError.value = ''
  try {
    const payload: UpdatePasswordPayload = {
      currentPassword: authStore.isAdmin ? null : passwordModel.value.currentPassword,
      newPassword: passwordModel.value.newPassword,
    }
    await usersService.updatePassword(userId.value!, payload)
    toast.add({
      severity: 'success',
      summary: t('users.passwordCard.successTitle'),
      detail: t('users.passwordCard.successDetail'),
      life: 3000,
    })
    passwordModel.value = { currentPassword: '', newPassword: '', confirmPassword: '' }
  } catch {
    passwordError.value = t('users.passwordCard.error')
  } finally {
    passwordLoading.value = false
  }
}

// Role card
async function onRoleChange() {
  roleLoading.value = true
  try {
    await usersService.updateRole(userId.value!, selectedRole.value)
    currentRole.value = selectedRole.value
    toast.add({
      severity: 'success',
      summary: t('users.roleCard.updatedTitle'),
      detail: t('users.roleCard.updatedDetail', {
        role: t(`users.roles.${selectedRole.value}`),
      }),
      life: 3000,
    })
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('users.roleCard.updateError'),
      life: 3000,
    })
    selectedRole.value = currentRole.value
  } finally {
    roleLoading.value = false
  }
}

// Delete
function confirmDelete() {
  confirm.require({
    message: t('users.deleteDialog.message'),
    header: t('users.deleteDialog.title'),
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
        await usersService.delete(userId.value!)
        toast.add({
          severity: 'success',
          summary: t('common.feedback.deleted'),
          detail: t('users.deleteDialog.success'),
          life: 3000,
        })
        router.push('/users')
      } catch {
        toast.add({
          severity: 'error',
          summary: t('common.feedback.error'),
          detail: t('users.deleteDialog.error'),
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
    const user = await usersService.getById(userId.value!)
    model.value = { name: user.name, email: user.email, password: '' }
    currentRole.value = user.role
    selectedRole.value = user.role
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('users.messages.loadError'),
      life: 3000,
    })
    router.push('/users')
  } finally {
    fetchLoading.value = false
  }
})
</script>

<template>
  <div class="p-6 max-w-2xl mx-auto space-y-6">
    <ConfirmDialog />

    <div class="flex items-center justify-between gap-3 flex-wrap">
      <div class="flex items-center gap-3">
        <Button
          icon="pi pi-arrow-left"
          severity="secondary"
          text
          rounded
          :aria-label="t('common.actions.back')"
          @click="router.push('/users')"
        />
        <div>
          <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
            {{ isEdit ? t('users.titles.edit') : t('users.titles.create') }}
          </h1>
          <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
            {{ isEdit ? t('users.descriptions.edit') : t('users.descriptions.create') }}
          </p>
        </div>
      </div>

      <div class="flex items-center gap-2 shrink-0">
        <!-- Admins can delete anyone except themselves, regular users cannot delete -->
        <Button
          v-if="isEdit && authStore.isAdmin && !isSelf"
          icon="pi pi-trash"
          severity="danger"
          outlined
          :loading="deleteLoading"
          :aria-label="t('users.actions.delete')"
          @click="confirmDelete"
        />
      </div>
    </div>

    <div v-if="fetchLoading" class="flex items-center justify-center py-24">
      <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
    </div>

    <template v-else>
      <!-- Main form -->
      <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
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
                {{ t('users.fields.name.label') }} <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="model.name"
                :placeholder="t('users.fields.name.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="email" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                Email <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="model.email"
                type="email"
                :placeholder="t('users.fields.email.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <!-- Password only on create -->
            <FormField v-if="!isEdit" v-slot="$field" name="password" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('users.fields.password.label') }} <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="model.password"
                type="password"
                :placeholder="t('users.fields.password.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <div class="flex items-center justify-end gap-3 pt-2">
              <Button
                :label="t('common.actions.cancel')"
                severity="secondary"
                outlined
                @click="router.push('/users')"
              />
              <Button
                type="submit"
                :label="isEdit ? t('common.actions.saveChanges') : t('users.actions.create')"
                :icon="isEdit ? 'pi pi-check' : 'pi pi-plus'"
                iconPos="right"
                :loading="loading"
                :disabled="!$form.valid"
              />
            </div>
          </Form>
        </template>
      </Card>

      <!-- Password card, edit mode only -->
      <Card v-if="isEdit" class="border border-surface-200 dark:border-surface-700 shadow-sm">
        <template #content>
          <Form
            :initialValues="passwordModel"
            :resolver="passwordResolver"
            :validateOnBlur="true"
            :validateOnValueUpdate="true"
            class="p-2 space-y-4"
            @submit="onPasswordSubmit"
          >
            <div class="flex items-center gap-2">
              <i class="pi pi-lock text-primary-500 dark:text-primary-400"></i>
              <h2
                class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
              >
                {{ t('users.passwordCard.title') }}
              </h2>
            </div>

            <!-- Current password, only required for non-admins -->
            <FormField
              v-if="!authStore.isAdmin"
              v-slot="$field"
              name="currentPassword"
              class="flex flex-col gap-1.5"
            >
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('users.passwordCard.fields.currentPassword.label') }}
                <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.currentPassword"
                type="password"
                :placeholder="t('users.passwordCard.fields.currentPassword.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="newPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('users.passwordCard.fields.newPassword.label') }} <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.newPassword"
                type="password"
                :placeholder="t('users.passwordCard.fields.newPassword.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="confirmPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('users.passwordCard.fields.confirmPassword.label') }}
                <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.confirmPassword"
                type="password"
                :placeholder="t('users.passwordCard.fields.confirmPassword.placeholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <Message v-if="passwordError" severity="error" size="small" variant="simple">
              {{ passwordError }}
            </Message>

            <div class="flex justify-end pt-1">
              <Button
                type="submit"
                :label="t('users.passwordCard.submit')"
                icon="pi pi-lock"
                iconPos="right"
                :loading="passwordLoading"
              />
            </div>
          </Form>
        </template>
      </Card>

      <!-- Role card: edit mode, admin only (and not for self) -->
      <Card
        v-if="isEdit && authStore.isAdmin && !isSelf"
        class="border border-surface-200 dark:border-surface-700 shadow-sm"
      >
        <template #content>
          <div class="p-2 space-y-4">
            <div class="flex items-center gap-2">
              <i class="pi pi-shield text-primary-500 dark:text-primary-400"></i>
              <h2
                class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
              >
                {{ t('users.roleCard.title') }}
              </h2>
            </div>

            <p class="text-sm text-surface-500 dark:text-surface-400">
              {{ t('users.roleCard.description') }}
            </p>

            <div class="flex items-center gap-3">
              <Select
                v-model="selectedRole"
                :options="roleOptions"
                optionLabel="label"
                optionValue="value"
                class="flex-1"
              />
              <Button
                :label="t('common.actions.apply')"
                icon="pi pi-check"
                :loading="roleLoading"
                :disabled="!roleChanged"
                @click="onRoleChange"
              />
            </div>
          </div>
        </template>
      </Card>
    </template>
  </div>
</template>
