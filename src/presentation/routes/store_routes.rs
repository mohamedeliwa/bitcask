use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    infrastructure::web::WebAppStateType,
    presentation::handlers::store_handler::{create_store_handler, get_store_by_id_handler},
};

/// builds store related routes
/// returns store router
pub fn routes() -> Router<WebAppStateType> {
    Router::new()
        .route("/store", post(create_store_handler))
        .route("/store/:id", get(get_store_by_id_handler))
}
