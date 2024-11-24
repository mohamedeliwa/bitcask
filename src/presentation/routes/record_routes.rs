use axum::{
    routing::{get, post},
    Router,
};

use crate::{
    infrastructure::web::WebAppStateType,
    presentation::handlers::record_handler::{get_record_handler, set_record_handler},
};

/// builds store related routes
/// returns store router
pub fn routes() -> Router<WebAppStateType> {
    Router::new()
        .route("/record/:store", post(set_record_handler))
        .route("/record/:store/:key", get(get_record_handler))
}
