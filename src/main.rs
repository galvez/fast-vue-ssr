// Leaving this here because I have a feeling it'll be needed soon
#[macro_use]
extern crate lazy_static;

mod renderer;
mod vue;

use renderer::RendererPool;
use std::io;
use std::sync::Arc;
use std::sync::Mutex;
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
        header::{
            HeaderName,
        },
    },
};

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let pool = Arc::new(Mutex::new(RendererPool::new(64)));
    let renderer = full()
        .and(header::headers_cloned())
        .map(move |path: FullPath, headers: HeaderMap| {
        println!("GET {}", path.as_str());
        println!("Headers: {}", headers.len()); //keys().collect::<Vec<&HeaderName>>());
        let renderer = Arc::clone(&pool);
        let s = path.as_str().to_string();
        let result = renderer.lock().unwrap().render(s);
        result
    });

    let routes = full()
        .and(renderer)
        .map(|_, result| {
            reply::html(result)
        });
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// https://docs.rs/surf/1.0.3/surf/
// https://docs.rs/redis/0.16.0/redis/
