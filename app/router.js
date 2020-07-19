
export default Router => new Router({
  mode: 'history',
  routes: [
    {
      path: '/',
      component: () => import('./pages/home.vue').then(m => m.default),
    },
    {
      path: '/other',
      component: () => import('./pages/other.vue').then(m => m.default),
    },
  ],
})
