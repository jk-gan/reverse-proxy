use anyhow::*;
use hyper::Client;

#[tokio::main]
async fn main() -> Result<()> {
    let https = hyper_rustls::HttpsConnector::with_native_roots();
    let client: Client<_, hyper::Body> = Client::builder().build(https);

    let url = "https://httpbin.org/json".parse().context("Parsing URL")?;
    let res = client.get(url).await.context("Performing HTTP request")?;
    println!("{:?}", res);

    let body = res.body();
    println!("body: {:?}", body);

    Ok(())
}
