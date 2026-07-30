<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useRouter } from 'vue-router'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'
import InputText from 'primevue/inputtext'
import ConfirmDialog from 'primevue/confirmdialog'
import { useToast } from 'primevue/usetoast'
import { useConfirm } from 'primevue/useconfirm'
import type { DataTableRowClickEvent } from 'primevue/datatable'
import { interestsService } from '@/services/interests.service'
import type { InterestResponse } from '@/types/interest.types'

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()
const confirm = useConfirm()
const router = useRouter()

const allInterests = ref<InterestResponse[]>([])
const loading = ref(false)
const search = ref('')

const filtered = computed(() => {
  const q = search.value.toLowerCase().trim()
  if (!q) return allInterests.value
  return allInterests.value.filter(
    (i) => i.name.toLowerCase().includes(q) || i.description?.toLowerCase().includes(q),
  )
})

async function fetchInterests() {
  loading.value = true
  try {
    allInterests.value = await interestsService.getAll()
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('interests.messages.loadListError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function onRowClick(event: DataTableRowClickEvent) {
  const interest = event.data as InterestResponse
  router.push(`/interests/${interest.id}/edit`)
}

function confirmDelete(event: Event, interest: InterestResponse) {
  event.stopPropagation()
  confirm.require({
    message: t('interests.deleteDialog.message', { name: interest.name }),
    header: t('interests.deleteDialog.title'),
    icon: 'pi pi-exclamation-triangle',
    rejectProps: {
      label: t('common.actions.cancel'),
      severity: 'secondary',
      outlined: true,
    },
    acceptProps: { label: t('common.actions.delete'), severity: 'danger' },
    accept: async () => {
      try {
        await interestsService.delete(interest.id)
        toast.add({
          severity: 'success',
          summary: t('common.feedback.deleted'),
          detail: t('interests.deleteDialog.success', { name: interest.name }),
          life: 3000,
        })
        fetchInterests()
      } catch {
        toast.add({
          severity: 'error',
          summary: t('common.feedback.error'),
          detail: t('interests.deleteDialog.error'),
          life: 3000,
        })
      }
    },
  })
}

onMounted(fetchInterests)
</script>

<template>
  <div class="p-6 space-y-4">
    <ConfirmDialog />

    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
          {{ t('interests.titles.list') }}
        </h1>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
          {{ t('interests.list.count', filtered.length) }}
        </p>
      </div>

      <div class="flex items-center gap-2">
        <div class="relative flex-1 sm:flex-none">
          <InputText
            v-model="search"
            data-tour="interests-search"
            :placeholder="t('common.placeholders.searchInterests')"
            class="pl-9 w-full sm:w-56"
          />
        </div>
        <Button
          :label="t('interests.actions.createNew')"
          data-tour="interests-add"
          icon="pi pi-plus"
          class="shrink-0"
          @click="router.push('/interests/new')"
        />
      </div>
    </div>

    <DataTable
      :value="filtered"
      :loading="loading"
      :rows="10"
      :rowsPerPageOptions="[5, 10, 25, 50]"
      data-tour="interests-table"
      paginator
      removableSort
      row-hover
      paginatorTemplate="RowsPerPageDropdown FirstPageLink PrevPageLink CurrentPageReport NextPageLink LastPageLink"
      currentPageReportTemplate="{first} to {last} of {totalRecords}"
      class="border border-surface-200 dark:border-surface-700 rounded-xl overflow-hidden"
      @row-click="onRowClick"
    >
      <Column field="name" :header="t('common.fields.name')" style="width: 30%" sortable>
        <template #body="{ data }: { data: InterestResponse }">
          <span class="font-medium text-surface-900 dark:text-surface-0">{{ data.name }}</span>
        </template>
      </Column>

      <Column field="description" :header="t('common.fields.description')" style="width: 60%">
        <template #body="{ data }: { data: InterestResponse }">
          <span class="text-surface-500 dark:text-surface-400 text-sm">
            {{ data.description ?? '—' }}
          </span>
        </template>
      </Column>

      <Column style="width: 10%">
        <template #body="{ data }: { data: InterestResponse }">
          <div class="flex justify-end">
            <Button
              icon="pi pi-trash"
              severity="danger"
              text
              rounded
              :aria-label="t('interests.actions.delete')"
              @click="confirmDelete($event, data)"
            />
          </div>
        </template>
      </Column>

      <template #empty>
        <div class="flex flex-col items-center justify-center py-16 gap-3 text-surface-400">
          <i class="pi pi-tag text-4xl"></i>
          <p class="text-sm">{{ t('interests.list.empty') }}</p>
        </div>
      </template>
    </DataTable>
  </div>
</template>
