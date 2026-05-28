<script setup lang="ts">
import Card from 'primevue/card'
import InputText from 'primevue/inputtext'
import Password from 'primevue/password'
import Button from 'primevue/button'
import { ref } from 'vue'
import { useRouter, useRoute } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import type { LoginPayload } from '@/stores/auth'
import type { AxiosError } from 'axios'
import { useToast } from 'primevue/usetoast'

const auth = useAuthStore()
const router = useRouter()
const route = useRoute()
const toast = useToast()

const model = ref<LoginPayload>({
  email: '',
  password: '',
})

const loading = ref(false)

const login = async () => {
  loading.value = true

  try {
    await auth.login(model.value)
    const redirect = (route.query.redirect as string) ?? '/'
    router.push(redirect)
  } catch (e) {
    const err = e as AxiosError<{ message: string }>
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: err.response?.data?.message ?? 'Login failed',
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
            <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">Sign in</h1>
            <p class="text-sm text-surface-500 dark:text-surface-400">
              Welcome back — please enter your details
            </p>
          </div>

          <form class="space-y-5" @submit.prevent="login">
            <div class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                Email
              </label>
              <InputText
                v-model="model.email"
                type="email"
                placeholder="Enter email"
                autocomplete="email"
                fluid
              />
            </div>

            <div class="flex flex-col gap-1.5">
              <label class="text-sm font-medium text-surface-700 dark:text-surface-300">
                Password
              </label>
              <Password
                v-model="model.password"
                placeholder="Enter password"
                :feedback="false"
                toggleMask
                fluid
              />
            </div>

            <Button
              type="submit"
              label="Sign in"
              icon="pi pi-sign-in"
              iconPos="right"
              class="w-full"
              :loading="loading"
              :disabled="!model.email || !model.password"
            />
          </form>
        </div>
      </template>
    </Card>
  </div>
</template>
