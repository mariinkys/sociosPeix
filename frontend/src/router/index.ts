import { useAuthStore } from '@/stores/auth'
import { createRouter, createWebHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('@/views/HomeView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('@/views/auth/LoginView.vue'),
    meta: { guestOnly: true },
  },
  {
    path: '/users',
    name: 'Users',
    component: () => import('@/views/users/ListView.vue'),
    meta: { requiresAuth: true, requiresAdmin: true },
  },
  {
    path: '/members',
    name: 'Members',
    component: () => import('@/views/members/ListView.vue'),
    meta: { requiresAuth: true },
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach(async (to) => {
  const auth = useAuthStore()

  // On first navigation, restore session from cookie
  if (!auth.isAuthenticated && !auth.loading) {
    await auth.fetchCurrentUser()
  }

  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    return { name: 'Login', query: { redirect: to.fullPath } }
  }

  if (to.meta.requiresAdmin && !auth.isAdmin) {
    return { name: 'Home' } // authenticated but wrong role we go back to home
  }

  if (to.meta.guestOnly && auth.isAuthenticated) {
    return { name: 'Home' }
  }
})

export default router
