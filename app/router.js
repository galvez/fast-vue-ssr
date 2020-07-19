import Home from './pages/home.vue'
import Other from './pages/other.vue'
export default Router => new Router({
  mode: 'history',
  routes: [
    {
      path: '/',
      component: Home,
    },
    {
      path: '/other',
      component: Other,
    },
  ],
})
