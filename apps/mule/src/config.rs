use tracing::Level;

pub fn init_logging() {
    let log_level = if std::env::var("DEBUG").unwrap_or_else(|_| "false".to_string()) == "true" {
        Level::TRACE
    } else {
        Level::INFO
    };

    tracing_subscriber::fmt()
        .with_max_level(log_level)
        .init();
}
