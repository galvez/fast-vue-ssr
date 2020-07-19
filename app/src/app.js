import getRouter from './router'
import defaultLayout from './layouts/default.vue'

Vue.mixin({
  beforeCreate () {
    this.$ssrContext = this.$root.$options.$ssrContext
  }
})

let $ssrContext = {
  req: {
  }
}

const appSettings = router => ({
  router,
  render (h) {
    return h(defaultLayout)
  },
  get $ssrContext () {
    return {
      req: $ssrContext.req
    }
  },
})

export default Router => {
  const router = getRouter(Router)
  return {
    router,
    app: new Vue(appSettings(router)),
  }
}
