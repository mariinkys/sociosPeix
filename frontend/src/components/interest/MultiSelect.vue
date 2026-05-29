<script setup lang="ts">
import { ref, onMounted, watch } from 'vue'
import MultiSelect from 'primevue/multiselect'
import type { InterestResponse } from '@/types/interest.types'
import { interestsService } from '@/services/interests.service'

const props = defineProps<{ modelValue: number[]; invalid?: boolean }>()
const emit = defineEmits<{ 'update:modelValue': [value: number[]] }>()

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
</script>

<template>
  <MultiSelect
    v-model="selected"
    :options="all"
    optionLabel="name"
    placeholder="Select interests..."
    filter
    filterPlaceholder="Search interests..."
    display="chip"
    :invalid="invalid"
    fluid
    @update:modelValue="onChange"
  />
</template>
