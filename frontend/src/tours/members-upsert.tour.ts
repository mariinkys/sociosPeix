import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function membersUpsertTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="member-personal-details"]',
      popover: {
        title: t('tours.members.upsert.personalDetails.title'),
        description: t('tours.members.upsert.personalDetails.description'),
        side: 'right',
      },
    },
    {
      element: '[data-tour="member-contact"]',
      popover: {
        title: t('tours.members.upsert.contact.title'),
        description: t('tours.members.upsert.contact.description'),
        side: 'right',
      },
    },
    {
      element: '[data-tour="member-interests"]',
      popover: {
        title: t('tours.members.upsert.interests.title'),
        description: t('tours.members.upsert.interests.description'),
        side: 'left',
      },
    },
    {
      element: '[data-tour="member-emails-history"]',
      popover: {
        title: t('tours.members.upsert.emailsHistory.title'),
        description: t('tours.members.upsert.emailsHistory.description'),
        side: 'left',
      },
    },
    {
      element: '[data-tour="member-save"]',
      popover: {
        title: t('tours.members.upsert.save.title'),
        description: t('tours.members.upsert.save.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="member-delete"]',
      popover: {
        title: t('tours.members.upsert.delete.title'),
        description: t('tours.members.upsert.delete.description'),
        side: 'bottom',
      },
    },
  ]
}
