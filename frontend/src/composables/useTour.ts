import { driver, type DriveStep, type Driver } from 'driver.js'
import 'driver.js/dist/driver.css'
import '@/assets/driver-theme.css'
import { watch } from 'vue'
import { useRoute } from 'vue-router'
import { useI18n } from 'vue-i18n'

let activeDriver: Driver | null = null

export function useTour() {
  const route = useRoute()
  const { t } = useI18n({ useScope: 'global' })

  function start(steps: DriveStep[]) {
    activeDriver?.destroy()

    const validSteps = steps.filter(
      (s) => typeof s.element !== 'string' || document.querySelector(s.element),
    )
    if (!validSteps.length) return

    activeDriver = driver({
      showProgress: true,
      progressText: t('tours.common.progress', { current: '{{current}}', total: '{{total}}' }),
      nextBtnText: t('tours.common.next'),
      prevBtnText: t('tours.common.previous'),
      doneBtnText: t('tours.common.done'),
      allowClose: true,
      overlayOpacity: 0.6,
      steps: validSteps,
      onPopoverRender: (popover) => {
        popover.closeButton.setAttribute('aria-label', t('tours.common.close'))
      },
      onDestroyStarted: () => {
        activeDriver?.destroy()
        activeDriver = null
      },
    })
    activeDriver.drive()
  }

  function stop() {
    activeDriver?.destroy()
    activeDriver = null
  }

  watch(
    () => route.fullPath,
    () => stop(),
  )

  return { start, stop }
}
