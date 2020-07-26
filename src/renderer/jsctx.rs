pub static INJECT_SSR_CONTEXT: &'static str = r###"
Vue.mixin({
  beforeCreate () {
    this.$ssrContext = this.$root.$options.$ssrContext
  }
})

let $ssrContext = {
  req: {
  }
}

true
"###;

pub static WAIT_ROUTER_READY: &'static str = r###"
router.push($ssrContext.req.url)

new Promise((resolve) => {
    router.onReady(() => {
        resolve(true)
    })
})
"###;

pub static RENDER_VUE_COMPONENT: &'static str = r###"
renderVueComponentToString(app, (err, res) => {
  if (err) {
    error = err.toString()
  }
  result = res
})

if (error) {
  // https://docs.rs/quick-js/0.2.0/quick_js/
  // JS objects can not be deserialized into Rust (JsValue::Object) 
  // due to a missing property enumeration API (will be fixed soon)
  "error:" + error
} else {
  result
}
"###;
