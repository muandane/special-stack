use hyper::{Body, Request, Response, StatusCode};
use crate::db::DbPool;
use crate::cache::{add_to_cache, fetch_mappings};
use tracing::{info, error};
use std::convert::Infallible;
use serde_json::json;
use sha2::{Sha256, Digest};
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn cache_file(req: Request<Body>, db: DbPool) -> Result<Response<Body>, Infallible> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let url = String::from_utf8(whole_body.to_vec()).unwrap();
    let hash = format!("{:x}", Sha256::digest(url.as_bytes()));

    match reqwest::get(&url).await {
        Ok(resp) => {
            let bytes = resp.bytes().await.unwrap();
            let cdn_root = "./cdn_root".to_string();
            let file_path = format!("{}/{}", cdn_root, hash);
            let mut file = fs::File::create(&file_path).await.unwrap();
            file.write_all(&bytes).await.unwrap();

            add_to_cache(db, &hash, &url).await;
            info!("Cached file '{}' with hash '{}'", &url, &hash);
            Ok(Response::new(Body::from("Cached successfully")))
        },
        Err(err) => {
            error!("Failed to fetch the URL: {}", err);
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Failed to fetch the URL"))
                .unwrap())
        }
    }
}

pub async fn get_cache_mapping(_req: Request<Body>, db: DbPool) -> Result<Response<Body>, Infallible> {
    let mappings = fetch_mappings(db).await;
    let json = json!(mappings);

    Ok(Response::new(Body::from(json.to_string())))
}
