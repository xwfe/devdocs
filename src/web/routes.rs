//! u8defu7531u914du7f6e

use axum::Router;
use axum::routing::get;
use crate::core::config::Config;
use super::handlers;
use std::sync::Arc;
use super::handlers::AppState;
use crate::docs::registry::DocRegistry;

/// u521bu5efau6240u6709u5e94u7528u7a0bu5e8fu8defu7531
pub fn create_routes(_config: &Config) -> Router {
    Router::new()
        .route("/", get(handlers::index))
        .route("/ping", get(handlers::ping))
        .route("/search", get(handlers::search))
        .route("/docs.json", get(handlers::docs_list))
        .route("/docs/:doc", get(handlers::doc_index))
        .route("/docs/:doc/*page", get(handlers::doc_page))
        .with_state(Arc::new(AppState {
            config: _config.clone(),
            doc_registry: Arc::new(DocRegistry::new()),
        }))
}