import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function interestsUpsertTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="interest-form"]',
      popover: {
        title: t('tours.interests.upsert.form.title'),
        description: t('tours.interests.upsert.form.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="interest-save"]',
      popover: {
        title: t('tours.interests.upsert.save.title'),
        description: t('tours.interests.upsert.save.description'),
        side: 'top',
      },
    },
    {
      element: '[data-tour="interest-delete"]',
      popover: {
        title: t('tours.interests.upsert.delete.title'),
        description: t('tours.interests.upsert.delete.description'),
        side: 'bottom',
      },
    },
  ]
}
