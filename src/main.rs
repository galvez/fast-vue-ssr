#[macro_use]
extern crate lazy_static;

// exposes renderVueComponentToString()
// See https://ssr.vuejs.org/guide/non-node.html for details
mod jslib;
mod renderer;
mod request;

use surf;
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
    Rejection,
    Filter,
    http::{
        HeaderMap,
        // header::{
        //     HeaderName,
        // },
    },
};

use std::boxed::Box;
use std::error::Error;
use std::marker::Send;
use std::marker::Sync;
use serde::{Deserialize, Serialize};
use crate::request::RendererRequest;

pub static IMPORT: &'static str = r###"
<script type="module" src="./static/client.js"></script>
"###;


#[derive(Deserialize, Serialize)]
struct Ip {
    ip: String
}

type HttpCallError = dyn Error + Send + Sync + 'static;

pub async fn get_home_data() -> Result<String, Rejection>  {
    let result = sample_http_call().await;
    let data = result.unwrap();
    println!("get_home_data result {:?}", &data);
    Ok(data)
}

async fn sample_http_call() -> Result<String, Box<HttpCallError>> {
    let uri = "https://httpbin.org/post";
    let data = &Ip { ip: "129.0.0.1".into() };
    let res = surf::post(uri).body_json(data)?.await?;
    assert_eq!(res.status(), 200);

    let uri = "https://api.ipify.org?format=json";
    let Ip { ip } = surf::get(uri).recv_json().await?;
    assert!(ip.len() > 10);
    Ok(ip)
}

#[tokio::main]
pub async fn main() -> io::Result<()> {
    let dist = warp::path("static")
        .and(warp::fs::dir("./dist/"));

    let r_pool = Arc::new(Mutex::new(RendererPool::new(64)));

    let home_data = warp::path::end()
        .and_then(get_home_data);

    let renderer = full()
        .and(header::headers_cloned())
        .and(home_data)
        .map(move |path: FullPath, headers: HeaderMap, data| {
            println!("data: {}", data);
            println!("GET {}", path.as_str());
            println!("Headers: {}", headers.len()); //keys().collect::<Vec<&HeaderName>>());
            let renderer = Arc::clone(&r_pool);
            let url = path.as_str().to_string();
            let ssr_request = RendererRequest::new(url);
            let result = renderer.lock()
                .unwrap()
                .render(ssr_request);
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
