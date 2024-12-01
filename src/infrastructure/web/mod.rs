use std::sync::Arc;

use axum::Router;

use crate::{
    infrastructure::repositories::{
        disk_record_repo::DiskRecordRepo, disk_store_repo::DiskStoreRepo,
        mem_hash_index_repo::MemHashIndexRepo,
    },
    presentation::routes::{record_routes, store_routes},
};

#[derive(Clone)]
pub struct WebAppState<H, R, S> {
    pub hash_index_repo: H,
    pub record_repo: R,
    pub store_repo: S,
}

pub type WebAppStateType =
    WebAppState<Arc<MemHashIndexRepo>, Arc<DiskRecordRepo>, Arc<DiskStoreRepo>>;

/// initializes the http web server
pub async fn run() -> Result<(), String> {
    // can be read from the env
    // or passed as a param from the caller
    let path = "./stores";
    let hash_index_repo = Arc::new(MemHashIndexRepo::new(path));
    let record_repo = Arc::new(DiskRecordRepo::new(path));
    let store_repo = Arc::new(DiskStoreRepo::new(path));

    let state = WebAppState {
        hash_index_repo,
        record_repo,
        store_repo,
    };

    let store_routes = store_routes::routes();
    let record_routes = record_routes::routes();

    println!("Starting web server...!");

    // build our application with a route
    let app = Router::new()
        .merge(store_routes)
        .merge(record_routes)
        .with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
