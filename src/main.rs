//use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use warp::http::HeaderValue as WarpHeaderValue;
use warp::Filter;

#[tokio::main]
async fn main() {

    let proxy = warp::path("proxy")
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(|params: std::collections::HashMap<String, String>| {


            let request_url = params.get("url").unwrap();

            let response = tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(take_request(request_url.to_string()))
            });

            warp::reply::with_header(
                warp::reply::with_status(warp::reply::json(&response), warp::http::StatusCode::OK),
                "Access-Control-Allow-Origin",
                WarpHeaderValue::from_static("*"),
            )
        });

    println!("Starting the server...");
    warp::serve(proxy).run(([0, 0, 0, 0], 3030)).await;

    println!("Server Stopped!");

}

async fn take_request(url: String) -> serde_json::Value {
    let response_text = reqwest::get(url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    serde_json::from_str(&response_text).unwrap()
}
