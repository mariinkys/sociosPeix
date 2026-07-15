import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'
import type { RouteLocationNormalizedLoaded } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

export function usersUpsertTour(
  t: ComposerTranslation,
  mode: 'new' | 'edit',
  route?: RouteLocationNormalizedLoaded,
): DriveStep[] {
  const steps: DriveStep[] = [
    {
      element: '[data-tour="user-email"]',
      popover: {
        title: t('tours.users.upsert.email.title'),
        description: t('tours.users.upsert.email.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="user-role"]',
      popover: {
        title: t('tours.users.upsert.role.title'),
        description: t('tours.users.upsert.role.description'),
        side: 'bottom',
      },
    },
  ]

  if (mode === 'edit') {
    const auth = useAuthStore()
    const isSelf = auth.user?.id === route?.params.id

    steps.push({
      element: '[data-tour="user-save"]',
      popover: {
        title: t('tours.users.upsert.save.title'),
        description: isSelf
          ? t('tours.users.upsert.save.descriptionSelf')
          : t('tours.users.upsert.save.description'),
        side: 'top',
      },
    })
  }

  return steps
}
