<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import Card from 'primevue/card'
import Button from 'primevue/button'
import { useToast } from 'primevue/usetoast'
import { emailsService } from '@/services/emails.service'
import type { EmailProviderStatusResponse } from '@/types/email.types'

const toast = useToast()

const status = ref<EmailProviderStatusResponse | null>(null)
const loading = ref(false)

const usagePercent = computed(() => {
  if (!status.value) return 0
  return Math.round((status.value.sentToday / status.value.dailyLimit) * 100)
})

const usageColor = computed(() => {
  if (usagePercent.value >= 90) return 'bg-red-500'
  if (usagePercent.value >= 70) return 'bg-amber-500'
  return 'bg-green-500'
})

const usageTextColor = computed(() => {
  if (usagePercent.value >= 90) return 'text-red-600 dark:text-red-400'
  if (usagePercent.value >= 70) return 'text-amber-600 dark:text-amber-400'
  return 'text-green-600 dark:text-green-400'
})

async function fetchStatus() {
  loading.value = true
  try {
    status.value = await emailsService.getProviderStatus()
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: 'Failed to load email provider status.',
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

onMounted(fetchStatus)
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm max-w-sm">
    <template #content>
      <div class="p-2 space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <i class="pi pi-envelope text-primary-500 dark:text-primary-400" />
            <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0">
              Email Quota
            </h2>
          </div>
          <Button
            icon="pi pi-refresh"
            severity="secondary"
            text
            rounded
            size="small"
            aria-label="Refresh status"
            :loading="loading"
            @click="fetchStatus"
          />
        </div>

        <div v-if="loading && !status" class="flex items-center justify-center py-6">
          <i class="pi pi-spinner pi-spin text-2xl text-surface-400" />
        </div>

        <template v-else-if="status">
          <div class="flex items-center gap-1.5">
            <span class="text-xs text-surface-400">Provider</span>
            <span
              class="inline-flex items-center px-2 py-0.5 rounded-full bg-surface-100 dark:bg-surface-800 text-xs font-medium text-surface-700 dark:text-surface-300 capitalize"
            >
              {{ status.provider }}
            </span>
          </div>

          <div class="space-y-1.5">
            <div class="flex items-center justify-between text-xs">
              <span class="text-surface-500 dark:text-surface-400">
                {{ status.sentToday }} / {{ status.dailyLimit }} sent today
              </span>
              <span :class="['font-semibold', usageTextColor]">{{ usagePercent }}%</span>
            </div>
            <div class="w-full h-2 rounded-full bg-surface-100 dark:bg-surface-800 overflow-hidden">
              <div
                :class="['h-full rounded-full transition-all duration-500', usageColor]"
                :style="{ width: `${usagePercent}%` }"
              ></div>
            </div>
          </div>

          <div
            class="flex items-center gap-3 rounded-lg px-3 py-2.5"
            :class="
              status.remaining === 0
                ? 'bg-red-50 dark:bg-red-950'
                : 'bg-surface-50 dark:bg-surface-800'
            "
          >
            <i
              class="pi text-lg"
              :class="
                status.remaining === 0
                  ? 'pi-times-circle text-red-500'
                  : 'pi-check-circle text-green-500'
              "
            ></i>
            <div>
              <p class="text-sm font-semibold text-surface-900 dark:text-surface-0">
                <template v-if="status.remaining === 0">No emails left today</template>
                <template v-else>
                  <span :class="usageTextColor">{{ status.remaining }}</span>
                  email{{ status.remaining === 1 ? '' : 's' }} remaining today
                </template>
              </p>
              <p class="text-xs text-surface-400 mt-0.5">Resets at midnight</p>
            </div>
          </div>
        </template>
      </div>
    </template>
  </Card>
</template>
