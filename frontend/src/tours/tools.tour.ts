import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function toolsTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="tools-quota"]',
      popover: {
        title: t('tours.tools.quota.title'),
        description: t('tours.tools.quota.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="tools-send-interest"]',
      popover: {
        title: t('tours.tools.sendByInterest.title'),
        description: t('tours.tools.sendByInterest.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="tools-send-all"]',
      popover: {
        title: t('tours.tools.sendToAll.title'),
        description: t('tours.tools.sendToAll.description'),
        side: 'top',
      },
    },
    {
      element: '[data-tour="tools-version-chip"]',
      popover: {
        title: t('tours.tools.versionChip.title'),
        description: t('tours.tools.versionChip.description'),
        side: 'top',
      },
    },
  ]
}
