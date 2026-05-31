<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { Form, FormField } from '@primevue/forms'
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

const roleOptions: { label: string; value: UserRole }[] = [
  { label: 'User', value: 'USER' },
  { label: 'Admin', value: 'ADMIN' },
]

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
    errors.name = [{ message: 'Name is required' }]
  } else if (String(values.name).length > 100) {
    errors.name = [{ message: 'Name must not exceed 100 characters' }]
  }

  if (!values.email) {
    errors.email = [{ message: 'Email is required' }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(String(values.email))) {
    errors.email = [{ message: 'Email must be valid' }]
  }

  if (!isEdit.value && !values.password) {
    errors.password = [{ message: 'Password is required' }]
  } else if (values.password && String(values.password).length < 8) {
    errors.password = [{ message: 'Password must be at least 8 characters' }]
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
        summary: 'Saved',
        detail: 'User updated successfully',
        life: 3000,
      })
    } else {
      await usersService.create(model.value)
      toast.add({
        severity: 'success',
        summary: 'Created',
        detail: 'User created successfully',
        life: 3000,
      })
    }
    router.push('/users')
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: isEdit.value ? 'Failed to update user' : 'Failed to create user',
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
    errors.currentPassword = [{ message: 'Current password is required' }]
  }

  if (!values.newPassword) {
    errors.newPassword = [{ message: 'New password is required' }]
  } else if (String(values.newPassword).length < 8) {
    errors.newPassword = [{ message: 'Password must be at least 8 characters' }]
  }

  if (!values.confirmPassword) {
    errors.confirmPassword = [{ message: 'Please confirm the new password' }]
  } else if (values.newPassword !== values.confirmPassword) {
    errors.confirmPassword = [{ message: 'Passwords do not match' }]
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
      summary: 'Password updated',
      detail: 'Password changed successfully.',
      life: 3000,
    })
    passwordModel.value = { currentPassword: '', newPassword: '', confirmPassword: '' }
  } catch {
    passwordError.value = 'Failed to update password. Check your current password and try again.'
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
      summary: 'Role updated',
      detail: `Role changed to ${selectedRole.value}`,
      life: 3000,
    })
  } catch {
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to update role', life: 3000 })
    selectedRole.value = currentRole.value
  } finally {
    roleLoading.value = false
  }
}

// Delete
function confirmDelete() {
  confirm.require({
    message: 'Are you sure you want to delete this user? This action cannot be undone.',
    header: 'Delete User',
    icon: 'pi pi-exclamation-triangle',
    rejectProps: { label: 'Cancel', severity: 'secondary', outlined: true },
    acceptProps: { label: 'Delete', severity: 'danger' },
    accept: async () => {
      deleteLoading.value = true
      try {
        await usersService.delete(userId.value!)
        toast.add({
          severity: 'success',
          summary: 'Deleted',
          detail: 'User deleted successfully.',
          life: 3000,
        })
        router.push('/users')
      } catch {
        toast.add({
          severity: 'error',
          summary: 'Error',
          detail: 'Failed to delete user. Please try again.',
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
    toast.add({ severity: 'error', summary: 'Error', detail: 'Failed to load user', life: 3000 })
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
          aria-label="Go back"
          @click="router.push('/users')"
        />
        <div>
          <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
            {{ isEdit ? 'Edit User' : 'New User' }}
          </h1>
          <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
            {{
              isEdit
                ? "Update the user's details below"
                : 'Fill in the details to create a new user'
            }}
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
          aria-label="Delete user"
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

            <FormField v-slot="$field" name="email" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                Email <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="model.email"
                type="email"
                placeholder="Enter email"
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
                Password <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="model.password"
                type="password"
                placeholder="Enter password"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <div class="flex items-center justify-end gap-3 pt-2">
              <Button label="Cancel" severity="secondary" outlined @click="router.push('/users')" />
              <Button
                type="submit"
                :label="isEdit ? 'Save Changes' : 'Create User'"
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
                Change Password
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
                Current Password <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.currentPassword"
                type="password"
                placeholder="Enter current password"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="newPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                New Password <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.newPassword"
                type="password"
                placeholder="Enter new password"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="confirmPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                Confirm New Password <span class="text-red-500">*</span>
              </label>
              <InputText
                v-model="passwordModel.confirmPassword"
                type="password"
                placeholder="Repeat new password"
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
                label="Update Password"
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
              <i class="pi pi-shield text-primary-500 dark:text-primary-400" />
              <h2
                class="text-sm font-semibold text-surface-700 dark:text-surface-300 uppercase tracking-wide"
              >
                Role
              </h2>
            </div>

            <p class="text-sm text-surface-500 dark:text-surface-400">
              Changing this will immediately affect what the user can access.
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
                label="Apply"
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
