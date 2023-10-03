import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/customers'
    },
    {
      path: '/customers',
      component: () => import('@/views/CustomerListView.vue')
    },
    {
      path: '/customers/:id',
      component: () => import('@/views/CustomerView.vue')
    },
    {
      path: '/logs',
      component: () => import('@/views/EntryLogView.vue')
    }
  ]
})

export default router
