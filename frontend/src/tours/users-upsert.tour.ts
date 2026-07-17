import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'

export function usersUpsertTour(t: ComposerTranslation): DriveStep[] {
  return [
    {
      element: '[data-tour="user-form"]',
      popover: {
        title: t('tours.users.upsert.form.title'),
        description: t('tours.users.upsert.form.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="user-password-field"]',
      popover: {
        title: t('tours.users.upsert.passwordField.title'),
        description: t('tours.users.upsert.passwordField.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="user-save"]',
      popover: {
        title: t('tours.users.upsert.save.title'),
        description: t('tours.users.upsert.save.description'),
        side: 'top',
      },
    },
    {
      element: '[data-tour="user-password-card"]',
      popover: {
        title: t('tours.users.upsert.passwordCard.title'),
        description: t('tours.users.upsert.passwordCard.description'),
        side: 'top',
      },
    },
    {
      element: '[data-tour="user-role-card"]',
      popover: {
        title: t('tours.users.upsert.roleCard.title'),
        description: t('tours.users.upsert.roleCard.description'),
        side: 'top',
      },
    },
    {
      element: '[data-tour="user-delete"]',
      popover: {
        title: t('tours.users.upsert.delete.title'),
        description: t('tours.users.upsert.delete.description'),
        side: 'bottom',
      },
    },
  ]
}
