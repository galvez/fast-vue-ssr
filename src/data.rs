use std::boxed::Box;
use std::error::Error;
use std::marker::Send;
use std::marker::Sync;

use serde::{Deserialize, Serialize};
use warp::Rejection;

#[derive(Deserialize, Serialize)]
struct Ip {
    ip: String
}

type HttpCallError = dyn Error + Send + Sync + 'static;

pub async fn get_home_data() -> Result<String, Rejection>  {
    let result = sample_http_call().await;
    let data = result.unwrap();
    // println!("get_home_data result {:?}", &data);
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
