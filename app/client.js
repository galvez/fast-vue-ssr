import Vue from 'vue'
import Router from 'vue-router'

import getRouter from './router'
import createApp from './app'

Vue.use(Router)

const { app, router } = createApp(
  {
    req: {},
  },
  Vue,
  getRouter(Router),
)

router.onReady(() => app.$mount('#app'))
