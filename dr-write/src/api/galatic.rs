use serde::Deserialize;
use serde_json::{json, Value};
use anyhow::{anyhow, Result};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_messaging::*;

pub(crate) async fn doi_fetch(ctx: &Context, uid: String, doi: String) -> Result<bool> {

    MessagingSender::new()
    .publish(
        ctx,
        &PubMessage {
            body: format!("doi-{}-{}", uid, doi).as_bytes().to_vec(),
            reply_to: None,
            subject: "app.drwrite.fetch".to_string(),
        },
    )
    .await?;

    Ok(true)
}
