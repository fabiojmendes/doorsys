import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/users'
    },
    {
      path: '/users',
      component: () => import('../views/UserListView.vue')
    },
    {
      path: '/users/:id',
      component: () => import('../views/UserView.vue')
    },
    {
      path: '/about',
      component: () => import('../views/AboutView.vue')
    }
  ]
})

export default router
