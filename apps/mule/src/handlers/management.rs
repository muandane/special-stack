use hyper::{Body, Request, Response, StatusCode};
use crate::db::DbPool;
use crate::cache::{add_to_cache, fetch_mappings};
use tracing::{info, error};
use std::convert::Infallible;
use serde_json::json;
use sha2::{Sha256, Digest};
use tokio::fs;
use tokio::io::AsyncWriteExt;
use crate::utils::with_cors;

pub async fn cache_file(cdn_root: String,req: Request<Body>, db: DbPool) -> Result<Response<Body>, Infallible> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let url = String::from_utf8(whole_body.to_vec()).unwrap();
    let hash = format!("{:x}", Sha256::digest(url.as_bytes()));

    match reqwest::get(&url).await {
        Ok(resp) => {
            let bytes = resp.bytes().await.unwrap();
            let file_path = format!("{}/{}", cdn_root, hash);
            let mut file = fs::File::create(&file_path).await.unwrap();
            file.write_all(&bytes).await.unwrap();

            add_to_cache(db, &hash, &url).await;
            info!("Cached file '{}' with hash '{}'", &url, &hash);
            Ok(with_cors(Response::new(Body::from("Cached successfully"))))
        },
        Err(err) => {
            error!("Failed to fetch the URL: {}", err);
            Ok(with_cors(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Failed to fetch the URL"))
                .unwrap()))
        }
    }
}

pub async fn get_cache_mapping(_req: Request<Body>, db: DbPool) -> Result<Response<Body>, Infallible> {
    let mappings = fetch_mappings(db).await;
    let json = json!(mappings);

    Ok(with_cors(Response::new(Body::from(json.to_string()))))
}
