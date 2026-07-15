import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function usersListTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="users-add"]',
      popover: {
        title: t('tours.users.list.add.title'),
        description: t('tours.users.list.add.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="users-table"]',
      popover: {
        title: t('tours.users.list.table.title'),
        description: t('tours.users.list.table.description'),
        side: 'top',
      },
    },
  ]
}
