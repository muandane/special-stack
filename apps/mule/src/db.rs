use sqlx::sqlite::{SqlitePool};
use sqlx::Executor;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;
use std::fs;
use std::path::Path;

pub type DbPool = Arc<RwLock<SqlitePool>>;

pub async fn init_db(db_url: &str) -> DbPool {
    // Extract directory from the DB URL and ensure it exists
    let path = &db_url[9..]; // Skip "sqlite://" part
    let db_path = Path::new(path);

    // Ensure the directory exists
    if let Some(dir) = db_path.parent() {
        if !dir.exists() {
            info!("Creating directory for database: {}", dir.display());
            fs::create_dir_all(dir).expect("Failed to create database directory");
        }
    } else {
        panic!("Invalid database path: {}", db_path.display());
    }

    // Ensure the database file exists
    if !db_path.exists() {
        info!("Creating database file: {}", db_path.display());
        fs::File::create(&db_path).expect("Failed to create database file");
    }

    // Use absolute path for the database URL
    let absolute_db_url = format!("sqlite://{}", db_path.canonicalize().expect("Failed to get canonical path").display());

    // Create the database pool
    let pool = SqlitePool::connect(&absolute_db_url).await.expect("Failed to create database pool");
    let pool = Arc::new(RwLock::new(pool));
    initialize_schema(&pool).await;
    pool
}

async fn initialize_schema(pool: &DbPool) {
    let sql = r#"
    CREATE TABLE IF NOT EXISTS mappings (
        id INTEGER PRIMARY KEY,
        hash TEXT NOT NULL,
        url TEXT NOT NULL
    );
    "#;
    pool.write().await.execute(sql).await.expect("Failed to initialize schema");
}

pub async fn save_mapping(pool: &DbPool, hash: &str, url: &str) {
    let sql = "INSERT INTO mappings (hash, url) VALUES (?, ?)";
    pool.write().await.execute(sqlx::query(sql).bind(hash).bind(url)).await.expect("Failed to save mapping");
}

pub async fn get_mappings(pool: &DbPool) -> Vec<(String, String)> {
    let sql = "SELECT hash, url FROM mappings";
    sqlx::query_as::<_, (String, String)>(sql)
        .fetch_all(&*pool.read().await)
        .await
        .expect("Failed to fetch mappings")
}
