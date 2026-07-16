import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function membersListTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="members-search"]',
      popover: {
        title: t('tours.members.search.title'),
        description: t('tours.members.search.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="members-interest-filter"]',
      popover: {
        title: t('tours.members.interestFilter.title'),
        description: t('tours.members.interestFilter.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="members-export"]',
      popover: {
        title: t('tours.members.export.title'),
        description: t('tours.members.export.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="members-add"]',
      popover: {
        title: t('tours.members.add.title'),
        description: t('tours.members.add.description'),
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
