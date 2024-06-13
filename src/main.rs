use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::path::PathBuf;
use std::env;
use tracing::{error, info, trace};

async fn serve_file(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let cdn_root = env::var("CDN_ROOT").unwrap_or_else(|_| {
        info!("CDN_ROOT environment variable not set, using default path: ./cdn_root");
        "./cdn_root".to_string()
    });

    let path = req.uri().path().trim_start_matches('/');
    let file_path = PathBuf::from(&cdn_root).join(path);

    trace!("Serving file: {}", file_path.display());

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
            error!("File not found: {}", e);
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("File Not Found"))
                .unwrap())
        }
    }
}
#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber with the desired log level
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();
    let cdn_root = env::var("CDN_ROOT").unwrap_or_else(|_| {
        info!("CDN_ROOT environment variable not set, using default path: ./cdn_root");
        "./cdn_root".to_string()
    });

    info!("Serving files from: {}", cdn_root);
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(serve_file))
    });
    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    info!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}