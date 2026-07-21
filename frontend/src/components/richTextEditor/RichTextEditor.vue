<script setup lang="ts">
import { onBeforeUnmount, computed, ref, watch, nextTick } from 'vue'
import { useEditor, EditorContent } from '@tiptap/vue-3'
import StarterKit from '@tiptap/starter-kit'
import Underline from '@tiptap/extension-underline'
import Link from '@tiptap/extension-link'
import TextAlign from '@tiptap/extension-text-align'
import { TextStyle, FontFamily } from '@tiptap/extension-text-style'
import { Placeholder } from '@tiptap/extensions'
import { ResizableImage } from '@/extensions/ResizableImage'

const props = withDefaults(
  defineProps<{
    placeholder?: string
    minHeight?: string
  }>(),
  {
    placeholder: '',
    minHeight: '260px',
  },
)

const modelValue = defineModel<string>({ required: true })

const editor = useEditor({
  content: modelValue.value,
  extensions: [
    StarterKit,
    Underline,
    Link.configure({ openOnClick: false, autolink: true }),
    TextAlign.configure({ types: ['heading', 'paragraph'] }),
    TextStyle,
    FontFamily,
    ResizableImage,
    Placeholder.configure({ placeholder: props.placeholder }),
  ],
  onUpdate: ({ editor: e }) => {
    modelValue.value = e.getHTML()
  },
})

onBeforeUnmount(() => {
  editor.value?.destroy()
  document.removeEventListener('click', onDocumentClick)
})

const BLOCK_TYPE_OPTIONS = [
  { label: 'Paragraph', value: 'paragraph' },
  { label: 'Heading 1', value: 'h1' },
  { label: 'Heading 2', value: 'h2' },
  { label: 'Heading 3', value: 'h3' },
]

const currentBlockType = computed(() => {
  if (!editor.value) return 'paragraph'
  if (editor.value.isActive('heading', { level: 1 })) return 'h1'
  if (editor.value.isActive('heading', { level: 2 })) return 'h2'
  if (editor.value.isActive('heading', { level: 3 })) return 'h3'
  return 'paragraph'
})

function onBlockTypeChange(event: Event) {
  const value = (event.target as HTMLSelectElement).value
  if (!editor.value) return
  if (value === 'paragraph') {
    editor.value.chain().focus().setParagraph().run()
    return
  }
  const level = Number(value.replace('h', '')) as 1 | 2 | 3
  editor.value.chain().focus().toggleHeading({ level }).run()
}

type AlignValue = 'left' | 'center' | 'right' | 'justify'

function applyAlign(align: AlignValue) {
  if (!editor.value) return
  if (editor.value.isActive('image')) {
    if (align === 'justify') return // justify has no meaning for a single image
    editor.value.chain().focus().updateAttributes('image', { align }).run()
    return
  }
  editor.value.chain().focus().setTextAlign(align).run()
}

function isAlignActive(align: AlignValue): boolean {
  if (!editor.value) return false
  if (editor.value.isActive('image')) {
    return (editor.value.getAttributes('image').align ?? 'left') === align
  }
  return editor.value.isActive({ textAlign: align })
}

const showLinkPopover = ref(false)
const linkUrl = ref('')
const linkPopoverRef = ref<HTMLElement | null>(null)

function openLinkPopover() {
  if (!editor.value) return
  linkUrl.value = (editor.value.getAttributes('link').href as string | undefined) ?? ''
  showLinkPopover.value = true
}

function applyLink() {
  if (!editor.value) return
  const url = linkUrl.value.trim()
  if (url === '') {
    editor.value.chain().focus().extendMarkRange('link').unsetLink().run()
  } else {
    editor.value.chain().focus().extendMarkRange('link').setLink({ href: url }).run()
  }
  showLinkPopover.value = false
}

function removeLink() {
  editor.value?.chain().focus().extendMarkRange('link').unsetLink().run()
  showLinkPopover.value = false
}

function onDocumentClick(event: MouseEvent) {
  if (linkPopoverRef.value && !linkPopoverRef.value.contains(event.target as Node)) {
    showLinkPopover.value = false
  }
}

watch(showLinkPopover, (open) => {
  if (open) {
    nextTick(() => document.addEventListener('click', onDocumentClick))
  } else {
    document.removeEventListener('click', onDocumentClick)
  }
})

const MAX_IMAGE_BYTES = 5 * 1024 * 1024
const imageInput = ref<HTMLInputElement | null>(null)
const imageError = ref('')

function triggerImagePicker() {
  imageError.value = ''
  imageInput.value?.click()
}

function onImageSelected(event: Event) {
  const input = event.target as HTMLInputElement
  const file = input.files?.[0]
  input.value = ''
  if (!file) return

  if (file.size > MAX_IMAGE_BYTES) {
    imageError.value = 'Image is too large (max 5 MB)'
    return
  }

  const reader = new FileReader()
  reader.onload = () => {
    const dataUrl = reader.result as string
    editor.value?.chain().focus().setImage({ src: dataUrl, alt: file.name }).run()
  }
  reader.readAsDataURL(file)
}
</script>

<template>
  <div
    v-if="editor"
    class="rounded-lg border border-surface-200 dark:border-surface-700 overflow-hidden"
  >
    <div
      class="flex flex-wrap items-center gap-1 px-2 py-1.5 border-b border-surface-200 dark:border-surface-700 bg-surface-50 dark:bg-surface-800"
    >
      <select
        :value="currentBlockType"
        class="h-7 text-xs rounded border border-surface-200 dark:border-surface-700 bg-white dark:bg-surface-900 text-surface-700 dark:text-surface-300 px-1.5"
        @change="onBlockTypeChange"
      >
        <option v-for="opt in BLOCK_TYPE_OPTIONS" :key="opt.value" :value="opt.value">
          {{ opt.label }}
        </option>
      </select>

      <span class="w-px h-5 bg-surface-200 dark:bg-surface-700 mx-1"></span>

      <button
        v-for="btn in [
          {
            cmd: () => editor?.chain().focus().toggleBold().run(),
            active: 'bold',
            label: 'B',
            glyphClass: 'font-bold',
          },
          {
            cmd: () => editor?.chain().focus().toggleItalic().run(),
            active: 'italic',
            label: 'I',
            glyphClass: 'italic',
          },
          {
            cmd: () => editor?.chain().focus().toggleUnderline().run(),
            active: 'underline',
            label: 'U',
            glyphClass: 'underline',
          },
          {
            cmd: () => editor?.chain().focus().toggleStrike().run(),
            active: 'strike',
            label: 'S',
            glyphClass: 'line-through',
          },
        ]"
        :key="btn.label"
        type="button"
        class="w-7 h-7 flex items-center justify-center rounded text-sm transition-colors"
        :class="[
          btn.glyphClass,
          editor.isActive(btn.active)
            ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
            : 'text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700',
        ]"
        @click="btn.cmd()"
      >
        {{ btn.label }}
      </button>

      <span class="w-px h-5 bg-surface-200 dark:bg-surface-700 mx-1"></span>

      <button
        v-for="btn in [
          { value: 'left' as const, icon: 'pi-align-left' },
          { value: 'center' as const, icon: 'pi-align-center' },
          { value: 'right' as const, icon: 'pi-align-right' },
          { value: 'justify' as const, icon: 'pi-align-justify' },
        ]"
        :key="btn.icon"
        type="button"
        class="w-7 h-7 flex items-center justify-center rounded transition-colors"
        :class="
          isAlignActive(btn.value)
            ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
            : 'text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700'
        "
        @click="applyAlign(btn.value)"
      >
        <i :class="`pi ${btn.icon} text-sm`"></i>
      </button>

      <span class="w-px h-5 bg-surface-200 dark:bg-surface-700 mx-1"></span>

      <button
        type="button"
        class="w-7 h-7 flex items-center justify-center rounded transition-colors"
        :class="
          editor.isActive('bulletList')
            ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
            : 'text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700'
        "
        @click="editor.chain().focus().toggleList('bulletList', 'listItem').run()"
      >
        <i class="pi pi-list text-sm"></i>
      </button>
      <button
        type="button"
        class="w-7 h-7 flex items-center justify-center rounded text-xs font-semibold transition-colors"
        :class="
          editor.isActive('orderedList')
            ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
            : 'text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700'
        "
        @click="editor.chain().focus().toggleList('orderedList', 'listItem').run()"
      >
        1.
      </button>
      <button
        type="button"
        class="w-7 h-7 flex items-center justify-center rounded text-lg font-serif leading-none transition-colors"
        :class="
          editor.isActive('blockquote')
            ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
            : 'text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700'
        "
        @click="editor.chain().focus().toggleBlockquote().run()"
      >
        "
      </button>

      <span class="w-px h-5 bg-surface-200 dark:bg-surface-700 mx-1"></span>

      <div ref="linkPopoverRef" class="relative">
        <button
          type="button"
          class="w-7 h-7 flex items-center justify-center rounded transition-colors"
          :class="
            editor.isActive('link')
              ? 'bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300'
              : 'text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700'
          "
          @click="openLinkPopover"
        >
          <i class="pi pi-link text-sm"></i>
        </button>

        <div
          v-if="showLinkPopover"
          class="absolute z-10 top-full left-0 mt-1 w-64 rounded-lg border border-surface-200 dark:border-surface-700 bg-white dark:bg-surface-900 shadow-lg p-2 flex flex-col gap-2"
        >
          <input
            v-model="linkUrl"
            type="text"
            placeholder="https://example.com"
            class="h-8 text-xs rounded border border-surface-200 dark:border-surface-700 bg-white dark:bg-surface-900 text-surface-700 dark:text-surface-300 px-2"
            @keydown.enter.prevent="applyLink"
            @keydown.escape.prevent="showLinkPopover = false"
          />
          <div class="flex items-center justify-end gap-1">
            <button
              v-if="editor.isActive('link')"
              type="button"
              class="text-xs px-2 py-1 rounded text-red-600 hover:bg-red-50 dark:hover:bg-red-950"
              @click="removeLink"
            >
              Remove
            </button>
            <button
              type="button"
              class="text-xs px-2 py-1 rounded bg-primary-500 text-white hover:bg-primary-600"
              @click="applyLink"
            >
              Apply
            </button>
          </div>
        </div>
      </div>

      <span class="w-px h-5 bg-surface-200 dark:bg-surface-700 mx-1"></span>

      <button
        type="button"
        class="w-7 h-7 flex items-center justify-center rounded text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700"
        @click="triggerImagePicker"
      >
        <svg
          width="15"
          height="15"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
        >
          <rect x="3" y="3" width="18" height="18" rx="2" />
          <circle cx="8.5" cy="8.5" r="1.5" />
          <path d="M21 15l-5-5L5 21" />
        </svg>
      </button>
      <input
        ref="imageInput"
        type="file"
        accept="image/*"
        class="hidden"
        @change="onImageSelected"
      />

      <span class="w-px h-5 bg-surface-200 dark:bg-surface-700 mx-1"></span>

      <button
        type="button"
        class="w-7 h-7 flex items-center justify-center rounded text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700 disabled:opacity-30 disabled:hover:bg-transparent"
        :disabled="!editor.can().undo()"
        @click="editor.chain().focus().undo().run()"
      >
        <i class="pi pi-replay text-sm"></i>
      </button>
      <button
        type="button"
        class="w-7 h-7 flex items-center justify-center rounded text-surface-600 dark:text-surface-400 hover:bg-surface-200 dark:hover:bg-surface-700 disabled:opacity-30 disabled:hover:bg-transparent"
        :disabled="!editor.can().redo()"
        @click="editor.chain().focus().redo().run()"
      >
        <i class="pi pi-refresh text-sm"></i>
      </button>
    </div>

    <p
      v-if="imageError"
      class="px-3 py-1 text-xs text-red-500 border-b border-surface-200 dark:border-surface-700"
    >
      {{ imageError }}
    </p>

    <EditorContent
      :editor="editor"
      class="text-surface-900 dark:text-surface-0"
      :style="{ '--rte-min-height': minHeight }"
    />
  </div>
</template>

<style scoped>
:deep(.tiptap) {
  min-height: var(--rte-min-height, 260px);
  max-height: 400px;
  overflow-y: auto;
  padding: 0.75rem 1rem;
  outline: none;
  font-size: 0.875rem;
  color: inherit;
}

:deep(.tiptap p.is-editor-empty:first-child::before) {
  content: attr(data-placeholder);
  float: left;
  height: 0;
  pointer-events: none;
  color: #94a3b8;
}

:deep(.tiptap p:empty) {
  margin: 0;
  min-height: 0;
}

:deep(.tiptap h1) {
  font-size: 1.5rem;
  font-weight: 700;
  margin: 0.75rem 0 0.5rem;
}

:deep(.tiptap h2) {
  font-size: 1.25rem;
  font-weight: 700;
  margin: 0.75rem 0 0.5rem;
}

:deep(.tiptap h3) {
  font-size: 1.1rem;
  font-weight: 600;
  margin: 0.75rem 0 0.5rem;
}

:deep(.tiptap ul) {
  list-style-type: disc;
  list-style-position: outside;
  padding-left: 1.25rem;
  margin: 0.5rem 0;
}

:deep(.tiptap ol) {
  list-style-type: decimal;
  list-style-position: outside;
  padding-left: 1.25rem;
  margin: 0.5rem 0;
}

:deep(.tiptap li) {
  margin: 0.15rem 0;
}

:deep(.tiptap li p) {
  margin: 0;
}

:deep(.tiptap blockquote) {
  border-left: 3px solid #cbd5e1;
  padding-left: 0.75rem;
  margin: 0.5rem 0;
  color: #64748b;
}

:deep(.tiptap a) {
  color: #6366f1;
  text-decoration: underline;
  cursor: pointer;
}

:deep(.tiptap p) {
  margin: 0.375rem 0;
}
</style>
