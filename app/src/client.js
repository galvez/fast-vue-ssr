import Vue from 'vue'
import Router from 'vue-router'
import createApp from './app'

Vue.use(Router)

const { app, router } = createApp(Router)

router.onReady(() => app.$mount('#app'))
