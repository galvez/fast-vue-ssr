import Foobar from './Foobar.vue'

Vue.component('Foobar', Foobar)

Vue.mixin({
  beforeCreate () {
    this.$req = this.$root.$options.req
  }
})

var app = new Vue({
  template: `<div><Foobar>{{ $req.url }}</Foobar></div>`,
  data: {
    msg: 'hello'
  },
  req: {
    url: '/dummy',
  },
})