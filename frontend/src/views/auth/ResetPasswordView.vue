<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Form, FormField } from '@primevue/forms'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Password from 'primevue/password'
import Button from 'primevue/button'
import Message from 'primevue/message'
import { computed, ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { passwordResetService } from '@/services/password-reset.service'
import type { ResetPasswordPayload } from '@/types/auth.types'
import type { AxiosError } from 'axios'
import { useToast } from 'primevue/usetoast'
import { usePasswordResetCooldown } from '@/composables/usePasswordResetCooldown'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const route = useRoute()
const toast = useToast()
const loading = ref(false)

const model = ref<ResetPasswordPayload>({
  email: (route.query.email as string) ?? '',
  code: '',
  newPassword: '',
})

const initialValues = { ...model.value }

const emailRef = computed(() => model.value.email)
const cooldown = usePasswordResetCooldown(emailRef)
const resending = ref(false)

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}
  const email = values.email as string
  const code = values.code as string
  const newPassword = values.newPassword as string

  if (!email) {
    errors.email = [{ message: t('common.validation.emailRequired') }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    errors.email = [{ message: t('common.validation.invalidEmail') }]
  }

  if (!code) {
    errors.code = [{ message: t('common.validation.codeRequired') }]
  } else if (!/^\d{6}$/.test(code)) {
    errors.code = [{ message: t('common.validation.invalidCode') }]
  }

  if (!newPassword) {
    errors.newPassword = [{ message: t('common.validation.passwordRequired') }]
  } else if (newPassword.length < 8) {
    errors.newPassword = [{ message: t('common.validation.min8') }]
  }

  return { errors }
}

async function onResend() {
  if (cooldown.isInCooldown.value || !model.value.email) return

  resending.value = true
  try {
    await passwordResetService.requestReset({ email: model.value.email })
    cooldown.startCooldown()
    toast.add({
      severity: 'success',
      summary: t('common.feedback.success'),
      detail: t('auth.forgotPassword.successMessage'),
      life: 4000,
    })
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('auth.forgotPassword.errors.requestFailed'),
      life: 3000,
    })
  } finally {
    resending.value = false
  }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return

  loading.value = true
  try {
    await passwordResetService.resetPassword(model.value)
    toast.add({
      severity: 'success',
      summary: t('common.feedback.success'),
      detail: t('auth.resetPassword.successMessage'),
      life: 4000,
    })
    router.push({ name: 'Login' })
  } catch (e) {
    const err = e as AxiosError<{ message: string }>
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: err.response?.data?.message ?? t('auth.resetPassword.errors.resetFailed'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div
    class="flex h-full items-center justify-center bg-surface-50 dark:bg-surface-950 px-6 transition-colors duration-200"
  >
    <Card class="w-full max-w-md border border-surface-200 dark:border-surface-700 shadow-lg">
      <template #content>
        <div class="space-y-8 p-2">
          <div class="text-center space-y-1">
            <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
              {{ t('auth.resetPassword.title') }}
            </h1>
            <p class="text-sm text-surface-500 dark:text-surface-400">
              {{ t('auth.resetPassword.description') }}
            </p>
          </div>

          <Form
            v-slot="$form"
            :initialValues
            :resolver
            :validateOnBlur="true"
            :validateOnValueUpdate="true"
            class="space-y-5"
            @submit="onSubmit"
          >
            <FormField v-slot="$field" name="email" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('common.fields.email') }}
              </label>
              <InputText
                v-model="model.email"
                type="email"
                :placeholder="t('common.placeholders.enterYourEmail')"
                autocomplete="email"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="code" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('auth.resetPassword.codeLabel') }}
              </label>
              <InputText
                v-model="model.code"
                inputmode="numeric"
                maxlength="6"
                :placeholder="t('auth.resetPassword.codePlaceholder')"
                :invalid="$field?.invalid"
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <FormField v-slot="$field" name="newPassword" class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                {{ t('auth.resetPassword.newPasswordLabel') }}
              </label>
              <Password
                v-model="model.newPassword"
                :placeholder="t('common.placeholders.enterPassword')"
                :feedback="false"
                :invalid="$field?.invalid"
                toggleMask
                fluid
              />
              <Message v-if="$field?.invalid" severity="error" size="small" variant="simple">
                {{ $field.error?.message }}
              </Message>
            </FormField>

            <Button
              type="submit"
              :label="t('auth.resetPassword.submit')"
              icon="pi pi-lock"
              iconPos="right"
              class="w-full"
              :loading="loading"
              :disabled="!!$form.invalid"
            />
          </Form>

          <div class="text-center text-sm">
            <button
              v-if="!cooldown.isInCooldown.value"
              type="button"
              class="text-primary-500 hover:underline disabled:opacity-50"
              :disabled="resending || !model.email"
              @click="onResend"
            >
              {{ t('auth.resetPassword.resendCode') }}
            </button>
            <span v-else class="text-surface-500 dark:text-surface-400">
              {{ t('auth.resetPassword.resendIn', { time: cooldown.formattedRemaining.value }) }}
            </span>
          </div>
        </div>
      </template>
    </Card>
  </div>
</template>
