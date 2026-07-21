import { Image } from '@tiptap/extension-image'
import { VueNodeViewRenderer } from '@tiptap/vue-3'
import ResizableImageView from '@/components/richTextEditor/ResizableImage.vue'

declare module '@tiptap/extension-image' {
  interface SetImageOptions {
    contentId?: string | null
    width?: number | undefined
    align?: 'left' | 'center' | 'right'
  }
}

export type ImageAlign = 'left' | 'center' | 'right'

export const ResizableImage = Image.extend({
  addAttributes() {
    return {
      ...this.parent?.(),
      width: {
        default: null,
        renderHTML: (attributes: { width?: number | null }) => {
          if (!attributes.width) return {}
          return { style: `width: ${attributes.width}px` }
        },
        parseHTML: (element: HTMLElement) => {
          const width = element.style.width
          return width ? parseInt(width, 10) : null
        },
      },
      align: {
        default: 'left',
        renderHTML: (attributes: { align?: ImageAlign | null }) => {
          if (!attributes.align || attributes.align === 'left') return {}
          if (attributes.align === 'center') {
            return { style: 'display: block; margin-left: auto; margin-right: auto;' }
          }
          return { style: 'display: block; margin-left: auto; margin-right: 0;' }
        },
        parseHTML: (element: HTMLElement): ImageAlign => {
          const { marginLeft, marginRight } = element.style
          if (marginLeft === 'auto' && marginRight === 'auto') return 'center'
          if (marginLeft === 'auto' && marginRight !== 'auto') return 'right'
          return 'left'
        },
      },
      // only set for locally-uploaded images. Rendered as data-cid so it survives
      // getHTML() - used at send time to know which <img> tags need their blob:
      // src swapped for a real cid: reference. URL-inserted images never get this.
      contentId: {
        default: null,
        renderHTML: (attributes: { contentId?: string | null }) => {
          if (!attributes.contentId) return {}
          return { 'data-cid': attributes.contentId }
        },
        parseHTML: (element: HTMLElement) => element.getAttribute('data-cid') || null,
      },
    }
  },

  addNodeView() {
    return VueNodeViewRenderer(ResizableImageView)
  },
})
