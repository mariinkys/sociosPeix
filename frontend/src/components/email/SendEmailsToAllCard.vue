<script setup lang="ts">
import { ref } from 'vue'
import Card from 'primevue/card'
import Button from 'primevue/button'
import SendEmailDialog from '@/components/email/SendEmailDialog.vue'

defineOptions({ inheritAttrs: false })

const dialogVisible = ref(false)
</script>

<template>
  <Card
    v-bind="$attrs"
    class="border border-surface-200 dark:border-surface-700 shadow-sm max-w-sm [&_.p-card-body]:h-full [&_.p-card-content]:h-full"
  >
    <template #content>
      <div class="p-2 flex flex-col justify-between h-full gap-4">
        <div class="space-y-4">
          <div class="flex items-center gap-2">
            <i class="pi pi-globe text-primary-500 dark:text-primary-400"></i>
            <h2 class="text-base font-semibold text-surface-900 dark:text-surface-0">
              Email All Members
            </h2>
          </div>
          <p class="text-sm text-surface-500 dark:text-surface-400">
            Send an email to every member in the database.
          </p>
        </div>
        <Button
          label="Compose Email"
          icon="pi pi-send"
          size="small"
          fluid
          @click="dialogVisible = true"
        />
      </div>
    </template>
  </Card>

  <SendEmailDialog mode="all" v-model:visible="dialogVisible" />
</template>
