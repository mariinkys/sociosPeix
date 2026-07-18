<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Form, FormField } from '@primevue/forms'
import type { FormResolverOptions, FormSubmitEvent } from '@primevue/forms'
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Button from 'primevue/button'
import Message from 'primevue/message'
import { computed, ref } from 'vue'
import { useRouter } from 'vue-router'
import { passwordResetService } from '@/services/password-reset.service'
import type { ForgotPasswordPayload } from '@/types/auth.types'
import { useToast } from 'primevue/usetoast'
import { usePasswordResetCooldown } from '@/composables/usePasswordResetCooldown'

const { t } = useI18n({ useScope: 'global' })
const router = useRouter()
const toast = useToast()
const loading = ref(false)

const model = ref<ForgotPasswordPayload>({ email: '' })
const initialValues = { email: '' }

const emailRef = computed(() => model.value.email)
const cooldown = usePasswordResetCooldown(emailRef)

const resolver = ({ values }: FormResolverOptions) => {
  const errors: Record<string, { message: string }[]> = {}
  const email = values.email as string

  if (!email) {
    errors.email = [{ message: t('common.validation.emailRequired') }]
  } else if (!/^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(email)) {
    errors.email = [{ message: t('common.validation.invalidEmail') }]
  }

  return { errors }
}

async function onSubmit({ valid }: FormSubmitEvent) {
  if (!valid) return

  loading.value = true
  try {
    await passwordResetService.requestReset(model.value)
    cooldown.startCooldown()
    toast.add({
      severity: 'success',
      summary: t('common.feedback.success'),
      detail: t('auth.forgotPassword.successMessage'),
      life: 4000,
    })
    router.push({ name: 'reset-password', query: { email: model.value.email } })
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('auth.forgotPassword.errors.requestFailed'),
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
              {{ t('auth.forgotPassword.title') }}
            </h1>
            <p class="text-sm text-surface-500 dark:text-surface-400">
              {{ t('auth.forgotPassword.description') }}
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

            <Button
              type="submit"
              :label="
                cooldown.isInCooldown.value
                  ? t('auth.forgotPassword.waitLabel', { time: cooldown.formattedRemaining.value })
                  : t('auth.forgotPassword.submit')
              "
              icon="pi pi-send"
              iconPos="right"
              class="w-full"
              :loading="loading"
              :disabled="!!$form.invalid || cooldown.isInCooldown.value"
            />
          </Form>

          <div class="text-center">
            <RouterLink :to="{ name: 'Login' }" class="text-sm text-primary-500 hover:underline">
              {{ t('auth.forgotPassword.backToLogin') }}
            </RouterLink>
          </div>
        </div>
      </template>
    </Card>
  </div>
</template>
