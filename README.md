# Fast Vue SSR with Rust and QuickJS


```rust
use std::io;
use std::fs::read_to_string;
use quick_js::Context;

static RENDERER: &'static str = "./src/renderer.js";
static RENDER: &'static str = "./src/render.js";
static BUNDLE: &'static str = "./app/bundle.js";

pub fn main() -> io::Result<()> {
    let context = Context::new().unwrap();
  let renderer = read_to_string(RENDERER)?;
  let render = read_to_string(RENDER)?;
  let bundle = read_to_string(BUNDLE)?;
    let _loaded_renderer = context.eval(&renderer).unwrap();
    let _loaded_bundle = context.eval(&bundle).unwrap();
    let result = context.eval(&render).unwrap();
    println!("{:?}", result);
    Ok(())
}
```

An ongoing experiment using [Rust][rust] and [QuickJS][quickjs] to server-side render Vue.js applications.

[rust]: https://www.rust-lang.org/
[quickjs]: https://bellard.org/quickjs/

Inspired by [Xinjiang Shao's experiment][https://github.com/soleo/quickjs-docker].

# TODO

- [ ] Add build command (façade to rollup)
- [ ] Add command-line argument to set bundle path
- [ ] Serve /* and SSR Vue bundle with Warp
