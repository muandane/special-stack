use hyper::{Body, Request, Response, StatusCode};
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::path::PathBuf;
use std::convert::Infallible;
use tracing::info;

pub async fn serve_file(req: Request<Body>, _cache: crate::cache::Cache) -> Result<Response<Body>, Infallible> {
    let cdn_root = std::env::var("CDN_ROOT").unwrap_or_else(|_| {
        "/data/content".to_string()
    });

    let path = req.uri().path().trim_start_matches('/');
    let file_path = PathBuf::from(&cdn_root).join(path);

    info!("Serving file: {}", file_path.display());

    match File::open(&file_path).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            if let Err(_e) = file.read_to_end(&mut contents).await {
                return Ok(Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(Body::from("Internal Server Error"))
                    .unwrap());
            }
            Ok(Response::new(Body::from(contents)))
        }
        Err(_) => {
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("File Not Found"))
                .unwrap())
        }
    }
}
