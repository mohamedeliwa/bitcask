use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    infrastructure::repositories::{
        disk_record_repo::DiskRecordRepo, mem_hash_index_repo::MemHashIndexRepo,
    },
    presentation::handlers::record_handler::{get_record_handler, set_record_handler},
};

/// builds store related routes
/// returns store router
pub fn routes() -> Router {
    // can be read from the env
    // or passed as a param from the caller
    let path = "./stores";
    let hash_index = Arc::new(MemHashIndexRepo::new());
    let repo = Arc::new(DiskRecordRepo::new(path, hash_index));

    Router::new()
        .route("/record/:store", post(set_record_handler))
        .route("/record/:store/:key", get(get_record_handler))
        .with_state(repo)
}
