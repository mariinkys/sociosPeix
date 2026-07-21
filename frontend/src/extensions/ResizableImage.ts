import { Image } from '@tiptap/extension-image'
import { VueNodeViewRenderer } from '@tiptap/vue-3'
import ResizableImageView from '@/components/richTextEditor/ResizableImage.vue'

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
    }
  },

  addNodeView() {
    return VueNodeViewRenderer(ResizableImageView)
  },
})
