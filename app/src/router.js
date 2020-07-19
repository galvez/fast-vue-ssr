export default Router => new Router({
  mode: 'history',
  routes: [
    {
      path: '/',
      component: () => import('./pages/home.vue'),
    },
    {
      path: '/other',
      component: () => import('./pages/other.vue'),
    },
  ],
})
