use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

pub type Cache = Arc<RwLock<HashMap<String, String>>>;
