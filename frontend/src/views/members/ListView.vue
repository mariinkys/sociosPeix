<script setup lang="ts">
import { ref, onMounted } from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import { useToast } from 'primevue/usetoast'
import Button from 'primevue/button'
import type {
  DataTableSortEvent,
  DataTablePageEvent,
  DataTableRowClickEvent,
} from 'primevue/datatable'
import { membersService } from '@/services/members.service'
import type { MemberResponse } from '@/types/member.types'
import { useRouter } from 'vue-router'

const toast = useToast()
const router = useRouter()

const members = ref<MemberResponse[]>([])
const loading = ref(false)
const totalElements = ref(0)

const page = ref(0)
const size = ref(10)
const sortBy = ref('createdAt')
const sortDir = ref<'asc' | 'desc'>('desc')

async function fetchMembers() {
  loading.value = true
  try {
    const data = await membersService.getAll({
      page: page.value,
      size: size.value,
      sortBy: sortBy.value,
      sortDir: sortDir.value,
    })
    members.value = data.content
    totalElements.value = data.totalElements
  } catch {
    toast.add({
      severity: 'error',
      summary: 'Error',
      detail: 'Failed to load members. Please try again.',
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

onMounted(fetchMembers)
</script>

<template>
  <div class="p-6 space-y-4">
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">Members</h1>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
          {{ totalElements }} total members
        </p>
      </div>

      <Button label="New Member" icon="pi pi-plus" @click="router.push('/members/new')" />
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
      <Column field="name" header="Name" sortable>
        <template #body="{ data }: { data: MemberResponse }">
          <div class="flex items-center gap-3">
            <div
              class="w-8 h-8 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 flex items-center justify-center text-sm font-semibold shrink-0"
            >
              {{ data.name.charAt(0).toUpperCase() }}
            </div>
            <span class="font-medium text-surface-900 dark:text-surface-0">{{ data.name }}</span>
          </div>
        </template>
      </Column>

      <Column field="email" header="Email" sortable>
        <template #body="{ data }: { data: MemberResponse }">
          <span class="text-surface-600 dark:text-surface-400">{{ data.email }}</span>
        </template>
      </Column>

      <Column field="createdAt" header="Created At" sortable>
        <template #body="{ data }: { data: MemberResponse }">
          <span class="text-surface-500 dark:text-surface-400 text-sm">
            {{
              new Date(data.createdAt).toLocaleDateString('en-GB', {
                day: 'numeric',
                month: 'short',
                year: 'numeric',
              })
            }}
          </span>
        </template>
      </Column>

      <template #empty>
        <div class="flex flex-col items-center justify-center py-16 gap-3 text-surface-400">
          <i class="pi pi-users text-4xl"></i>
          <p class="text-sm">No members found</p>
        </div>
      </template>
    </DataTable>
  </div>
</template>
