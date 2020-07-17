# Fast Vue SSR with Rust and QuickJS

An ongoing experiment using [Rust][rust], [Warp][warp] and [QuickJS][quickjs] to server-side render Vue.js applications.


```rust
use renderer::RendererPool;
#[tokio::main]
pub async fn main() -> io::Result<()> {
    let pool = Arc::new(Mutex::new(RendererPool::new(64)));
    let renderer = warp::path::full().map(move |path: FullPath| {
        let renderer = Arc::clone(&pool);
        let s = path.as_str().to_string();
        // Currently only passing path to renderer is possible
        // Full Request object is a WIP
        let result = renderer.lock().unwrap().render(s);
        result
    });
    let routes = warp::path::full()
        .and(renderer)
        .map(|_, result| reply::html(result));
```

[rust]: https://www.rust-lang.org/
[quickjs]: https://bellard.org/quickjs/
[warp]: https://github.com/seanmonstar/warp

Inspired by [Xinjiang Shao's experiment](https://github.com/soleo/quickjs-docker).

# TODO

- [ ] Add build command (façade to rollup)
- [ ] Add command-line argument to set bundle path
- [ ] Serve /* and SSR Vue bundle with Warp
