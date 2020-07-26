#[macro_use]
extern crate lazy_static;

// exposes renderVueComponentToString()
// See https://ssr.vuejs.org/guide/non-node.html for details
mod jslib;
mod renderer;
mod data;

use std::collections::HashMap;
use renderer::{
    pool::RendererPool,
};
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
        method::Method,
        HeaderMap,
        header::{
            HeaderName,
            HeaderValue,
        },
    },
};

use crate::renderer::request::RendererRequest;
use crate::data::get_home_data;

pub static IMPORT: &'static str = r###"
<script type="module" src="./static/client.js"></script>
"###;

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let dist = warp::path("static")
        .and(warp::fs::dir("./dist/"));

    let r_pool = Arc::new(Mutex::new(RendererPool::new(64)));

    // Run async data fetch if path === '/'
    let home_data = warp::path::end()
        .and_then(get_home_data);

    let renderer = full()
        .and(warp::method())
        .and(header::headers_cloned())
        .and(home_data)
        .map(move |
            path: FullPath,
            method: Method,
            headers: HeaderMap,
            async_data: String,
        | {
            println!("{} {}", method, path.as_str());
            let headers = format!(
                "{:?}",
                headers
                    .iter()
                    .collect::<HashMap<&HeaderName, &HeaderValue>>()
            );
            let renderer = Arc::clone(&r_pool);
            let url = path.as_str().to_string();
            let ssr_request = RendererRequest::new(
                url,
                method.to_string(),
                headers,
                async_data.to_owned(),
            );
            let result = renderer.lock()
                .unwrap()
                .render(ssr_request);
            (result, async_data)
        })
        .map(|(result, async_data)| {
            let async_data = format!(
                "<script>window.__ASYNC_DATA__ = '{}';</script>",
                async_data
            );
            reply::html(
                format!("{}{}{}", async_data, IMPORT, result)
            )
        });

    let routes = dist.or(renderer);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// https://docs.rs/surf/1.0.3/surf/
// https://docs.rs/redis/0.16.0/redis/
