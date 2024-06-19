mod handlers;
mod cache;
mod config;
mod shutdown;
mod db;

use handlers::file::serve_file;
use handlers::management::{cache_file, get_cache_mapping};
use handlers::health::health_check;
use config::init_logging;
use shutdown::shutdown_signal;
use db::{init_db};

use hyper::{Body, Request, Response, Server, StatusCode};
use hyper::service::{make_service_fn, service_fn};
use std::convert::Infallible;
use std::env;
use std::sync::Arc;
use tracing::{info, error};

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    let cdn_root = env::var("CDN_ROOT").unwrap_or_else(|_| {
        info!("CDN_ROOT environment variable not set, using default path: ./cdn_root");
        "./cdn_root".to_string()
    });

    info!("Serving files from: {}", cdn_root);

    let db_url = "sqlite://./data/cache_mappings.db";
    let db_pool = init_db(db_url).await;

    // Main file server
    let db_pool_for_main_svc = Arc::clone(&db_pool);
    let make_svc = make_service_fn(move |_conn| {
        let db_pool = Arc::clone(&db_pool_for_main_svc);
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                let db_pool = Arc::clone(&db_pool);
                serve_file(req, db_pool)
            }))
        }
    });


    let addr = ([0, 0, 0, 0], 3000).into();
    let server = Server::bind(&addr).serve(make_svc);
    let graceful_server = server.with_graceful_shutdown(shutdown_signal());

    info!("File server listening on http://{}", addr);

    // Management server
    let db_pool_for_management_svc = Arc::clone(&db_pool);
    let management_svc = make_service_fn(move |_conn| {
        let db_pool = Arc::clone(&db_pool_for_management_svc);
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let db_pool = Arc::clone(&db_pool);
                async move {
                    match (req.method(), req.uri().path()) {
                        (&hyper::Method::POST, "/cache") => cache_file(req, db_pool).await,
                        (&hyper::Method::GET, "/mappings") => get_cache_mapping(req, db_pool).await,
                        _ => Ok(Response::builder()
                            .status(StatusCode::NOT_FOUND)
                            .body(Body::from("Not Found"))
                            .unwrap())
                    }
                }
            }))
        }
    });

    let management_addr = ([0, 0, 0, 0], 9001).into();
    let management_server = Server::bind(&management_addr).serve(management_svc);
    let graceful_management_server = management_server.with_graceful_shutdown(shutdown_signal());

    info!("Management server listening on http://{}", management_addr);

    // Health check server
    let health_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(health_check))
    });

    let health_addr = ([0, 0, 0, 0], 8080).into();
    let health_server = Server::bind(&health_addr).serve(health_svc);
    let graceful_health_server = health_server.with_graceful_shutdown(shutdown_signal());

    info!("Health check server listening on http://{}", health_addr);

    // Run all servers concurrently
    tokio::select! {
        res = graceful_server => {
            if let Err(e) = res {
                error!("File server error: {}", e);
            }
        }
        res = graceful_management_server => {
            if let Err(e) = res {
                error!("Management server error: {}", e);
            }
        }
        res = graceful_health_server => {
            if let Err(e) = res {
                error!("Health check server error: {}", e);
            }
        }
    }
}
