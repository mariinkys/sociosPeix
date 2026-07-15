<script setup lang="ts">
import { computed } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'
import Button from 'primevue/button'
import { useTour } from '@/composables/useTour'
import { tourRegistry } from '@/tours/registry'

const route = useRoute()
const { t } = useI18n({ useScope: 'global' })
const { start } = useTour()

const hasTour = computed(() => !!route.name && route.name.toString() in tourRegistry)

async function runTour() {
  const name = route.name?.toString()
  const loader = name ? tourRegistry[name] : undefined
  if (!loader) return

  const steps = await loader(t, route)
  start(steps)
}
</script>

<template>
  <Button
    v-if="hasTour"
    v-tooltip.bottom="t('common.actions.help')"
    icon="pi pi-question-circle"
    severity="secondary"
    text
    rounded
    size="small"
    @click="runTour"
  />
</template>
