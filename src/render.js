let error
let result

renderVueComponentToString(app, (err, res) => {
  if (err) {
    error = err.message
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
