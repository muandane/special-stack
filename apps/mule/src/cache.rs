use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::sqlite::SqlitePool;
use crate::db::{save_mapping, get_mappings};

pub type Cache = Arc<RwLock<SqlitePool>>;

pub async fn add_to_cache(pool: Cache, hash: &str, url: &str) {
    save_mapping(&pool, hash, url).await;
}

pub async fn fetch_mappings(pool: Cache) -> Vec<(String, String)> {
    get_mappings(&pool).await
}
