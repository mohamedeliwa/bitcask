use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    infrastructure::repositories::disk_store_repo::DiskStoreRepo,
    presentation::handlers::store_handler::{create_store_handler, get_store_by_id_handler},
};

/// builds store related routes
/// returns store router
pub fn routes() -> Router {
    // can be read from the env
    // or passed as a param from the caller
    let path = "./stores";
    let repo = Arc::new(DiskStoreRepo::new(path));

    Router::new()
        .route("/store/:id", get(get_store_by_id_handler))
        .route("/store", post(create_store_handler))
        .with_state(repo)
}
