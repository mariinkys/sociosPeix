import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function membersListTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="members-add"]',
      popover: {
        title: t('tours.members.add.title'),
        description: t('tours.members.add.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="members-filters"]',
      popover: {
        title: t('tours.members.filters.title'),
        description: t('tours.members.filters.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="members-table"]',
      popover: {
        title: t('tours.members.table.title'),
        description: t('tours.members.table.description'),
        side: 'top',
      },
    },
  ]
}
