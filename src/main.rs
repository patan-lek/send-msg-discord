use std::env;
use std::collections::HashMap;
use http::header::HeaderMap;

#[tokio::main]
async fn main() {
    smd().await.map_err(|err| println!("{:?}", err));
}

//#[tokio::main]
async fn smd() -> Result<(), std::io::Error> {
    dotenv::dotenv().ok();

    let _url_key = env::var("DISCORD_URL_KEY").unwrap();
    let _auth_key = env::var("DISCORD_AUTH_KEY").unwrap();

    let mut msg = HashMap::new();
    let mut headers = HeaderMap::new();

    //Your message that you want to send
    msg.insert("content", "");
    headers.insert("authorization", _auth_key.parse().unwrap());

    let client = reqwest::Client::new();
    client.post(_url_key)
        .json(&msg)
        .headers(headers)
        .send().await;
        return Ok(());
}
