import getRouter from './router'
import createApp from './app'

Vue.use(VueRouter)

const { app, router } = createApp(
  $ssrContext,
  Vue,
  getRouter(VueRouter),
)
