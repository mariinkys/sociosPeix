<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import AutoComplete from 'primevue/autocomplete'
import type { CountryResponse } from '@/types/country.types'
import { countriesService } from '@/services/countries.service'

const props = defineProps<{ modelValue: number | null; invalid?: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [value: number | null] }>()

const all = ref<CountryResponse[]>([])
const filtered = ref<CountryResponse[]>([])
const selected = ref<CountryResponse | null>(null)

function syncSelected(id: number | null) {
  selected.value = all.value.find((c) => c.id === id) ?? null
}

onMounted(async () => {
  all.value = await countriesService.getAll()
  syncSelected(props.modelValue)
})

watch(
  () => props.modelValue,
  (id) => syncSelected(id),
)

function search(event: { query: string }) {
  const q = event.query.toLowerCase()
  filtered.value = q ? all.value.filter((c) => c.name.toLowerCase().includes(q)) : [...all.value]
}

function onSelect(event: { value: CountryResponse }) {
  emit('update:modelValue', event.value.id)
}

function onClear() {
  emit('update:modelValue', null)
}
</script>

<template>
  <AutoComplete
    v-model="selected"
    :suggestions="filtered"
    optionLabel="name"
    placeholder="Search country..."
    forceSelection
    dropdown
    fluid
    :invalid="invalid"
    @complete="search"
    @option-select="onSelect"
    @clear="onClear"
  />
</template>
