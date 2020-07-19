#[macro_use]
extern crate lazy_static;

// exposes renderVueComponentToString()
// See https://ssr.vuejs.org/guide/non-node.html for details
mod vue;
mod renderer;

use renderer::RendererPool;
use std::io;
use std::sync::{
    Arc,
    Mutex
};
use warp::{
    self,
    header,
    filters::path::{
        full,
        FullPath,
    },
    reply,
    Filter,
    http::{
        HeaderMap,
        // header::{
        //     HeaderName,
        // },
    },
};

pub static IMPORT: &'static str = r###"
<script type="module" src="./static/client.js"></script>
"###;

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let dist = warp::path("static")
        .and(warp::fs::dir("./dist/"));

    let r_pool = Arc::new(Mutex::new(RendererPool::new(64)));

    let renderer = full()
        .and(header::headers_cloned())
        .map(move |path: FullPath, headers: HeaderMap| {
            println!("GET {}", path.as_str());
            println!("Headers: {}", headers.len()); //keys().collect::<Vec<&HeaderName>>());
            let renderer = Arc::clone(&r_pool);
            let s = path.as_str().to_string();
            let result = renderer.lock().unwrap().render(s);
            result
        })
        .map(|result| {
            println!("{}", result);            
            reply::html(
                format!("{}{}", IMPORT, result)
            )
        });

    let routes = dist.or(renderer);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// https://docs.rs/surf/1.0.3/surf/
// https://docs.rs/redis/0.16.0/redis/
