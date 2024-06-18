use tokio::signal::unix::{signal, SignalKind};
use tracing::info;

pub async fn shutdown_signal() {
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
