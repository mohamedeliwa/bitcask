use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    application::use_cases::{
        create_store::CreateStoreUseCase, get_store_by_id::GetStoreByIdUseCase,
    },
    infrastructure::repositories::disk_store_repo::DiskStoreRepo,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewStore {
    pub id: String,
    pub name: String,
}

#[axum::debug_handler]
pub async fn create_store_handler(
    State(repo): State<Arc<DiskStoreRepo>>,
    Json(input): Json<NewStore>,
) -> Response {
    match CreateStoreUseCase::new(repo).execute(&input) {
        Ok(_) => Json(input).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_store_by_id_handler(
    State(repo): State<Arc<DiskStoreRepo>>,
    Path(id): Path<String>,
) -> Response {
    if let Some(store) = GetStoreByIdUseCase::new(repo).execute(&id) {
        Json(NewStore {
            id: store.id,
            name: store.name,
        })
        .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
