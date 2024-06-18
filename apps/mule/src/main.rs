use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use std::path::PathBuf;
use std::env;
use tracing::{error, info};
use tokio::signal::unix::{signal, SignalKind};

fn get_cdn_root() -> String {
    env::var("CDN_ROOT").unwrap_or_else(|_| {
        "./cdn_root".to_string()
    })
}

async fn serve_file(req: Request<Body>) -> Result<Response<Body>, Infallible> {
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
            error!("File not found: {}", e);
            Ok(Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::from("File Not Found"))
                .unwrap())
        }
    }
}

async fn health_check(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response_body = "OK";
    info!("Health check status: {}", response_body);
    Ok(Response::new(Body::from(response_body)))
}

async fn shutdown_signal() {
    // Create a future to listen for the SIGINT (Ctrl+C) signal
    let mut sigint = signal(SignalKind::interrupt()).expect("Failed to register SIGINT handler");

    // Create a future to listen for the SIGTERM signal
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to register SIGTERM handler");

    // Wait for either SIGINT or SIGTERM
    tokio::select! {
        _ = sigint.recv() => {
            info!("Received SIGINT, shutting down gracefully");
        }
        _ = sigterm.recv() => {
            info!("Received SIGTERM, shutting down gracefully");
        }
    }
}

#[tokio::main]
async fn main() {
    let log_level = if env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true" {
        tracing::Level::TRACE
    } else {
        tracing::Level::INFO
    };
    // Initialize the tracing subscriber with the desired log level
    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();

    let cdn_root = get_cdn_root();
    info!("Serving files from: {}", cdn_root);

    // Main file server
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(serve_file))
    });

    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    let graceful_server = server.with_graceful_shutdown(shutdown_signal());

    info!("File server listening on http://{}", addr);

    // Health check server
    let health_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(health_check))
    });

    let health_addr = ([0, 0, 0, 0], 8080).into();
    let health_server = Server::bind(&health_addr).serve(health_svc);
    let graceful_health_server = health_server.with_graceful_shutdown(shutdown_signal());

    info!("Health check server listening on http://{}", health_addr);

    // Run both servers concurrently
    tokio::select! {
        res = graceful_server => {
            if let Err(e) = res {
                error!("File server error: {}", e);
            }
        }
        res = graceful_health_server => {
            if let Err(e) = res {
                error!("Health check server error: {}", e);
            }
        }
    }
}
