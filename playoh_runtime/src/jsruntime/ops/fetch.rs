use core::cell::RefCell;
use deno_core::{error::AnyError, op, serde::Deserialize, ByteString, OpState, ZeroCopyBuf};
use reqwest::{
    header::{HeaderName, HeaderValue},
    Method, Url,
};
use serde::Serialize;
use std::{rc::Rc, str::FromStr};

#[derive(Deserialize)]
pub struct FetchArgs {
    method: String,
    url: String,
    headers: Vec<(ByteString, ByteString)>,
    body: Option<ZeroCopyBuf>,
}
#[derive(Serialize)]
pub struct FetchResponse {
    status: u16,
    body: Option<ZeroCopyBuf>,
    status_text: String,
    headers: Vec<(ByteString, ByteString)>,
}

#[op]
async fn op_fetch(state: Rc<RefCell<OpState>>, args: FetchArgs) -> Result<FetchResponse, AnyError> {
    let state_ref = state.borrow();
    let client = state_ref.borrow::<reqwest::Client>().clone();
    let method = Method::from_str(&args.method.to_ascii_uppercase())?;
    let url = Url::parse(&args.url)?;
    let mut req = client.request(method, url);
    for (k, v) in args.headers {
        req = req.header(HeaderName::from_bytes(&k)?, HeaderValue::from_bytes(&v)?);
    }
    let req = if let Some(body) = args.body {
        req.body(Vec::from(&*body))
    } else {
        req
    };
    let res = req.send().await?;
    let status = res.status().as_u16();
    let status_text = res.status().canonical_reason().unwrap_or("").to_string();
    let headers = res
        .headers()
        .iter()
        .map(|(k, v)| (k.as_str().into(), v.as_bytes().to_vec().into()))
        .collect();
    let body = res.bytes().await?;
    let body = if body.is_empty() {
        None
    } else {
        Some(body.to_vec().into())
    };
    Ok(FetchResponse {
        status,
        body,
        status_text,
        headers,
    })
}

#[op]
fn op_decode_utf8(buf: ZeroCopyBuf) -> Result<String, AnyError> {
    Ok(String::from_utf8_lossy(&*buf).into())
}
