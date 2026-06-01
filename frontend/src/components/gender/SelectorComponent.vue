<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import AutoComplete from 'primevue/autocomplete'
import type { GenderResponse } from '@/types/gender.types'
import { gendersService } from '@/services/genders.service'

const props = defineProps<{ modelValue: number | null; invalid?: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [value: number | null] }>()

const { t } = useI18n({ useScope: 'global' })

const all = ref<GenderResponse[]>([])
const filtered = ref<GenderResponse[]>([])
const selected = ref<GenderResponse | null>(null)

function syncSelected(id: number | null) {
  selected.value = all.value.find((g) => g.id === id) ?? null
}

onMounted(async () => {
  all.value = await gendersService.getAll()
  syncSelected(props.modelValue)
})

watch(
  () => props.modelValue,
  (id) => syncSelected(id),
)

function search(event: { query: string }) {
  const q = event.query.toLowerCase()
  filtered.value = q ? all.value.filter((g) => g.name.toLowerCase().includes(q)) : [...all.value]
}

function onSelect(event: { value: GenderResponse }) {
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
    :placeholder="t('genders.selectorPlaceholder')"
    forceSelection
    dropdown
    fluid
    :invalid="invalid"
    @complete="search"
    @option-select="onSelect"
    @clear="onClear"
  />
</template>
