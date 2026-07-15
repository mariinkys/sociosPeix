import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function interestsListTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="interests-add"]',
      popover: {
        title: t('tours.interests.list.add.title'),
        description: t('tours.interests.list.add.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="interests-table"]',
      popover: {
        title: t('tours.interests.list.table.title'),
        description: t('tours.interests.list.table.description'),
        side: 'top',
      },
    },
  ]
}
