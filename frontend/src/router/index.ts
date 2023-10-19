import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import CryptoView from '@/views/CryptoView.vue'
import RepositoryView from '@/views/RepositoryView.vue';
import StatisticsView from '@/views/StatisticsView.vue';
import SearchRepositoriesView from '@/views/SearchRepositoriesView.vue';

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/crypto/:id',
      name: 'crypto',
      component: CryptoView
    },
    {
      path: '/repository/:id',
      name: 'repository',
      component: RepositoryView
    },
    {
      path: '/statistics',
      name: 'statistics',
      component: StatisticsView
    },
    {
      path: '/repository',
      name: 'repositories',
      component: SearchRepositoriesView
    }
  ]
})

export default router
