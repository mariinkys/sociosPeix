import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function homeTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="home-birthdays"]',
      popover: {
        title: t('tours.home.birthdays.title'),
        description: t('tours.home.birthdays.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="home-emails-today"]',
      popover: {
        title: t('tours.home.emailsToday.title'),
        description: t('tours.home.emailsToday.description'),
        side: 'top',
      },
    },
  ]
}
