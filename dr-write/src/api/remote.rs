use serde::Deserialize;
use serde_json::{json, Value};
use anyhow::{anyhow, Result};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpclient::{HttpClient, HttpClientSender, HttpRequest as CHttpRequest};

pub(crate) async fn doi_fetch(ctx: &Context, uid: String, doi: String) -> Result<Vec<u8>> {

    let url = format!("https://api.crossref.org/works/{}", doi);
    let client = HttpClientSender::new();
    let resp = client
        .request(ctx, &CHttpRequest::get(&url))
        .await
        .map_err(|e| anyhow!(e))?;
    if !(200..300).contains(&resp.status_code) {
        return Err(anyhow!(resp.status_code));
    }
    Ok(resp.body)

    // let info = serde_json::from_slice(&resp.body)
    //     .map_err(|e| anyhow!(e))?;
    
    // Ok(info)
}
