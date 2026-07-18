<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import Card from 'primevue/card'
import Button from 'primevue/button'
import Select from 'primevue/select'
import { useToast } from 'primevue/usetoast'
import { useConfirm } from 'primevue/useconfirm'
import ConfirmDialog from 'primevue/confirmdialog'
import { storeToRefs } from 'pinia'
import { useAuthStore } from '@/stores/auth'
import { emailsService } from '@/services/emails.service'
import type { EmailProviderInfo, EmailProviderStatusResponse } from '@/types/email.types'

defineOptions({ inheritAttrs: false })

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()
const confirm = useConfirm()
const { isAdmin } = storeToRefs(useAuthStore())

const status = ref<EmailProviderStatusResponse | null>(null)
const loading = ref(false)

const providers = ref<EmailProviderInfo[]>([])
const providersLoading = ref(false)
const switching = ref(false)
// mirrors status.provider for the Select's v-model; kept separate so a
// declined confirmation can revert the dropdown without touching `status`
const selectedProvider = ref<string | null>(null)

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
    selectedProvider.value = status.value.provider
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('email.quotaCard.errors.load'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

async function fetchProviders() {
  providersLoading.value = true
  try {
    providers.value = await emailsService.listProviders()
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('email.quotaCard.errors.loadProviders'),
      life: 3000,
    })
  } finally {
    providersLoading.value = false
  }
}

function onProviderPicked(newProvider: string) {
  const current = status.value?.provider ?? null
  if (!newProvider || newProvider === current) return

  confirm.require({
    header: t('email.quotaCard.switchConfirm.header'),
    message: t('email.quotaCard.switchConfirm.message', { provider: newProvider }),
    icon: 'pi pi-exclamation-triangle',
    acceptLabel: t('common.actions.apply'),
    rejectLabel: t('common.actions.cancel'),
    acceptProps: { severity: 'warn' },
    accept: () => applyProviderChange(newProvider),
    reject: () => {
      selectedProvider.value = current // revert the dropdown
    },
  })
}

async function applyProviderChange(newProvider: string) {
  switching.value = true
  try {
    await emailsService.setActiveProvider(newProvider)
    toast.add({
      severity: 'success',
      summary: t('common.feedback.success'),
      detail: t('email.quotaCard.switchSuccess', { provider: newProvider }),
      life: 3000,
    })
    await fetchStatus() // refresh quota numbers for the newly active provider
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('email.quotaCard.errors.switchFailed'),
      life: 3000,
    })
    selectedProvider.value = status.value?.provider ?? null // revert on failure too
  } finally {
    switching.value = false
  }
}

onMounted(async () => {
  await fetchStatus()
  if (isAdmin.value) {
    await fetchProviders()
  }
})
</script>

<template>
  <ConfirmDialog />

  <Card
    v-bind="$attrs"
    class="border border-surface-200 dark:border-surface-700 shadow-sm max-w-sm h-full"
  >
    <template #content>
      <div class="p-2 space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <i class="pi pi-envelope text-primary-500 dark:text-primary-400"></i>
            <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0">
              {{ t('email.titles.quota') }}
            </h2>
          </div>
          <Button
            icon="pi pi-refresh"
            severity="secondary"
            text
            rounded
            size="small"
            :aria-label="t('email.quotaCard.refresh')"
            :loading="loading"
            @click="fetchStatus"
          />
        </div>

        <div v-if="loading && !status" class="flex items-center justify-center py-6">
          <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
        </div>

        <template v-else-if="status">
          <div class="flex items-center gap-1.5">
            <span class="text-xs text-surface-400">{{ t('common.fields.provider') }}</span>

            <!-- Admins -->
            <Select
              v-if="isAdmin"
              v-model="selectedProvider"
              :options="providers"
              optionLabel="name"
              optionValue="name"
              :loading="providersLoading || switching"
              :disabled="switching"
              size="small"
              class="text-xs w-full"
              @update:modelValue="onProviderPicked"
            />

            <!-- Non-admins -->
            <span
              v-else
              class="inline-flex items-center px-2 py-0.5 rounded-full bg-surface-100 dark:bg-surface-800 text-xs font-medium text-surface-700 dark:text-surface-300 capitalize"
            >
              {{ status.provider }}
            </span>
          </div>

          <div class="space-y-1.5">
            <div class="flex items-center justify-between text-xs">
              <span class="text-surface-500 dark:text-surface-400">
                {{
                  t('email.quotaCard.sentToday', {
                    sent: status.sentToday,
                    limit: status.dailyLimit,
                  })
                }}
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
                <template v-if="status.remaining === 0">{{
                  t('email.quotaCard.noRemaining')
                }}</template>
                <template v-else>
                  <span :class="usageTextColor">{{ status.remaining }}</span>
                  {{ t('email.quotaCard.remaining') }}
                </template>
              </p>
              <p class="text-xs text-surface-400 mt-0.5">
                {{ t('email.quotaCard.resetsAt') }}
              </p>
            </div>
          </div>
        </template>
      </div>
    </template>
  </Card>
</template>
