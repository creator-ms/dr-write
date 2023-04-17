use anyhow::{Result};
use cookie::Cookie;
use rust_embed::RustEmbed;
use serde_json::{json, from_slice, Value};
use serde::{Deserialize, Serialize};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver, HeaderMap};
use std::str::FromStr;
use doi_interface::{DoiRequest};

#[derive(RustEmbed)]
#[folder = "./asset"]
struct Asset;


#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct DrWriteActor {}

mod api;

fn get_from_cookie(headers: &HeaderMap, field: &str) -> Option<String> {
    let emptyString = "".to_string();
    let mut cookies  = Cookie::split_parse(headers.get("cookie")?.first().unwrap_or_else(|| &emptyString));
    
    cookies.find_map(|result| match result {
        Ok(cookie) if cookie.name() == field => Some(cookie.value().to_string()),
        _ => None
    })
}

fn get_uid(headers: &HeaderMap) -> Option<String> {
    return get_from_cookie(headers, &"uid")
}
fn get_name(headers: &HeaderMap) -> Option<String> {
    return get_from_cookie(headers, &"name")
}

// format output and control cookie duration
async fn wrap_result(ctx: &Context, uid: String, json: impl Serialize) -> RpcResult<HttpResponse> {
    let mut header = HeaderMap::new();
    header.insert("Access-Control-Allow-Origin".to_string(), vec!["*".to_string()]);
    header.insert("Access-Control-Allow-Headers".to_string(), vec!["*".to_string()]);
    header.insert("Access-Control-Allow-Methods".to_string(), vec!["GET, OPTIONS, PUT, POST, DELETE".to_string()]);
    header.insert("Set-cookie".to_string(), vec![format!("uid={}; Path=/; Max-Age=34473600",uid).to_string()]);
    HttpResponse::json_with_headers(
        json,
        200,
        header
    )
}
// format output and control cookie duration
async fn wrap_future_result(ctx: &Context, uid: String, jr: Result<impl Serialize>) -> RpcResult<HttpResponse> {
    match jr {
        Ok(json) => wrap_result(ctx, uid, json).await,
        Err(e) => Ok(HttpResponse::bad_request(format!(
            "wfr error: {:?}",
            e
        )))
    }
}

#[derive(Serialize, Deserialize)]
pub struct TestUpdateRequest {
    title: String
}

struct GetAssetResponse {
    found: bool,
    content_type: Option<String>,
    asset: Vec<u8>
}


fn get_asset(
    path: &String,
) -> GetAssetResponse {
    let path = path.to_string();
    let trimmed = if path.trim() == "/" {
        "index.html"
    } else {
        path.trim().trim_start_matches('/')
    };

    if let Some(file) = Asset::get(trimmed) {
        return GetAssetResponse {
            found: true,
            asset: Vec::from(file.data),
            content_type: mime_guess::from_path(trimmed)
                .first()
                .map(|m| m.to_string()),
        };
    }

    return GetAssetResponse {
        found: false,
        content_type: None,
        asset: vec![]
    };
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for DrWriteActor {

    
    /// Returns a greeting, "Hello World", in the response body.
    /// If the request contains a query parameter 'name=NAME', the
    /// response is changed to "Hello NAME"
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        
        if req.method.eq("OPTIONS") {
            let mut header = HeaderMap::new();
            header.insert("Access-Control-Allow-Origin".to_string(), vec!["*".to_string()]);
            header.insert("Access-Control-Allow-Headers".to_string(), vec!["*".to_string()]);
            header.insert("Access-Control-Allow-Methods".to_string(), vec!["GET, OPTIONS, PUT, POST, DELETE".to_string()]);
                
            return Ok(HttpResponse{
                header,
                status_code: 204,
                ..Default::default()
            });
        }


        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        // wrap_result(ctx, "test".to_string(), json!({"test": true})).await
        let some_uid = get_uid(&req.header);
        let some_name = get_name(&req.header);
        
        match some_uid {
            Some(uid) => {
                match (req.method.as_ref(), segments.as_slice()) {
                    //
                    // ("GET", ["api", "doi"]) => {
                    //     let doi = form_urlencoded::parse(req.query_string.as_bytes()).find(|(n, _)| n == "doi").map(|(_, v)| v.to_string()).unwrap();
                    //     match remote::doi_fetch(ctx, doi).await {
                    //         Ok(data) => HttpResponse::json(data, 200),
                    //         Err(_) => Ok(HttpResponse::not_found())
                    //     }
                    // },
                    ("GET", ["api", "app"]) => wrap_result(ctx, uid.clone(), json!({"ok":true,"uid":uid.clone()})).await,
                    ("GET", ["api", "app","counter"]) => 
                        wrap_future_result(ctx, uid.clone(), api::print_counter(ctx, uid).await).await,
                    ("POST", ["api", "app"]) =>  match serde_json::from_slice::<TestUpdateRequest>(&req.body) {
                        Ok(r) => HttpResponse::json(r, 200),
                        Err(e) => Ok(HttpResponse::bad_request(format!(
                            "malformed body: {:?}",
                            e
                        ))),
                    },
                    ("GET", ["api", "app","folders"]) => 
                        wrap_future_result(ctx, uid.clone(), api::print_index(ctx, uid).await).await,
                    //end test debug
                    ("GET", ["api","folders"|"folder","next"]) => 
                    wrap_future_result(ctx, uid.clone(), api::folder_counter(ctx, uid).await).await,

                    // ("GET", ["api","dois"|"doi","next"]) => 
                    // wrap_future_result(ctx, uid.clone(), api::doi_counter(ctx, uid).await).await,

                    // start FOLDERS
                    //C
                    ("POST", ["api"|"v1"|"v2"|"v3","folders"]) =>  match from_slice(&req.body) { 
                        Ok(req) => wrap_future_result(ctx, uid.clone(), api::folders_create(ctx, uid, req).await).await,
                        Err(e) => Ok(HttpResponse::bad_request(format!(
                            "input error: {:?}",
                            e
                        )))
                    },
                    //U
                    ("POST", ["api"|"v1"|"v2"|"v3","folders", path]) =>  match from_slice(&req.body) { 
                        Ok(req) => wrap_future_result(ctx, uid.clone(), api::folders_update(ctx, uid, path.to_string(), req).await).await,
                        Err(e) => Ok(HttpResponse::bad_request(format!(
                            "input error: {:?}",
                            e
                        )))
                    },
                    //R - all
                    ("GET", ["api"|"v1"|"v2"|"v3","folders"]) => 
                        wrap_future_result(ctx, uid.clone(), api::folders_read_all(ctx, uid).await).await,
                        
                    //D
                    ("DELETE", ["api"|"v1"|"v2"|"v3","folders", path]) =>  wrap_future_result(ctx, uid.clone(), api::folders_delete(ctx, uid, path.to_string()).await).await,
                    

                    //// DOI fetch and create
                    ("POST", ["api"|"v1"|"v2"|"v3","doi"]) => match from_slice::<Value>(&req.body) { 
                        Ok(r) => {
                            wrap_future_result(ctx, uid.clone(), api::doi_create(ctx, uid.clone(), some_name.clone(), &r).await).await
                        },
                        Err(e) => Ok(HttpResponse::bad_request(format!(
                            "input error: {:?}",
                            e
                        )))
                    },

                    ("GET", ["api"|"v1"|"v2"|"v3","doi",_]) => { 
                        let doi = form_urlencoded::parse(req.query_string.as_bytes()).find(|(n, _)| n == "doi").map(|(_, v)| v.to_string()).unwrap();
                        match api::doi_read(ctx, uid.clone(), doi).await {
                            Ok(n) => wrap_result(ctx, uid.clone(), n).await,
                            Err(e) => Ok(HttpResponse::bad_request(format!(
                                "fetch error: {:?}",
                                e
                            )))
                        }
                    },

                    // //// DOI refetch
                    // ("POST", ["ref","doi"]) => match from_slice(&req.body) { 
                    //     Ok(r) => wrap_future_result(ctx, uid.clone(), api::doi_fetch(ctx, r).await).await,
                    //     Err(e) => Ok(HttpResponse::bad_request(format!(
                    //         "input error: {:?}",
                    //         e
                    //     )))
                    // },
                    //// DOI inject

                    // ("GET", ["dpt",folder]) => { 
                    //     let doi = form_urlencoded::parse(req.query_string.as_bytes()).find(|(n, _)| n == "doi").map(|(_, v)| v.to_string()).unwrap();
                    //     wrap_future_result(ctx, uid.clone(), api::doi_print_text(ctx, uid.clone(), folder.to_string(), doi).await).await
                    // },

                    // ("GET", ["j2","doi", folder]) => {
                    //     let resp = get_asset(&"j2.txt".to_string());
                    //     if !resp.found {
                    //         Ok(HttpResponse::not_found())
                    //     } else {
                    //         let r = DoiRequest{
                    //             doi:"".to_string(),
                    //             uid:uid.clone(),
                    //             user:None,
                    //             folder: Some(folder.clone()),
                    //         };
                    //         wrap_future_result(ctx, uid.clone(), api::doi_inject_2(ctx, r, resp.asset).await).await
                    //     }
                    // },


                    //R - public access, if allowed
                    ("GET", ["pub","folders", path]) =>  wrap_future_result(ctx, uid.clone(), api::folders_read_public(ctx, path.to_string()).await).await,
                    
                    ("GET", _) => {
                        let resp = get_asset(&req.path);
                        if !resp.found {
                            Ok(HttpResponse::not_found())
                        } else {
                            let mut header = HeaderMap::new();
                            if let Some(content_type) = resp.content_type {
                                header.insert("Content-Type".to_string(), vec![content_type]);
                            }
                            Ok(HttpResponse {
                                status_code: 200,
                                header,
                                body: resp.asset,
                            })
                        }
                    },
                    (_, _) => Ok(HttpResponse::not_found())
                }
            },
            None => {
                match (req.method.as_ref(), segments.as_slice()) {

                    ("GET", ["pub","folders", path]) =>  match api::folders_read_public(ctx, path.to_string()).await {
                        Ok(f) => HttpResponse::json(
                            f,
                            200
                        ),
                        Err(_) => Ok(HttpResponse::not_found())
                    },

                    ("GET", ["echo"] | ["api", "echo"]) => Ok(HttpResponse::bad_request("test")),

                    ("POST", ["api","setup"]) => match from_slice(&req.body) {
                        Ok(req) => match api::setup(ctx, req).await {
                            Ok(uid) => wrap_result(ctx, uid.clone(), json!({"ok":true,"uid":uid.clone()})).await,
                            Err(e) => Ok(HttpResponse::bad_request(format!(
                                "setup error: {:?}",
                                e
                            )))
                        },
                        Err(e) => Ok(HttpResponse::bad_request(format!(
                            "malformed body: {:?}",
                            e
                        ))),
                    },

                    ("GET", _) => {
                        let resp = get_asset(&req.path);
                        if !resp.found {
                            Ok(HttpResponse::not_found())
                        } else {
                            let mut header = HeaderMap::new();
                            if let Some(content_type) = resp.content_type {
                                header.insert("Content-Type".to_string(), vec![content_type]);
                            }
                            Ok(HttpResponse {
                                status_code: 200,
                                header,
                                body: resp.asset,
                            })
                        }
                    },
                    (_, _) => Ok(HttpResponse::not_found())
                }
            }
        }
        
    }
}