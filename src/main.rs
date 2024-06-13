use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::path::PathBuf;
use tracing::{error, info, trace, warn};

async fn serve_file(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let path = req.uri().path().trim_start_matches('/');
    let mut file_path = PathBuf::from("cdn_root"); // Root directory for the CDN files
    file_path.push(path);

    trace!("Received request for file: {}", file_path.display());

    match File::open(&file_path).await {
        Ok(mut file) => {
            info!("File opened successfully: {}", file_path.display());
            let mut contents = Vec::new();
            match file.read_to_end(&mut contents).await {
                Ok(_) => {
                    trace!("File read successfully: {}", file_path.display());
                    Ok(Response::new(Body::from(contents)))
                }
                Err(e) => {
                    error!("Error reading file: {}", e);
                    Ok(Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::from("Internal Server Error"))
                        .unwrap())
                }
            }
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

#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber
    tracing_subscriber::fmt::init();

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(serve_file)) }
    });
    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    info!("Listening on http://{}", addr);
    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }
}