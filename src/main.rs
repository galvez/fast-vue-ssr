
// Leaving this here because I have a feeling it'll be needed soon
// #[macro_use] extern crate lazy_static;

mod renderer;

use std::fs::read_to_string;
use std::io;
use std::sync::mpsc;
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::sync::Mutex;
use warp::{
    self,
    filters,
    filters::BoxedFilter,
    reply,
    Filter,
    Rejection,
    Reply
};
use quick_js::Context;
use renderer::RendererPool;

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let pool = Arc::new(Mutex::new(RendererPool::new(64)));
    let renderer = warp::path::full().map(move |path: filters::path::FullPath| {
        let _pool = Arc::clone(&pool);
        let s = path.as_str().to_string();
        let result = _pool.lock().unwrap().execute(s);
        result
    });

    let routes = warp::path::full()
        .and(renderer)
        .map(|path, result| format!("Getting path: {:?}!\nGot result: {:?}!", path, result));
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// https://docs.rs/surf/1.0.3/surf/
// https://docs.rs/redis/0.16.0/redis/
