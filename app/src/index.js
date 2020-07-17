import Foobar from './Foobar.vue'

Vue.component('Foobar', Foobar)

Vue.mixin({
  beforeCreate () {
    this.$req = this.$root.$options.$req
  }
})

let warpReq = {}

const appSettings = {
  template: `<div><Foobar>{{ $req.url }}</Foobar></div>`,
  data: {
    msg: 'hello'
  },
  get $req () {
    return warpReq
  },
}

var app = new Vue(appSettings)
