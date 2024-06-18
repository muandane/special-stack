use hyper::{Body, Request, Response, StatusCode};
use std::convert::Infallible;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::path::PathBuf;
use std::env;
use tracing::{error, info, warn};
use crate::cache::Cache;

pub async fn serve_file(req: Request<Body>, _cache: Cache) -> Result<Response<Body>, Infallible> {
    let cdn_root = env::var("CDN_ROOT").unwrap_or_else(|_| {
        "./cdn_root".to_string()
    });

    let path = req.uri().path().trim_start_matches('/');
    let file_path = PathBuf::from(&cdn_root).join(path);

    info!("Serving file: {}", file_path.display());

    match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            if let Err(e) = file.read_to_end(&mut contents).await {
                error!("Error reading file: {}", e);
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal Server Error"))
                    .unwrap());
            }
            Ok(Response::new(Body::from(contents)))
        }
        Err(e) => {
            warn!("File not found: {}", e);
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("File Not Found"))
                .unwrap())
        }
    }
}
