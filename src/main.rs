// Leaving this here because I have a feeling it'll be needed soon
#[macro_use]
extern crate lazy_static;

mod renderer;
mod vue;

use renderer::RendererPool;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;
use warp::{self, filters::path::FullPath, reply, Filter};

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let pool = Arc::new(Mutex::new(RendererPool::new(64)));
    let renderer = warp::path::full().map(move |path: FullPath| {
        let renderer = Arc::clone(&pool);
        let s = path.as_str().to_string();
        let result = renderer.lock().unwrap().render(s);
        result
    });

    let routes = warp::path::full()
        .and(renderer)
        .map(|_, result| reply::html(result));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// https://docs.rs/surf/1.0.3/surf/
// https://docs.rs/redis/0.16.0/redis/
