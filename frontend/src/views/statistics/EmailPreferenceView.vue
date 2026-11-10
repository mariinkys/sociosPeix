<script setup lang="ts">
import { ref, computed, onMounted, onBeforeUnmount, nextTick } from 'vue'
import { useI18n } from 'vue-i18n'
import Card from 'primevue/card'
import Button from 'primevue/button'
import { useToast } from 'primevue/usetoast'
import { Chart, DoughnutController, ArcElement, Tooltip } from 'chart.js'
import { statisticsService } from '@/services/statistics.service'
import type { EmailPreferenceStats } from '@/types/statistics.types'

Chart.register(DoughnutController, ArcElement, Tooltip)

const { t, locale } = useI18n({ useScope: 'global' })
const toast = useToast()

const ENABLED_COLOR = '#10b981'
const DISABLED_COLOR = '#f97316'

const stats = ref<EmailPreferenceStats | null>(null)
const loading = ref(false)

const canvasRef = ref<HTMLCanvasElement | null>(null)
let chart: Chart | null = null

const isEmpty = computed(() => !stats.value || stats.value.total === 0)

const percentFormat = computed(
  () => new Intl.NumberFormat(locale.value, { style: 'percent', maximumFractionDigits: 1 }),
)

function percent(count: number): string {
  const total = stats.value?.total ?? 0
  return total === 0 ? '—' : percentFormat.value.format(count / total)
}

const legend = computed(() => {
  if (!stats.value) return []
  return [
    {
      label: t('statistics.emailPreferences.enabled'),
      count: stats.value.enabled,
      color: ENABLED_COLOR,
    },
    {
      label: t('statistics.emailPreferences.disabled'),
      count: stats.value.disabled,
      color: DISABLED_COLOR,
    },
  ]
})

function renderChart() {
  if (!canvasRef.value || !stats.value) return

  chart?.destroy()
  chart = new Chart(canvasRef.value, {
    type: 'doughnut',
    data: {
      labels: legend.value.map((l) => l.label),
      datasets: [
        {
          label: t('statistics.emailPreferences.datasetLabel'),
          data: legend.value.map((l) => l.count),
          backgroundColor: legend.value.map((l) => l.color),
          borderWidth: 0,
          hoverOffset: 6,
        },
      ],
    },
    options: {
      responsive: true,
      maintainAspectRatio: false,
      cutout: '68%',
      plugins: {
        legend: { display: false },
        tooltip: {
          callbacks: {
            label: (ctx) => ` ${ctx.label}: ${ctx.parsed} (${percent(ctx.parsed)})`,
          },
        },
      },
    },
  })
}

async function fetchData() {
  loading.value = true
  try {
    stats.value = await statisticsService.getEmailPreferenceStats()
    await nextTick()
    renderChart()
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('statistics.errors.load'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

onMounted(fetchData)
onBeforeUnmount(() => chart?.destroy())
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="space-y-1">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2 min-w-0">
            <i class="pi pi-chart-pie text-primary-500 dark:text-primary-400 shrink-0"></i>
            <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0 truncate">
              {{ t('statistics.emailPreferences.title') }}
            </h2>
          </div>
          <Button icon="pi pi-refresh" v-tooltip.top="t('common.actions.refresh')" severity="secondary" text rounded
            :aria-label="t('common.actions.refresh')" :loading="loading" @click="fetchData" />
        </div>

        <p class="text-xs text-surface-400">
          {{ t('statistics.emailPreferences.description') }}
        </p>

        <div v-if="loading && !stats" class="flex items-center justify-center py-16">
          <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
        </div>

        <div v-else-if="isEmpty" class="flex flex-col items-center justify-center py-10 gap-2 text-surface-400">
          <i class="pi pi-chart-pie text-3xl"></i>
          <p class="text-sm">{{ t('statistics.emailPreferences.empty') }}</p>
        </div>

        <div v-else class="mt-4 flex flex-col sm:flex-row items-center gap-8">
          <div class="relative w-56 h-56 shrink-0">
            <canvas ref="canvasRef" class="block"></canvas>
            <div class="absolute inset-0 flex flex-col items-center justify-center pointer-events-none">
              <span class="text-2xl font-semibold text-surface-900 dark:text-surface-0">
                {{ stats?.total }}
              </span>
              <span class="text-xs text-surface-400">
                {{ t('statistics.emailPreferences.datasetLabel') }}
              </span>
            </div>
          </div>

          <ul class="flex-1 w-full space-y-3">
            <li v-for="item in legend" :key="item.label" class="flex items-center gap-3 text-sm">
              <span class="w-3 h-3 rounded-sm shrink-0" :style="{ backgroundColor: item.color }"></span>
              <span class="flex-1 text-surface-700 dark:text-surface-200">{{ item.label }}</span>
              <span class="font-semibold text-surface-900 dark:text-surface-0">{{
                item.count
              }}</span>
              <span class="w-14 text-right text-surface-400">{{ percent(item.count) }}</span>
            </li>
          </ul>
        </div>
      </div>
    </template>
  </Card>
</template>