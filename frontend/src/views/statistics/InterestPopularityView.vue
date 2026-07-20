<script setup lang="ts">
import { ref, onMounted, onBeforeUnmount, nextTick, watch, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import Card from 'primevue/card'
import Button from 'primevue/button'
import { useToast } from 'primevue/usetoast'
import { Chart, BarController, BarElement, CategoryScale, LinearScale, Tooltip } from 'chart.js'
import { statisticsService } from '@/services/statistics.service'
import type { InterestPopularity } from '@/types/statistics.types'

Chart.register(BarController, BarElement, CategoryScale, LinearScale, Tooltip)

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()

const data = ref<InterestPopularity[]>([])
const loading = ref(false)
const exporting = ref(false)

const canvasRef = ref<HTMLCanvasElement | null>(null)
let chart: Chart | null = null

const ROW_HEIGHT_PX = 28
const MIN_CHART_HEIGHT_PX = 360

const chartHeight = computed(() => Math.max(data.value.length * ROW_HEIGHT_PX, MIN_CHART_HEIGHT_PX))

function renderChart() {
  if (!canvasRef.value) return

  chart?.destroy()
  chart = new Chart(canvasRef.value, {
    type: 'bar',
    data: {
      labels: data.value.map((d) => d.interestName),
      datasets: [
        {
          label: t('statistics.interestPopularity.datasetLabel'),
          data: data.value.map((d) => d.memberCount),
          backgroundColor: '#6366f1',
          borderRadius: 4,
          maxBarThickness: 22,
        },
      ],
    },
    options: {
      indexAxis: 'y',
      responsive: true,
      maintainAspectRatio: false,
      plugins: { legend: { display: false } },
      scales: {
        x: {
          beginAtZero: true,
          ticks: { stepSize: 1, color: '#94a3b8' },
          grid: { color: 'rgba(148, 163, 184, 0.15)' },
        },
        y: {
          ticks: { color: '#94a3b8' },
          grid: { display: false },
        },
      },
    },
  })
}

async function fetchData() {
  loading.value = true
  try {
    data.value = await statisticsService.getInterestPopularity()
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

async function onExport() {
  exporting.value = true
  try {
    await statisticsService.exportInterestPopularity()
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('statistics.errors.export'),
      life: 3000,
    })
  } finally {
    exporting.value = false
  }
}

watch(chartHeight, async () => {
  await nextTick()
  renderChart()
})

onMounted(fetchData)
onBeforeUnmount(() => chart?.destroy())
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="space-y-1">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2 min-w-0">
            <i class="pi pi-chart-bar text-primary-500 dark:text-primary-400 shrink-0"></i>
            <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0 truncate">
              {{ t('statistics.interestPopularity.title') }}
            </h2>
          </div>
          <div class="flex items-center gap-1 shrink-0">
            <Button
              icon="pi pi-file-excel"
              severity="secondary"
              v-tooltip.top="t('common.actions.export')"
              text
              rounded
              size="small"
              :aria-label="t('common.actions.export')"
              :loading="exporting"
              @click="onExport"
            />
            <Button
              icon="pi pi-refresh"
              v-tooltip.top="t('common.actions.refresh')"
              severity="secondary"
              text
              rounded
              size="small"
              :aria-label="t('common.actions.refresh')"
              :loading="loading"
              @click="fetchData"
            />
          </div>
        </div>

        <p class="text-xs text-surface-400">
          {{ t('statistics.interestPopularity.description') }}
        </p>

        <div v-if="loading && data.length === 0" class="flex items-center justify-center py-16">
          <i class="pi pi-spinner pi-spin text-2xl text-surface-400"></i>
        </div>

        <div
          v-else-if="data.length === 0"
          class="flex flex-col items-center justify-center py-10 gap-2 text-surface-400"
        >
          <i class="pi pi-chart-bar text-3xl"></i>
          <p class="text-sm">{{ t('statistics.interestPopularity.empty') }}</p>
        </div>

        <div v-else class="mt-4 relative w-full" :style="{ height: `${chartHeight}px` }">
          <canvas ref="canvasRef" class="block w-full"></canvas>
        </div>
      </div>
    </template>
  </Card>
</template>
