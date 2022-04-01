use anyhow::Result;

use bytes::BufMut;
use futures::StreamExt;

use reqsign::services::azure::storage::Signer;
use std::env;
use xml_01::parse_xml;
#[tokio::main]
async fn main() -> Result<()> {
    dotenv::from_filename(".env").ok();
    let account_name = &env::var("OPENDAL_AZBLOB_ACCOUNT_NAME").unwrap();
    let account_key = &env::var("OPENDAL_AZBLOB_ACCOUNT_KEY").unwrap();
    let signer = get_sign(account_name, account_key).await?;
    execute(signer).await?;
    Ok(())
}

async fn execute(signer: Signer) -> Result<()> {
    let mut url = "https://d2lark.blob.core.windows.net/myazurebucket?restype=container&comp=list"
        .to_string();

    let path = "dir1/";
    if !path.is_empty() {
        url.push_str(&format!("&prefix={}", path))
    }

    let mut req = hyper::Request::get(url)
        .body(hyper::Body::empty())
        .expect("must be valid request");

    signer.sign(&mut req).await.expect("sign must success");

    let client = hyper::Client::builder().build(hyper_tls::HttpsConnector::new());

    let mut resp = client
        .request(req)
        .await
        .map_err(|e| anyhow::Error::from(e))?;

    let body = resp.body_mut();
    let mut bs = bytes::BytesMut::new();
    while let Some(b) = body.next().await {
        let b = b.map_err(|e| anyhow::Error::from(e))?;
        bs.put_slice(&b)
    }

    let bs = bs.freeze();
    println!("{:?}", bs);
    let out = parse_xml(bs);
    println!("{:?}", out);
    Ok(())
}

async fn get_sign(account_name: &str, account_key: &str) -> Result<Signer> {
    let mut signer_builder = Signer::builder();
    signer_builder
        .account_name(&account_name)
        .account_key(&account_key);

    signer_builder.build().await
}
