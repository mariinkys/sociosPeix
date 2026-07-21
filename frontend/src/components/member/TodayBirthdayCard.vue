c
<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import DataTable, { type DataTableRowClickEvent } from 'primevue/datatable'
import Column from 'primevue/column'
import Button from 'primevue/button'
import Card from 'primevue/card'
import { useToast } from 'primevue/usetoast'
import { membersService } from '@/services/members.service'
import type { MemberResponse } from '@/types/member.types'
import { useRouter } from 'vue-router'

const { t, locale } = useI18n({ useScope: 'global' })
const toast = useToast()
const router = useRouter()

const members = ref<MemberResponse[]>([])
const loading = ref(false)

async function fetchTodayBirthdays() {
  loading.value = true
  try {
    members.value = await membersService.getToday()
  } catch {
    toast.add({
      severity: 'error',
      summary: t('common.feedback.error'),
      detail: t('members.todayBirthdayCard.errors.load'),
      life: 3000,
    })
  } finally {
    loading.value = false
  }
}

function onRowClick(event: DataTableRowClickEvent) {
  const member = event.data as MemberResponse
  router.push(`/members/${member.id}/edit`)
}

onMounted(fetchTodayBirthdays)
</script>

<template>
  <Card class="border border-surface-200 dark:border-surface-700 shadow-sm">
    <template #content>
      <div class="space-y-4">
        <div class="flex items-center justify-between">
          <div class="flex items-center gap-2">
            <i class="pi pi-gift text-primary-500 dark:text-primary-400"></i>
            <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0">
              {{ t('members.todayBirthdayCard.title') }}
            </h2>
            <span
              v-if="members.length > 0"
              class="inline-flex items-center justify-center w-5 h-5 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 text-xs font-semibold"
            >
              {{ members.length }}
            </span>
          </div>

          <Button
            icon="pi pi-refresh"
            severity="secondary"
            text
            rounded
            :aria-label="t('common.actions.refresh')"
            :loading="loading"
            @click="fetchTodayBirthdays"
          />
        </div>

        <DataTable
          :value="members"
          :loading="loading"
          :rows="5"
          :rowsPerPageOptions="[5, 10, 25]"
          paginator
          paginatorTemplate="RowsPerPageDropdown FirstPageLink PrevPageLink CurrentPageReport NextPageLink LastPageLink"
          currentPageReportTemplate="{first} to {last} of {totalRecords}"
          row-hover
          @row-click="onRowClick"
        >
          <Column field="fullName" :header="t('common.fields.member')" style="width: 25%" sortable>
            <template #body="{ data }: { data: MemberResponse }">
              <div class="flex items-center gap-2">
                <div
                  class="w-7 h-7 rounded-full bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 flex items-center justify-center text-xs font-semibold shrink-0"
                >
                  {{ data.fullName.charAt(0).toUpperCase() }}
                </div>
                <span class="font-medium text-surface-900 dark:text-surface-0 text-sm">
                  {{ data.fullName }}
                </span>
              </div>
            </template>
          </Column>

          <Column field="email" :header="t('common.fields.email')" style="width: 25%">
            <template #body="{ data }: { data: MemberResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm">{{ data.email }}</span>
            </template>
          </Column>

          <Column field="phone" :header="t('common.fields.phone')" style="width: 25%">
            <template #body="{ data }: { data: MemberResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm">
                {{ data.phone ?? '—' }}
              </span>
            </template>
          </Column>

          <Column field="birthdate" :header="t('common.fields.birthday')" style="width: 25%">
            <template #body="{ data }: { data: MemberResponse }">
              <span class="text-surface-500 dark:text-surface-400 text-sm">
                {{
                  new Date(data.birthdate).toLocaleDateString(locale, {
                    day: 'numeric',
                    month: 'long',
                  })
                }}
              </span>
            </template>
          </Column>

          <template #empty>
            <div class="flex flex-col items-center justify-center py-10 gap-2 text-surface-400">
              <i class="pi pi-gift text-3xl"></i>
              <p class="text-sm">{{ t('members.todayBirthdayCard.empty') }}</p>
            </div>
          </template>
        </DataTable>
      </div>
    </template>
  </Card>
</template>
