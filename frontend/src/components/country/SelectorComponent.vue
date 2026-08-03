<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import Select from 'primevue/select'
import type { CountryResponse } from '@/types/country.types'
import { countriesService } from '@/services/countries.service'

const props = defineProps<{
  modelValue: number | null
  invalid?: boolean
  initialName?: string | null
}>()
const emit = defineEmits<{ 'update:modelValue': [value: number | null] }>()

const { t } = useI18n({ useScope: 'global' })

const all = ref<CountryResponse[]>([])

onMounted(async () => {
  all.value = await countriesService.getAll()

  if (props.initialName && props.modelValue == null) {
    const normalized = props.initialName.trim().toLowerCase()
    const match = all.value.find((c) => c.name.trim().toLowerCase() === normalized)
    if (match) {
      emit('update:modelValue', match.id)
    }
  }
})

watch(
  () => props.modelValue,
  () => {},
)

function onChange(value: number | null) {
  emit('update:modelValue', value)
}
</script>

<template>
  <Select
    :model-value="modelValue"
    :options="all"
    option-label="name"
    option-value="id"
    :placeholder="t('countries.selectorPlaceholder')"
    filter
    show-clear
    fluid
    :invalid="invalid"
    @change="onChange($event.value)"
  />
</template>
