<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import MultiSelect from 'primevue/multiselect'
import type { InterestResponse } from '@/types/interest.types'
import { interestsService } from '@/services/interests.service'

const props = defineProps<{ modelValue: number[]; invalid?: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [value: number[]] }>()

const { t } = useI18n({ useScope: 'global' })

const all = ref<InterestResponse[]>([])
const selected = ref<InterestResponse[]>([])

function syncSelected(ids: number[]) {
  selected.value = all.value.filter((i) => ids.includes(i.id))
}

onMounted(async () => {
  all.value = await interestsService.getAll()
  syncSelected(props.modelValue)
})

watch(
  () => props.modelValue,
  (ids) => syncSelected(ids),
)

function onChange(values: InterestResponse[]) {
  emit(
    'update:modelValue',
    values.map((i) => i.id),
  )
}

function remove(interest: InterestResponse) {
  const updated = selected.value.filter((i) => i.id !== interest.id)
  onChange(updated)
}
</script>

<template>
  <div class="flex flex-col gap-2">
    <MultiSelect
      v-model="selected"
      :options="all"
      :maxSelectedLabels="2"
      optionLabel="name"
      :placeholder="t('interests.multiSelect.placeholder')"
      filter
      :filterPlaceholder="t('interests.multiSelect.filterPlaceholder')"
      display="chip"
      :invalid="invalid"
      fluid
      @update:modelValue="onChange"
    />

    <div v-if="selected.length > 2" class="flex flex-wrap gap-1.5">
      <span
        v-for="interest in selected"
        :key="interest.id"
        class="inline-flex items-center gap-1.5 px-3 py-1 rounded-full text-sm font-medium bg-primary-100 text-primary-800 dark:bg-primary-900/40 dark:text-primary-300"
      >
        {{ interest.name }}
        <button
          type="button"
          class="ml-1 rounded-full hover:bg-primary-200 dark:hover:bg-primary-800 transition-colors p-0.5"
          :aria-label="`Remove ${interest.name}`"
          @click="remove(interest)"
        >
          <svg
            class="w-3.5 h-3.5"
            viewBox="0 0 12 12"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
          >
            <path d="M2 2l8 8M10 2l-8 8" />
          </svg>
        </button>
      </span>
    </div>
  </div>
</template>
