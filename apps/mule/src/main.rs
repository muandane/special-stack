mod handlers;
mod cache;
mod config;
mod shutdown;

use handlers::file::serve_file;
use handlers::management::{cache_file, get_cache_mapping};
use handlers::health::health_check;
use cache::Cache;
use config::init_logging;
use shutdown::shutdown_signal;
use hyper::{Server, Response, Body, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    let cdn_root = env::var("CDN_ROOT").unwrap_or_else(|_| {
        info!("CDN_ROOT environment variable not set, using default path: ./cdn_root");
        "./cdn_root".to_string()
    });

    info!("Serving files from: {}", cdn_root);

    // Create the cache
    let cache: Cache = Arc::new(RwLock::new(std::collections::HashMap::new()));
    let cache_for_main_svc = Arc::clone(&cache);
    let cache_for_management_svc = Arc::clone(&cache);

    // Main file server
    let make_svc = make_service_fn(move |_conn| {
        let cache = Arc::clone(&cache_for_main_svc);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let cache = Arc::clone(&cache);
                serve_file(req, cache)
            }))
        }
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

    // Management server for caching and health check
    let management_svc = make_service_fn(move |_conn| {
        let cache = Arc::clone(&cache_for_management_svc);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let cache = Arc::clone(&cache);
                async move {
                    match (req.method(), req.uri().path()) {
                        (&hyper::Method::POST, "/cache") => cache_file(req, cache).await,
                        (&hyper::Method::GET, "/mappings") => get_cache_mapping(req, cache).await,
                        _ => Ok(Response::builder()
                                .status(StatusCode::NOT_FOUND)
                                .body(Body::from("Not Found"))
                                .unwrap()
                        ),
                    }
                }
            }))
        }
    });

    let management_addr = ([0, 0, 0, 0], 9001).into();
    let management_server = Server::bind(&management_addr).serve(management_svc);
    let graceful_management_server = management_server.with_graceful_shutdown(shutdown_signal());

    info!("Management server listening on http://{}", management_addr);

    // Run all servers concurrently
    tokio::select! {
        res = graceful_server => {
            if let Err(e) = res {
                tracing::error!("File server error: {}", e);
            }
        }
        res = graceful_health_server => {
            if let Err(e) = res {
                tracing::error!("Health check server error: {}", e);
            }
        }
        res = graceful_management_server => {
            if let Err(e) = res {
                tracing::error!("Management server error: {}", e);
            }
        }
    }
}
