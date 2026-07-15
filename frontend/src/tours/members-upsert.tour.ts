import type { DriveStep } from 'driver.js'
import type { ComposerTranslation } from 'vue-i18n'
import type { RouteLocationNormalizedLoaded } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

export function membersUpsertTour(
  t: ComposerTranslation,
  mode: 'new' | 'edit',
  route?: RouteLocationNormalizedLoaded,
): DriveStep[] {
  const steps: DriveStep[] = [
    {
      element: '[data-tour="user-email"]',
      popover: {
        title: t('tours.members.upsert.email.title'),
        description: t('tours.members.upsert.email.description'),
        side: 'bottom',
      },
    },
    {
      element: '[data-tour="user-role"]',
      popover: {
        title: t('tours.members.upsert.role.title'),
        description: t('tours.members.upsert.role.description'),
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
        title: t('tours.members.upsert.save.title'),
        description: isSelf
          ? t('tours.members.upsert.save.descriptionSelf')
          : t('tours.members.upsert.save.description'),
        side: 'top',
      },
    })
  }

  return steps
}
