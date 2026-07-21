<script setup lang="ts">
import { NodeViewWrapper, nodeViewProps } from '@tiptap/vue-3'
import { computed, ref } from 'vue'

const props = defineProps(nodeViewProps)

const imageRef = ref<HTMLImageElement | null>(null)

const align = computed(() => (props.node.attrs.align as 'left' | 'center' | 'right') ?? 'left')

const justifyContent = computed(() => {
  if (align.value === 'center') return 'center'
  if (align.value === 'right') return 'flex-end'
  return 'flex-start'
})

function setAlign(value: 'left' | 'center' | 'right') {
  props.updateAttributes({ align: value })
}

function startResize(event: PointerEvent) {
  event.preventDefault()
  event.stopPropagation()
  if (!imageRef.value) return

  const startX = event.clientX
  const startWidth = imageRef.value.offsetWidth
  const maxWidth = imageRef.value.parentElement?.offsetWidth ?? startWidth
  const minWidth = 60

  function onMove(moveEvent: PointerEvent) {
    const delta = moveEvent.clientX - startX
    const newWidth = Math.min(maxWidth, Math.max(minWidth, startWidth + delta))
    props.updateAttributes({ width: Math.round(newWidth) })
  }

  function onUp() {
    window.removeEventListener('pointermove', onMove)
    window.removeEventListener('pointerup', onUp)
  }

  window.addEventListener('pointermove', onMove)
  window.addEventListener('pointerup', onUp)
}
</script>

<template>
  <NodeViewWrapper as="div">
    <div class="rte-image-wrapper" :style="{ justifyContent }">
      <div class="rte-image-figure" :class="{ 'is-selected': selected }">
        <img
          ref="imageRef"
          :src="node.attrs.src"
          :alt="node.attrs.alt"
          :style="node.attrs.width ? { width: `${node.attrs.width}px` } : {}"
          draggable="false"
        />

        <div v-if="selected" class="rte-image-toolbar" @mousedown.prevent.stop>
          <button
            type="button"
            class="rte-image-toolbar-btn"
            :class="{ active: align === 'left' }"
            title="Align left"
            @click="setAlign('left')"
          >
            <i class="pi pi-align-left"></i>
          </button>
          <button
            type="button"
            class="rte-image-toolbar-btn"
            :class="{ active: align === 'center' }"
            title="Align center"
            @click="setAlign('center')"
          >
            <i class="pi pi-align-center"></i>
          </button>
          <button
            type="button"
            class="rte-image-toolbar-btn"
            :class="{ active: align === 'right' }"
            title="Align right"
            @click="setAlign('right')"
          >
            <i class="pi pi-align-right"></i>
          </button>
        </div>

        <span v-if="selected" class="rte-image-handle" @pointerdown="startResize"></span>
      </div>
    </div>
  </NodeViewWrapper>
</template>

<style scoped>
.rte-image-wrapper {
  display: flex;
  width: 100%;
}

.rte-image-figure {
  position: relative;
  display: inline-block;
  line-height: 0;
  max-width: 100%;
}

.rte-image-figure.is-selected img {
  outline: 2px solid #6366f1;
  outline-offset: 2px;
}

.rte-image-toolbar {
  position: absolute;
  top: -34px;
  left: 50%;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  gap: 2px;
  padding: 3px;
  border-radius: 6px;
  background: #1e293b;
  box-shadow: 0 2px 6px rgba(0, 0, 0, 0.2);
  z-index: 5;
}

.rte-image-toolbar-btn {
  width: 22px;
  height: 22px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
  color: #cbd5e1;
  font-size: 11px;
  background: transparent;
}

.rte-image-toolbar-btn:hover {
  background: rgba(255, 255, 255, 0.1);
}

.rte-image-toolbar-btn.active {
  background: #6366f1;
  color: white;
}

.rte-image-handle {
  position: absolute;
  right: -4px;
  bottom: -4px;
  width: 12px;
  height: 12px;
  background: #6366f1;
  border: 2px solid white;
  border-radius: 50%;
  cursor: nwse-resize;
}
</style>
