import Vue from 'vue'
import Router from 'vue-router'

import getRouter from './router'
import createApp from './app'

Vue.use(Router)

window.$ssrContext = {
  data: window.__ASYNC_DATA__,
  req: {
  },
}

const { app, router } = createApp(
  window.$ssrContext,
  Vue,
  getRouter(Router),
)

router.onReady(() => {
  app.$mount('#app')
})