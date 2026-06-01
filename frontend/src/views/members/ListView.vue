<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'
import ConfirmDialog from 'primevue/confirmdialog'
import MultiSelect from 'primevue/multiselect'
import { useToast } from 'primevue/usetoast'
import { useConfirm } from 'primevue/useconfirm'
import InputText from 'primevue/inputtext'
import type {
  DataTableSortEvent,
  DataTablePageEvent,
  DataTableRowClickEvent,
} from 'primevue/datatable'
import { membersService } from '@/services/members.service'
import { interestsService } from '@/services/interests.service'
import type { MemberParams, MemberResponse } from '@/types/member.types'
import type { InterestResponse } from '@/types/interest.types'
import { useRouter } from 'vue-router'

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()
const confirm = useConfirm()
const router = useRouter()

const members = ref<MemberResponse[]>([])
const loading = ref(false)
const exportLoading = ref(false)
const totalElements = ref(0)

const search = ref('')
const selectedInterests = ref<InterestResponse[]>([])
const allInterests = ref<InterestResponse[]>([])
let debounceTimer: ReturnType<typeof setTimeout>

const page = ref(0)
const size = ref(10)
const sortBy = ref('createdAt')
const sortDir = ref<'asc' | 'desc'>('desc')

async function fetchMembers() {
  loading.value = true
  try {
    const params: MemberParams = {
      page: page.value,
      size: size.value,
      sortBy: sortBy.value,
      sortDir: sortDir.value,
      search: search.value || undefined,
      interestIds: selectedInterests.value.length
        ? selectedInterests.value.map((i) => i.id)
        : undefined,
    }
    const data = await membersService.getAll(params)
    members.value = data.content
    totalElements.value = data.totalElements
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('members.messages.loadListError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function onPage(event: DataTablePageEvent) {
  page.value = event.page
  size.value = event.rows
  fetchMembers()
}

function onSort(event: DataTableSortEvent) {
  if (typeof event.sortField === 'string') {
    sortBy.value = event.sortField
    sortDir.value = event.sortOrder === 1 ? 'asc' : 'desc'
    page.value = 0
    fetchMembers()
  }
}

function onRowClick(event: DataTableRowClickEvent) {
  const member = event.data as MemberResponse
  router.push(`/members/${member.id}/edit`)
}

function confirmDelete(event: Event, member: MemberResponse) {
  event.stopPropagation()
  confirm.require({
    message: t('members.deleteDialog.message', { name: member.fullName }),
    header: t('members.deleteDialog.title'),
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
      try {
        await membersService.delete(member.id)
        toast.add({
          severity: 'success',
          summary: t('common.feedback.deleted'),
          detail: t('members.deleteDialog.success', { name: member.fullName }),
          life: 3000,
        })
        fetchMembers()
      } catch {
        toast.add({
          severity: 'error',
          summary: t('common.feedback.error'),
          detail: t('members.deleteDialog.error'),
          life: 3000,
        })
      }
    },
  })
}

function onSearch() {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    page.value = 0
    fetchMembers()
  }, 500)
}

function onInterestFilter() {
  page.value = 0
  fetchMembers()
}

async function onExport() {
  exportLoading.value = true
  try {
    await membersService.export(
      search.value || undefined,
      selectedInterests.value.length ? selectedInterests.value.map((i) => i.id) : undefined,
    )
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('members.list.exportError'),
      life: 3000,
    })
  } finally {
    exportLoading.value = false
  }
}

onMounted(async () => {
  allInterests.value = await interestsService.getAll()
  fetchMembers()
})
</script>

<template>
  <div class="p-6 space-y-4">
    <ConfirmDialog />

    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
          {{ t('members.titles.list') }}
        </h1>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
          {{ t('members.list.total', { total: totalElements }) }}
        </p>
      </div>

      <div class="flex flex-wrap items-center gap-2">
        <div class="relative flex-1 sm:flex-none">
          <InputText
            v-model="search"
            :placeholder="t('members.filters.searchPlaceholder')"
            class="pl-9 w-full sm:w-48"
            @input="onSearch"
          />
        </div>

        <MultiSelect
          v-model="selectedInterests"
          :options="allInterests"
          optionLabel="name"
          :placeholder="t('members.filters.interestsPlaceholder')"
          display="chip"
          filter
          :filterPlaceholder="t('members.filters.interestsSearchPlaceholder')"
          class="w-full sm:w-56"
          @update:modelValue="onInterestFilter"
        />

        <Button
          :label="t('common.actions.export')"
          icon="pi pi-download"
          severity="secondary"
          outlined
          class="shrink-0"
          :disabled="loading"
          :loading="exportLoading"
          @click="onExport"
        />
        <Button
          :label="t('members.actions.createNew')"
          icon="pi pi-plus"
          class="shrink-0"
          @click="router.push('/members/new')"
        />
      </div>
    </div>

    <DataTable
      :value="members"
      :loading="loading"
      :rows="size"
      :totalRecords="totalElements"
      :rowsPerPageOptions="[5, 10, 25, 50]"
      lazy
      paginator
      removableSort
      row-hover
      paginatorTemplate="RowsPerPageDropdown FirstPageLink PrevPageLink CurrentPageReport NextPageLink LastPageLink"
      currentPageReportTemplate="{first} to {last} of {totalRecords}"
      class="border border-surface-200 dark:border-surface-700 rounded-xl overflow-hidden"
      @row-click="onRowClick"
      @page="onPage"
      @sort="onSort"
    >
      <Column field="name" :header="t('common.fields.name')" style="width: 30%" sortable>
        <template #body="{ data }: { data: MemberResponse }">
          <div class="flex items-center gap-3">
            <div
              class="w-8 h-8 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 flex items-center justify-center text-sm font-semibold shrink-0"
            >
              {{ data.fullName.charAt(0).toUpperCase() }}
            </div>
            <span class="font-medium text-surface-900 dark:text-surface-0">{{
              data.fullName
            }}</span>
          </div>
        </template>
      </Column>

      <Column field="email" :header="t('common.fields.email')" style="width: 20%" sortable>
        <template #body="{ data }: { data: MemberResponse }">
          <span class="text-surface-600 dark:text-surface-400">{{ data.email }}</span>
        </template>
      </Column>

      <Column field="phone" :header="t('common.fields.phone')" style="width: 20%" sortable>
        <template #body="{ data }: { data: MemberResponse }">
          <span class="text-surface-600 dark:text-surface-400">{{ data.phone }}</span>
        </template>
      </Column>

      <Column
        field="birthdate"
        :header="t('common.fields.birthdate')"
        style="width: 10%"
        sortable
      >
        <template #body="{ data }: { data: MemberResponse }">
          <span v-if="data.birthdate" class="text-surface-500 dark:text-surface-400 text-sm">
            {{
              new Date(data.birthdate).toLocaleDateString('es-ES', {
                day: 'numeric',
                month: 'short',
                year: 'numeric',
              })
            }}
          </span>
        </template>
      </Column>

      <Column
        field="createdAt"
        :header="t('common.fields.createdAt')"
        style="width: 10%"
        sortable
      >
        <template #body="{ data }: { data: MemberResponse }">
          <span v-if="data.createdAt" class="text-surface-500 dark:text-surface-400 text-sm">
            {{
              new Date(data.createdAt).toLocaleDateString('es-ES', {
                day: 'numeric',
                month: 'short',
                year: 'numeric',
              })
            }}
          </span>
        </template>
      </Column>

      <Column style="width: 10%">
        <template #body="{ data }: { data: MemberResponse }">
          <div class="flex justify-end">
            <Button
              icon="pi pi-trash"
              severity="danger"
              text
              rounded
              size="small"
              :aria-label="t('members.actions.delete')"
              @click="confirmDelete($event, data)"
            />
          </div>
        </template>
      </Column>

      <template #empty>
        <div class="flex flex-col items-center justify-center py-16 gap-3 text-surface-400">
          <i class="pi pi-users text-4xl"></i>
          <p class="text-sm">{{ t('members.list.empty') }}</p>
        </div>
      </template>
    </DataTable>
  </div>
</template>
