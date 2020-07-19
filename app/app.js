import app from './app.vue'
import getRouter from './router'

function getApp (ssrContext, router) {
  return {
    router,
    render (h) {
      return h(app)
    },
    get $ssrContext () {
      return {
        req: ssrContext.req
      }
    },
  }
}

export default (ssrContext, Vue, router) => {
  return {
    router,
    app: new Vue(getApp(ssrContext, router)),
  }
}
