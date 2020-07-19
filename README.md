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

So far using a thread pool and channels to communicate with the Warp route handler. The goal is to get a full Node-like `IncomingMessage` object available as `$ssrContext.req`. It already includes a `/static` handler and serves a code-splitted build on the client via Rollup.

Node outperforms QuickJS by a wide margin. Especially with enough cores and memory. However, QuickJS is very small and has very low memory consumption, so running it threaded in a Rust shell makes it possible to have very high throughput using very few resources in comparison.

Inspired by [Xinjiang Shao's experiment](https://github.com/soleo/quickjs-docker).

## Running

1. [Install Rust](https://www.rust-lang.org/tools/install).
2. `npm install`
3. `npm test`

Or `npm run build` for generating the Rust binary.
