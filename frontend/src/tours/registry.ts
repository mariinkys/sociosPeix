import type { ComposerTranslation } from 'vue-i18n'
import type { DriveStep } from 'driver.js'
import type { RouteLocationNormalizedLoaded } from 'vue-router'

export type TourLoader = (
  t: ComposerTranslation,
  route: RouteLocationNormalizedLoaded,
) => Promise<DriveStep[]> | DriveStep[]

export const tourRegistry: Record<string, TourLoader> = {
  Users: async (t) => (await import('./users-list.tour.ts')).usersListTour(t),
  'New User': async (t) => (await import('./users-upsert.tour')).usersUpsertTour(t),
  'Update User': async (t) => (await import('./users-upsert.tour')).usersUpsertTour(t),

  Members: async (t) => (await import('./members-list.tour.ts')).membersListTour(t),
  'New Member': async (t) => (await import('./members-upsert.tour')).membersUpsertTour(t),
  'Update Member': async (t) => (await import('./members-upsert.tour')).membersUpsertTour(t),

  Interests: async (t) => (await import('./interests-list.tour.ts')).interestsListTour(t),
  'New Interest': async (t) => (await import('./interests-upsert.tour')).interestsUpsertTour(t),
  'Update Interest': async (t) => (await import('./interests-upsert.tour')).interestsUpsertTour(t),

  Tools: async (t) => (await import('./tools.tour.ts')).toolsTour(t),
  Home: async (t) => (await import('./home.tour.ts')).homeTour(t),
}
