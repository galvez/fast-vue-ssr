import Foobar from './Foobar.vue'

Vue.component('Foobar', Foobar)

var app = new Vue({
  template: `<div><Foobar>{{ msg }}</Foobar></div>`,
  data: {
    msg: 'hello'
  }
})
