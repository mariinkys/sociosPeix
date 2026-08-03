import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function homeTour(t: ComposerTranslation): DriveStep[] {
  return [
     {
      element: '[data-tour="members-scanForm"]',
      popover: {
        title: t('tours.members.scanForm.title'),
        description: t('tours.members.scanForm.description'),
        side: 'bottom',
      },
    },
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
    {
      element: '[data-tour="home-emails-today-transactional-button"]',
      popover: {
        title: t('tours.home.emailsTodayTransactionalButton.title'),
        description: t('tours.home.emailsTodayTransactionalButton.description'),
        side: 'bottom',
      },
    },
  ]
}
