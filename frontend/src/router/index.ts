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
    path: '/users/new',
    name: 'New User',
    component: () => import('@/views/users/UpsertView.vue'),
    meta: { requiresAuth: true, requiresAdmin: true },
  },
  {
    path: '/users/:id/edit',
    name: 'Update User',
    component: () => import('@/views/users/UpsertView.vue'),
    meta: { requiresAuth: true, requiresSelfOrAdmin: true },
  },
  {
    path: '/members',
    name: 'Members',
    component: () => import('@/views/members/ListView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/members/new',
    name: 'New Member',
    component: () => import('@/views/members/UpsertView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/members/:id/edit',
    name: 'Update Member',
    component: () => import('@/views/members/UpsertView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/interests',
    name: 'Interests',
    component: () => import('@/views/interests/ListView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/interests/new',
    name: 'New Interest',
    component: () => import('@/views/interests/UpsertView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/interests/:id/edit',
    name: 'Update Interest',
    component: () => import('@/views/interests/UpsertView.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/tools',
    name: 'Tools',
    component: () => import('@/views/ToolsView.vue'),
    meta: { requiresAuth: true },
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach(async (to) => {
  const auth = useAuthStore()
  await auth.init()

  if (to.meta.requiresAuth && !auth.isAuthenticated) {
    return { name: 'Login', query: { redirect: to.fullPath } }
  }

  if (to.meta.requiresAdmin && !auth.isAdmin) {
    return { name: 'Home' } // authenticated but wrong role we go back to home
  }

  if (to.meta.requiresSelfOrAdmin) {
    const targetId = to.params.id as string
    const isSelf = auth.user?.id === targetId
    if (!auth.isAdmin && !isSelf) {
      return { name: 'Home' }
    }
  }

  if (to.meta.guestOnly && auth.isAuthenticated) {
    return { name: 'Home' }
  }
})

export default router
