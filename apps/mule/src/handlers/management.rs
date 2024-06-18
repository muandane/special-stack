use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
use std::env;
use tracing::{info, error};
use sha2::{Sha256, Digest};
use tokio::fs;
use std::path::PathBuf;
use reqwest;

use crate::cache::Cache;

pub async fn cache_file(req: Request<Body>, cache: Cache) -> Result<Response<Body>, Infallible> {
    let whole_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
    let url = String::from_utf8(whole_body.to_vec()).unwrap();

    // Fetch the file content from the given URL
    let content = match reqwest::get(&url).await {
        Ok(resp) => match resp.bytes().await {
            Ok(bytes) => bytes,
            Err(e) => {
                error!("Failed to read bytes from response: {}", e);
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Failed to read bytes from response"))
                    .unwrap());
            }
        },
        Err(e) => {
            error!("Failed to fetch file from URL: {}", e);
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("Failed to fetch file from URL"))
                .unwrap());
        }
    };

    // Generate a hash for the filename
    let mut hasher = Sha256::new();
    hasher.update(&url);
    let hash = format!("{:x}", hasher.finalize());

    // Store the file in the CDN root directory with the hashed name
    let cdn_root = env::var("CDN_ROOT").unwrap_or_else(|_| "./cdn_root".to_string());
    let file_path = PathBuf::from(&cdn_root).join(&hash);
    if let Err(e) = fs::write(&file_path, &content).await {
        error!("Failed to write file to CDN: {}", e);
        return Ok(Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::from("Failed to write file to CDN"))
            .unwrap());
    }

    // Update the cache mapping
    {
        let mut cache_lock = cache.write().await;
        cache_lock.insert(hash.clone(), url.clone());
    }

    info!("Cached file from {} to {}", url.clone(), file_path.display());

    Ok(Response::new(Body::from(hash)))
}

pub async fn get_cache_mapping(_req: Request<Body>, cache: Cache) -> Result<Response<Body>, Infallible> {
    let cache_lock = cache.read().await;
    let mappings: Vec<String> = cache_lock.iter().map(|(k, v)| format!("{} -> {}", k, v)).collect();
    let body = mappings.join("\n");

    Ok(Response::new(Body::from(body)))
}
