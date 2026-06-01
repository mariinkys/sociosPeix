<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ref, onMounted } from 'vue'
import DataTable from 'primevue/datatable'
import Column from 'primevue/column'
import Tag from 'primevue/tag'
import Button from 'primevue/button'
import { useToast } from 'primevue/usetoast'
import InputText from 'primevue/inputtext'
import type {
  DataTableSortEvent,
  DataTablePageEvent,
  DataTableRowClickEvent,
} from 'primevue/datatable'
import { usersService } from '@/services/users.service'
import type { UserResponse } from '@/types/user.types'
import { useRouter } from 'vue-router'

const { t } = useI18n({ useScope: 'global' })
const toast = useToast()
const router = useRouter()

const users = ref<UserResponse[]>([])
const loading = ref(false)
const totalElements = ref(0)

const search = ref('')
let debounceTimer: ReturnType<typeof setTimeout>

const page = ref(0)
const size = ref(10)
const sortBy = ref('createdAt')
const sortDir = ref<'asc' | 'desc'>('desc')

async function fetchUsers() {
  loading.value = true
  try {
    const data = await usersService.getAll({
      page: page.value,
      size: size.value,
      sortBy: sortBy.value,
      sortDir: sortDir.value,
      search: search.value || undefined,
    })
    users.value = data.content
    totalElements.value = data.totalElements
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('users.messages.loadListError'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function onPage(event: DataTablePageEvent) {
  page.value = event.page
  size.value = event.rows
  fetchUsers()
}

function onSort(event: DataTableSortEvent) {
  if (typeof event.sortField === 'string') {
    sortBy.value = event.sortField
    sortDir.value = event.sortOrder === 1 ? 'asc' : 'desc'
    page.value = 0
    fetchUsers()
  }
}

function onSearch() {
  clearTimeout(debounceTimer)
  debounceTimer = setTimeout(() => {
    page.value = 0
    fetchUsers()
  }, 500)
}

function onRowClick(event: DataTableRowClickEvent) {
  const user = event.data as UserResponse
  router.push(`/users/${user.id}/edit`)
}

onMounted(fetchUsers)
</script>

<template>
  <div class="p-6 space-y-4">
    <div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
      <div>
        <h1 class="text-xl font-semibold text-surface-900 dark:text-surface-0">
          {{ t('users.titles.list') }}
        </h1>
        <p class="text-sm text-surface-500 dark:text-surface-400 mt-0.5">
          {{ t('users.list.total', { total: totalElements }) }}
        </p>
      </div>

      <div class="flex items-center gap-2">
        <div class="relative flex-1 sm:flex-none">
          <InputText
            v-model="search"
            :placeholder="t('common.placeholders.searchUsers')"
            class="pl-9 w-full sm:w-56"
            @input="onSearch"
          />
        </div>
        <Button
          :label="t('users.actions.createNew')"
          icon="pi pi-plus"
          class="shrink-0"
          @click="router.push('/users/new')"
        />
      </div>
    </div>

    <DataTable
      :value="users"
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
        <template #body="{ data }: { data: UserResponse }">
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

      <Column field="email" :header="t('common.fields.email')" style="width: 30%" sortable>
        <template #body="{ data }: { data: UserResponse }">
          <span class="text-surface-600 dark:text-surface-400">{{ data.email }}</span>
        </template>
      </Column>

      <Column field="role" :header="t('common.fields.role')" style="width: 20%">
        <template #body="{ data }: { data: UserResponse }">
          <Tag :value="data.role" :severity="data.role === 'ADMIN' ? 'info' : 'success'" />
        </template>
      </Column>

      <Column
        field="createdAt"
        :header="t('common.fields.createdAt')"
        style="width: 20%"
        sortable
      >
        <template #body="{ data }: { data: UserResponse }">
          <span class="text-surface-500 dark:text-surface-400 text-sm">
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

      <template #empty>
        <div class="flex flex-col items-center justify-center py-16 gap-3 text-surface-400">
          <i class="pi pi-users text-4xl"></i>
          <p class="text-sm">{{ t('users.list.empty') }}</p>
        </div>
      </template>
    </DataTable>
  </div>
</template>
