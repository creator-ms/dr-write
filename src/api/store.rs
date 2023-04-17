use serde_json::{json,Value,Map};
use futures::{future};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use anyhow::{anyhow, Result, Error};

// use wasmcloud_interface_blobstore::{Blobstore, BlobstoreSender, Chunk, PutObjectRequest};
use wasmcloud_interface_keyvalue::{
    IncrementRequest, KeyValue, KeyValueSender, SetRequest
};
// use wasmbus_rpc::common::Context;
use wasmbus_rpc::actor::prelude::Context;
use wasmcloud_interface_httpserver::{
    HttpRequest, HttpResponse, HeaderMap
};

#[path = "./remote.rs"]
mod remote;

#[derive(Serialize, Deserialize)]
pub struct Name {
    suffix: Option<String>,
    given: Option<String>,
    family: Option<String>,
    prefix: Option<String>,
    name: Option<String>
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    url: String,
    ctype: String,
    cversion: String,
    app: String
}

#[derive(Serialize, Deserialize)]
pub struct Folder {
    path: String,
    owner: String,
    title: String,
    #[serde(default)]
    nodes: HashMap<String, String>,
    #[serde(default)]
    public: bool
}

#[derive(Serialize, Deserialize)]
pub struct DoiNode {
    folder: String,
    doi: Option<String>,
    publisher: Option<String>,
    pub_year: Option<u16>,
    pub_month: Option<u8>,
    pub_day: Option<u8>,
    pol_year: Option<u16>,
    pol_month: Option<u8>,
    pol_day: Option<u8>,
    create_year: Option<u16>,
    create_month: Option<u8>,
    create_day: Option<u8>,
    titles: Vec<String>,
    summary: Option<String>,
    typ: Option<String>, 
    authors: Vec<Name>,
    editors: Vec<Name>,
    links:Vec<Link>,
}


pub(crate) async fn new_uid(ctx: &Context, name: String, ts: u64) -> Result<String> {
    let uid = format!("dr-{}-{}", name, ts);
    Ok(uid)
}


pub(crate) async fn counter(ctx: &Context, uid: String, pfx: &str) -> Result<String> {
    let id = KeyValueSender::new().increment(ctx, &IncrementRequest {
        key: format!("next-{}-{}", pfx, uid).to_string(),
        value: 1,
    }) 
    .await
    .map_err(|e| anyhow!(e))? as u64;

    Ok(format!("{}{}", pfx, id).to_string())
}

// TODO: remove debug point
pub(crate) async fn print_counter(ctx: &Context, uid: String) -> Result<String> {
    let id = KeyValueSender::new().increment(ctx, &IncrementRequest {
        key: format!("next-d-{}", uid).to_string(),
        value: 1,
    }) 
    .await
    .map_err(|e| anyhow!(e))? as u64;

    let id_s = format!("d{}", id);

    Ok(id_s.to_string())

    // let f = Folder{
    //     path: format!("folder-{}-{}", uid, id).to_string(),
    //     title: format!("folder-{}-{}", uid, id).to_string(),
    //     nodes:vec![],
    //     public: false
    // };

    // let s = serde_json::to_string(&f).map_err(|e| anyhow!(e))?;
    // KeyValueSender::new()
    // .set(
    //     ctx,
    //     &SetRequest {
    //         key: f.path.clone(),
    //         value: s.clone(),
    //         expires: 0,
    //     },
    // )
    // .await
    // .map_err(|e| anyhow!(e))?;

    // let set_addr = format!("folders-{}", uid);

    // let set_str = KeyValueSender::new()
    //     .get(ctx, &set_addr)
    //     .await
    //     .map_err(|e| anyhow!(e))?
    //     .value;

    // let mut smap: HashMap<String, String> = match serde_json::from_str(set_str.as_str()) {
    //     Ok(m) => m,
    //     Err(_) => HashMap::new()
    // };

    // smap.insert(id_s, f.path.clone());
    // let set_str = serde_json::to_string(&smap)?;

    // KeyValueSender::new()
    // .set(
    //     ctx,
    //     &SetRequest {
    //         key: set_addr.to_string(),
    //         value: set_str,
    //         expires: 0,
    //     },
    // )
    // .await
    // .map_err(|e| anyhow!(e))?;


    // let set_str = KeyValueSender::new()
    //     .get(ctx, &set_addr)
    //     .await
    //     .map_err(|e| anyhow!(e))?
    //     .value;
    // Ok(set_str)
    
}

pub(crate) async fn print_index(ctx: &Context, uid: String) -> Result<Vec<String>> {

    let urls = KeyValueSender::new()
        .set_query(ctx, &format!("folders-{}", uid))
        .await
        .map_err(|e| anyhow!(e))?;
        
    Ok(urls)
}
pub(crate) async fn debug(ctx: &Context, path: String) -> Result<String> {

    let query = KeyValueSender::new()
        .get(ctx, path.as_str())
        .await
        .map_err(|e| anyhow!(e))?
        .value;
    
    // let json = query.
    Ok(query)
}

pub(crate) async fn folders_c(ctx: &Context, uid: String, path: String, parent: String, title: String, public: bool) -> Result<Folder> {

    let fullpath = if parent.len() < 1 {
        path
    } else {
        format!("{}_{}",parent, path).to_string()
    };

    let f = Folder{
        path: fullpath,
        owner: uid.clone(),
        title: title,
        public: public,
        nodes: HashMap::new()
    };

    let s = serde_json::to_string(&f).map_err(|e| anyhow!(e))?;
    KeyValueSender::new()
    .set(
        ctx,
        &SetRequest {
            key: f.path.clone(),
            value: s.clone(),
            expires: 0,
        },
    )
    .await
    .map_err(|e| anyhow!(e))?;

    let set_addr = format!("folders-{}", uid);

    let set_str = KeyValueSender::new()
        .get(ctx, &set_addr)
        .await
        .map_err(|e| anyhow!(e))?
        .value;

    let mut smap: HashMap<String, String> = match serde_json::from_str(set_str.as_str()) {
        Ok(m) => m,
        Err(_) => HashMap::new()
    };

    smap.insert(f.path.clone(), f.path.clone());
    let set_str = serde_json::to_string(&smap)?;

    KeyValueSender::new()
    .set(
        ctx,
        &SetRequest {
            key: set_addr.to_string(),
            value: set_str,
            expires: 0,
        },
    )
    .await
    .map_err(|e| anyhow!(e))?;

    Ok(f)
}

pub(crate) async fn folders_u(ctx: &Context, uid: String, path: String, parent: String, title: String, public: bool) -> Result<Folder> {
    let f_str = KeyValueSender::new()
        .get(ctx, &path)
        .await
        .map_err(|e| anyhow!(e))?
        .value;
    let mut f: Folder = serde_json::from_str(&f_str)?;

    f.title = title;
    f.public = public;
    KeyValueSender::new()
        .set(
            ctx,
            &SetRequest {
                key: f.path.clone(),
                value: serde_json::to_string(&f)?,
                expires: 0,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?;

    Ok(f)
}

pub(crate) async fn folders_r_a(ctx: &Context, uid: String) -> Result<HashMap<String,Folder>> {
    
    let set_str = KeyValueSender::new()
        .get(ctx, &format!("folders-{}", uid) )
        .await
        .map_err(|e| anyhow!(e))?
        .value;

    let smap: HashMap<String, String> = match serde_json::from_str(set_str.as_str()) {
        Ok(m) => m,
        Err(_) => HashMap::new()
    };
    let mut rmap: HashMap<String, Folder> = HashMap::new();
    
    for path in smap.keys() {
        let f: Folder = folders_r(ctx, uid.clone(), path.clone()).await?;
        if f.owner.eq(&uid) {
            rmap.insert(f.path.clone(), f);
        }
        // result.push(folders_r(ctx, uid.clone(), path).await?)
    }
    Ok(rmap)
}

fn find_key_by_value(map: HashMap<String,String>, v:String) -> Option<String> {
    for (key, value) in &map {
        if value.eq(&v) {
            return Some(key.to_string());
        }
    }
    return None;
}

pub(crate) async fn folders_d(ctx: &Context, uid: String, path: String) -> Result<bool> {
    
    let set_addr = format!("folders-{}", uid);

    let set_str = KeyValueSender::new()
        .get(ctx, &set_addr)
        .await
        .map_err(|e| anyhow!(e))?
        .value;

    let mut smap: HashMap<String, String> = match serde_json::from_str(set_str.as_str()) {
        Ok(m) => m,
        Err(_) => HashMap::new()
    };

    if let Some(key) = find_key_by_value(smap.clone(), path.clone()) {
        smap.remove(&key);

        let set_str = serde_json::to_string(&smap)?;

        KeyValueSender::new()
        .del(ctx, &path)
        .await
        .map_err(|e| anyhow!(e))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

pub(crate) async fn folders_r(ctx: &Context, uid: String, path: String) -> Result<Folder> {
    let f_str = KeyValueSender::new()
        .get(ctx, &path)
        .await
        .map_err(|e| anyhow!(e))?
        .value;
    // let f = serde_json::from_str(&f_str)?;
    Ok(serde_json::from_str(&f_str)?)
}

pub(crate) async fn folders_r_p(ctx: &Context, path: String) -> Result<Folder> {
    let f_str = KeyValueSender::new()
        .get(ctx, &path)
        .await
        .map_err(|e| anyhow!(e))?
        .value;
    Ok(serde_json::from_str(&f_str)?)
}

async fn doi_to_folder(ctx: &Context, uid: String, path: String, doi: String) -> Result<bool> {
    let f_str = KeyValueSender::new()
        .get(ctx, &path)
        .await
        .map_err(|e| anyhow!(e))?
        .value;

    match serde_json::from_str::<Folder>(&f_str) {
        Ok(mut f) => {
            f.nodes.entry(doi.clone()).or_insert_with(|| "pending".to_string());
                    
            KeyValueSender::new()
                .set(
                    ctx,
                    &SetRequest {
                        key: path.clone(),
                        value: serde_json::to_string(&f)?,
                        expires: 0,
                    },
                )
                .await
                .map_err(|e| anyhow!(e))?;
            Ok(true) 
        },
        Err(_) => Ok(false)
    }
}

pub(crate) async fn doi_pt(ctx: &Context, uid: String, folder: String, doi: String) -> Result<String> {

    let rawid = format!("raw-{}::{}", folder.clone(), doi.clone());
    let uqid = format!("doi-{}::{}", folder, doi);
    let (rawo, po) = future::join(
        KeyValueSender::new().get(ctx, &rawid),
        KeyValueSender::new().get(ctx, &uqid)
    ).await;

    match (rawo, po) {
        (Ok(r),_) => {
            let resp: Value = serde_json::from_str(&r.value)?;
            let mut dn = DoiNode {
                folder: folder,
                doi: Some(doi),
                publisher: resp["message"]["publisher"].as_str().map(|s| s.to_string()),
                pub_year: None,
                pub_month: None,
                pub_day: None,
                pol_year: None,
                pol_month: None,
                pol_day: None,
                create_year: None,
                create_month: None,
                create_day: None,
                titles: vec![],
                summary: resp["message"]["abstract"].as_str().map(|s| s.to_string().replace("<jats:p>", "").replace("</jats:p>","")),
                typ: resp["message"]["type"].as_str().map(|s| s.to_string()),
                authors: vec![],
                editors: vec![],
                links: vec![]
            };
            if let Some(dp_y) = resp["message"]["published-print"]["date-parts"][0][0].as_u64() {
                dn.pub_year = Some(dp_y as u16);
        
                if let Some(dp_m) = resp["message"]["published-print"]["date-parts"][0][1].as_u64() {
                    dn.pub_month = Some(dp_m as u8);
        
                    if let Some(dp_d) = resp["message"]["published-print"]["date-parts"][0][2].as_u64() {
                        dn.pub_day = Some(dp_d as u8);
                    }
                }
            }
        
            if let Some(dp_y) = resp["message"]["published-online"]["date-parts"][0][0].as_u64() {
                dn.pol_year = Some(dp_y as u16);
        
                if let Some(dp_m) = resp["message"]["published-online"]["date-parts"][0][1].as_u64() {
                    dn.pol_month = Some(dp_m as u8);
        
                    if let Some(dp_d) = resp["message"]["published-online"]["date-parts"][0][2].as_u64() {
                        dn.pol_day = Some(dp_d as u8);
                    }
                }
            }
        
            if let Some(dp_y) = resp["message"]["created"]["date-parts"][0][0].as_u64() {
                dn.create_year = Some(dp_y as u16);
        
                if let Some(dp_m) = resp["message"]["created"]["date-parts"][0][1].as_u64() {
                    dn.create_month = Some(dp_m as u8);
        
                    if let Some(dp_d) = resp["message"]["created"]["date-parts"][0][2].as_u64() {
                        dn.create_day = Some(dp_d as u8);
                    }
                }
            }
        
            if let Some(ts) = resp["message"]["title"].as_array() {
                dn.titles = ts.iter().filter_map(Value::as_str)
                    .map(String::from)
                    .collect();
            }
        // /rust treat ["link"] as type link, but they are strings as above
            // if let Some(ts) = resp["message"]["link"].as_array() {
            //     dn.links = ts.iter().filter_map(Value::as_str)
            //         .map(String::from)
            //         .collect();
            // }
            if let Some(ts) = resp["message"]["author"].as_array() {
                for n in ts {
                    let mut nm = Name{
                        // affiliations: vec![],
                        suffix: None,
                        given: None,
                        family: None,
                        prefix: None,
                        name: None,
                    };
                    if let Some(s) = n.get("suffix") {
                        if let Some(t) = s.as_str() {
                            nm.suffix = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("given") {
                        if let Some(t) = s.as_str() {
                            nm.given = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("family") {
                        if let Some(t) = s.as_str() {
                            nm.family = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("prefix") {
                        if let Some(t) = s.as_str() {
                            nm.prefix = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("name") {
                        if let Some(t) = s.as_str() {
                            nm.name = Some(t.to_string());
                        }
                    }
                    dn.authors.push(nm);
                };
            }
        
            if let Some(ts) = resp["message"]["editor"].as_array() {
                for n in ts {
                    let mut nm = Name{
                        // affiliations: vec![],
                        suffix: None,
                        given: None,
                        family: None,
                        prefix: None,
                        name: None,
                    };
                    if let Some(s) = n.get("suffix") {
                        if let Some(t) = s.as_str() {
                            nm.suffix = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("given") {
                        if let Some(t) = s.as_str() {
                            nm.given = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("family") {
                        if let Some(t) = s.as_str() {
                            nm.family = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("prefix") {
                        if let Some(t) = s.as_str() {
                            nm.prefix = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("name") {
                        if let Some(t) = s.as_str() {
                            nm.name = Some(t.to_string());
                        }
                    }
                    dn.editors.push(nm)
                }
            }

            KeyValueSender::new()
            .set(
                ctx,
                &SetRequest {
                    key: uqid.to_string(),
                    value: serde_json::to_string(&dn)?,
                    expires: 0,
                },
            )
            .await
            .map_err(|e| anyhow!(e))?;

            Ok(serde_json::to_string(&dn)?)
        },
        (_, Ok(v)) => Ok(v.value.to_string() + " - from cache data"),
        (Err(e), _) | (_, Err(e)) => Err(anyhow!(e))
    }       
}

pub(crate) async fn doi_r_p(ctx: &Context, folder: String, doi: String) -> Result<DoiNode> {
    
    let rawid = format!("raw-{}::{}", folder.clone(), doi.clone());
    let uqid = format!("doi-{}::{}", folder, doi);
    let (rawo, po) = future::join(
        KeyValueSender::new().get(ctx, &rawid),
        KeyValueSender::new().get(ctx, &uqid)
    ).await;

    match (rawo, po) {
        (_, Ok(v)) => Ok(serde_json::from_str(&v.value)?),
        (Ok(r),_) => {
            let resp: Value = serde_json::from_str(&r.value)?;
            let mut dn = DoiNode {
                // id: uqid,
                folder: folder,
                doi: Some(doi),
                // date_year: None,
                // date_month: None,
                // date_day: None,
                publisher: resp["message"]["publisher"].as_str().map(|s| s.to_string()),
                pub_year: None,
                pub_month: None,
                pub_day: None,
                pol_year: None,
                pol_month: None,
                pol_day: None,
                create_year: None,
                create_month: None,
                create_day: None,
                titles: vec![],
                summary: resp["message"]["abstract"].as_str().map(|s| s.to_string().replace("<jats:p>", "").replace("</jats:p>","")),
                typ: resp["message"]["type"].as_str().map(|s| s.to_string()),
                authors: vec![],
                editors: vec![],
                links: vec![]
            };
            if let Some(dp_y) = resp["message"]["published-print"]["date-parts"][0][0].as_u64() {
                dn.pub_year = Some(dp_y as u16);
        
                if let Some(dp_m) = resp["message"]["published-print"]["date-parts"][0][1].as_u64() {
                    dn.pub_month = Some(dp_m as u8);
        
                    if let Some(dp_d) = resp["message"]["published-print"]["date-parts"][0][2].as_u64() {
                        dn.pub_day = Some(dp_d as u8);
                    }
                }
            }
        
            if let Some(dp_y) = resp["message"]["published-online"]["date-parts"][0][0].as_u64() {
                dn.pol_year = Some(dp_y as u16);
        
                if let Some(dp_m) = resp["message"]["published-online"]["date-parts"][0][1].as_u64() {
                    dn.pol_month = Some(dp_m as u8);
        
                    if let Some(dp_d) = resp["message"]["published-online"]["date-parts"][0][2].as_u64() {
                        dn.pol_day = Some(dp_d as u8);
                    }
                }
            }
        
            if let Some(dp_y) = resp["message"]["created"]["date-parts"][0][0].as_u64() {
                dn.create_year = Some(dp_y as u16);
        
                if let Some(dp_m) = resp["message"]["created"]["date-parts"][0][1].as_u64() {
                    dn.create_month = Some(dp_m as u8);
        
                    if let Some(dp_d) = resp["message"]["created"]["date-parts"][0][2].as_u64() {
                        dn.create_day = Some(dp_d as u8);
                    }
                }
            }
        
            if let Some(ts) = resp["message"]["title"].as_array() {
                dn.titles = ts.iter().filter_map(Value::as_str)
                    .map(String::from)
                    .collect();
            }
        // /rust treat ["link"] as type link, but they are strings as above
            // if let Some(ts) = resp["message"]["link"].as_array() {
            //     dn.links = ts.iter().filter_map(Value::as_str)
            //         .map(String::from)
            //         .collect();
            // }
            if let Some(ts) = resp["message"]["author"].as_array() {
                for n in ts {
                    let mut nm = Name{
                        // affiliations: vec![],
                        suffix: None,
                        given: None,
                        family: None,
                        prefix: None,
                        name: None,
                    };
                    if let Some(s) = n.get("suffix") {
                        if let Some(t) = s.as_str() {
                            nm.suffix = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("given") {
                        if let Some(t) = s.as_str() {
                            nm.given = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("family") {
                        if let Some(t) = s.as_str() {
                            nm.family = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("prefix") {
                        if let Some(t) = s.as_str() {
                            nm.prefix = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("name") {
                        if let Some(t) = s.as_str() {
                            nm.name = Some(t.to_string());
                        }
                    }
                    dn.authors.push(nm);
                };
            }
        
            if let Some(ts) = resp["message"]["editor"].as_array() {
                for n in ts {
                    let mut nm = Name{
                        // affiliations: vec![],
                        suffix: None,
                        given: None,
                        family: None,
                        prefix: None,
                        name: None,
                    };
                    if let Some(s) = n.get("suffix") {
                        if let Some(t) = s.as_str() {
                            nm.suffix = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("given") {
                        if let Some(t) = s.as_str() {
                            nm.given = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("family") {
                        if let Some(t) = s.as_str() {
                            nm.family = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("prefix") {
                        if let Some(t) = s.as_str() {
                            nm.prefix = Some(t.to_string());
                        }
                    }
                    if let Some(s) = n.get("name") {
                        if let Some(t) = s.as_str() {
                            nm.name = Some(t.to_string());
                        }
                    }
                    dn.editors.push(nm)
                }
            }

            KeyValueSender::new()
            .set(
                ctx,
                &SetRequest {
                    key: uqid.to_string(),
                    value: serde_json::to_string(&dn)?,
                    expires: 0,
                },
            )
            .await
            .map_err(|e| anyhow!(e))?; 

            Ok(dn)
        },
        (Err(e), _) | (_, Err(e)) => Err(anyhow!(e))
    }
}

pub(crate) async fn doi_ij_2(ctx: &Context, uid: String, folder: String, data: Vec<u8>) -> Result<DoiNode> {
    let resp = serde_json::from_slice::<Value>(&data).map_err(|e| anyhow!(e))?;
    if let Some(doi_str) = resp["message"]["DOI"].as_str() {
        let doi = doi_str.to_string();
        let rawid = format!("raw-{}::{}", folder.clone(), doi.clone());
        let uqid = format!("doi-{}::{}", folder.clone(), doi.clone());

        KeyValueSender::new()
        .set(
            ctx,
            &SetRequest {
                key: rawid.to_string(),
                value: serde_json::to_string(&resp)?,
                expires: 0,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?; 

        let mut dn = DoiNode {
            // id: uqid,
            folder: folder,
            doi: Some(doi),
            // date_year: None,
            // date_month: None,
            // date_day: None,
            publisher: resp["message"]["publisher"].as_str().map(|s| s.to_string()),
            pub_year: None,
            pub_month: None,
            pub_day: None,
            pol_year: None,
            pol_month: None,
            pol_day: None,
            create_year: None,
            create_month: None,
            create_day: None,
            titles: vec![],
            summary: resp["message"]["abstract"].as_str().map(|s| s.to_string().replace("<jats:p>", "").replace("</jats:p>","")),
            typ: resp["message"]["type"].as_str().map(|s| s.to_string()),
            authors: vec![],
            editors: vec![],
            links: vec![]
        };
        if let Some(dp_y) = resp["message"]["published-print"]["date-parts"][0][0].as_u64() {
            dn.pub_year = Some(dp_y as u16);
    
            if let Some(dp_m) = resp["message"]["published-print"]["date-parts"][0][1].as_u64() {
                dn.pub_month = Some(dp_m as u8);
    
                if let Some(dp_d) = resp["message"]["published-print"]["date-parts"][0][2].as_u64() {
                    dn.pub_day = Some(dp_d as u8);
                }
            }
        }
    
        if let Some(dp_y) = resp["message"]["published-online"]["date-parts"][0][0].as_u64() {
            dn.pol_year = Some(dp_y as u16);
    
            if let Some(dp_m) = resp["message"]["published-online"]["date-parts"][0][1].as_u64() {
                dn.pol_month = Some(dp_m as u8);
    
                if let Some(dp_d) = resp["message"]["published-online"]["date-parts"][0][2].as_u64() {
                    dn.pol_day = Some(dp_d as u8);
                }
            }
        }
    
        if let Some(dp_y) = resp["message"]["created"]["date-parts"][0][0].as_u64() {
            dn.create_year = Some(dp_y as u16);
    
            if let Some(dp_m) = resp["message"]["created"]["date-parts"][0][1].as_u64() {
                dn.create_month = Some(dp_m as u8);
    
                if let Some(dp_d) = resp["message"]["created"]["date-parts"][0][2].as_u64() {
                    dn.create_day = Some(dp_d as u8);
                }
            }
        }
    
        if let Some(ts) = resp["message"]["title"].as_array() {
            dn.titles = ts.iter().filter_map(Value::as_str)
                .map(String::from)
                .collect();
        }
    // /rust treat ["link"] as type link, but they are strings as above
        // if let Some(ts) = resp["message"]["link"].as_array() {
        //     dn.links = ts.iter().filter_map(Value::as_str)
        //         .map(String::from)
        //         .collect();
        // }
        if let Some(ts) = resp["message"]["author"].as_array() {
            for n in ts {
                let mut nm = Name{
                    // affiliations: vec![],
                    suffix: None,
                    given: None,
                    family: None,
                    prefix: None,
                    name: None,
                };
                if let Some(s) = n.get("suffix") {
                    if let Some(t) = s.as_str() {
                        nm.suffix = Some(t.to_string());
                    }
                }
                if let Some(s) = n.get("given") {
                    if let Some(t) = s.as_str() {
                        nm.given = Some(t.to_string());
                    }
                }
                if let Some(s) = n.get("family") {
                    if let Some(t) = s.as_str() {
                        nm.family = Some(t.to_string());
                    }
                }
                if let Some(s) = n.get("prefix") {
                    if let Some(t) = s.as_str() {
                        nm.prefix = Some(t.to_string());
                    }
                }
                if let Some(s) = n.get("name") {
                    if let Some(t) = s.as_str() {
                        nm.name = Some(t.to_string());
                    }
                }
                dn.authors.push(nm);
            };
        }
    
        if let Some(ts) = resp["message"]["editor"].as_array() {
            for n in ts {
                let mut nm = Name{
                    // affiliations: vec![],
                    suffix: None,
                    given: None,
                    family: None,
                    prefix: None,
                    name: None,
                };
                if let Some(s) = n.get("suffix") {
                    if let Some(t) = s.as_str() {
                        nm.suffix = Some(t.to_string());
                    }
                }
                if let Some(s) = n.get("given") {
                    if let Some(t) = s.as_str() {
                        nm.given = Some(t.to_string());
                    }
                }
                if let Some(s) = n.get("family") {
                    if let Some(t) = s.as_str() {
                        nm.family = Some(t.to_string());
                    }
                }
                if let Some(s) = n.get("prefix") {
                    if let Some(t) = s.as_str() {
                        nm.prefix = Some(t.to_string());
                    }
                }
                if let Some(s) = n.get("name") {
                    if let Some(t) = s.as_str() {
                        nm.name = Some(t.to_string());
                    }
                }
                dn.editors.push(nm)
            }
        }

        KeyValueSender::new()
        .set(
            ctx,
            &SetRequest {
                key: uqid.to_string(),
                value: serde_json::to_string(&dn)?,
                expires: 0,
            },
        )
        .await
        .map_err(|e| anyhow!(e))?; 

        Ok(dn)
    } else {
        Err(anyhow!("invalid input"))
    }
}


pub(crate) async fn doi_f(ctx: &Context, uid: String, folder: String, doi: String) -> Result<Value> {

    let uqid = format!("raw-{}::{}", folder.clone(), doi.clone());//.to_string()

    let fo = remote::doi_fetch(ctx, doi.clone()).await;

    match remote::doi_fetch(ctx, doi.clone()).await {
        Ok(resp) => {
            KeyValueSender::new()
                .set(
                    ctx,
                    &SetRequest {
                        key: uqid.to_string(),
                        value: serde_json::to_string(&resp)?,
                        expires: 0,
                    },
                )
                .await
                .map_err(|e| anyhow!(e))?; 
            Ok(resp)
        },
        Err(e) => Err(e)
    }
}

pub(crate) async fn doi_c(ctx: &Context, uid: String, folder: String, doi: String) -> Result<Value> {

    let uqid = format!("raw-{}::{}", folder.clone(), doi.clone());//.to_string()

    let (kvo, fo) = future::join(
        doi_to_folder(ctx, uid, folder.clone(), doi.clone()),
        remote::doi_fetch(ctx, doi.clone())
    ).await;

    match (kvo, fo) {
        (Ok(_), Ok(resp)) => {
            KeyValueSender::new()
                .set(
                    ctx,
                    &SetRequest {
                        key: uqid.to_string(),
                        value: serde_json::to_string(&resp)?,
                        expires: 0,
                    },
                )
                .await
                .map_err(|e| anyhow!(e))?; 
            Ok(resp)
        },
        (Err(e), _) | (_, Err(e)) => {
            Err(e)
        }
    }
}