use serde_json::{json,Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{anyhow, Result, Error};
use wasmbus_rpc::actor::prelude::Context;
use doi_interface::{DoiRequest,DoiNode};

#[derive(Serialize, Deserialize)]
pub struct SetupRequest {
    name: String,
    ts: u64
}

#[derive(Serialize, Deserialize)]
pub struct FolderUpdateRequest {
    #[serde(default)]
    id: String,

    #[serde(default)]
    parent: String,
    
    title: String,

    #[serde(default)]
    public: bool
}

pub mod store;

pub async fn setup(ctx: &Context, req: SetupRequest) -> Result<String> {
    store::new_uid(ctx, req.name, req.ts).await
}

// CURD folders
pub async fn folders_create(ctx: &Context, uid: String, req: FolderUpdateRequest) -> Result<store::Folder> {
    store::folders_c(ctx, uid, req.id, req.parent, req.title, req.public).await
}
pub async fn folders_update(ctx: &Context, uid: String, path: String, req: FolderUpdateRequest) -> Result<store::Folder> {
    store::folders_u(ctx, uid, path, req.parent, req.title, req.public).await
}
pub async fn folders_read_all(ctx: &Context, uid: String) -> Result<HashMap<String,store::Folder>> {
    store::folders_r_a(ctx, uid).await
}
pub async fn folders_delete(ctx: &Context, uid: String, path: String) -> Result<bool> {
    store::folders_d(ctx, uid, path).await
}
pub async fn folders_read_public(ctx: &Context, path: String) -> Result<store::Folder> {
    store::folders_r_p(ctx, path).await
}
pub async fn doi_read(ctx: &Context, uid: String, doi: String) -> Result<String> {
    store::doi_r(ctx, uid, doi.replace("%2F", "/")).await
}

// debug only
pub async fn folder_counter(ctx: &Context, uid: String) -> Result<String> {
    store::counter(ctx, uid, "f").await
}

// // debug only
// pub async fn doi_counter(ctx: &Context, uid: String) -> Result<String> {
//     store::counter(ctx, uid, "d").await
// }

// C of DOI, by upload
pub async fn doi_create(ctx: &Context, team: String, by_user: Option<String>, req: &Value) -> Result<String> {
    store::doi_c(ctx, team, by_user, &req).await
}

// // C_R_ DOI by fetch
// pub async fn doi_create(ctx: &Context, req: &DoiRequest) -> Result<bool> {
//     store::doi_c(ctx, req).await
// }
// pub async fn doi_fetch(ctx: &Context, req: DoiRequest) -> Result<Value> {
//     store::doi_f(ctx, req).await
// }

// debug and data injection features
// pub async fn doi_print_text(ctx: &Context, req: DoiRequest) -> Result<String> {
//     store::doi_pt(ctx, req, doi).await
// }
// pub async fn doi_inject_2(ctx: &Context, req: &DoiRequest, data: Vec<u8>) -> Result<store::DoiNode> {
//     store::doi_ij_2(ctx, req, data).await
// }
pub async fn print_index(ctx: &Context, uid: String) -> Result<Vec<String>> {
    store::print_index(ctx, uid).await
}

pub async fn print_counter(ctx: &Context, uid: String) -> Result<String> {
    store::print_counter(ctx, uid).await
}
pub async fn debug(ctx: &Context, path: String) -> Result<String> {
    store::debug(ctx, path).await
}