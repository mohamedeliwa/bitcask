use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
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

pub async fn create_store_handler(
    State(repo): State<Arc<DiskStoreRepo>>,
    Json(input): Json<NewStore>,
) -> Result<(), StatusCode> {
    if let Ok(_) = CreateStoreUseCase::new(repo).execute(input) {
        Ok(())
    } else {
        Err(StatusCode::BAD_REQUEST)
    }
}

pub async fn get_store_by_id_handler(
    State(repo): State<Arc<DiskStoreRepo>>,
    Path(id): Path<String>,
) -> Result<String, StatusCode> {
    if let Some(store) = GetStoreByIdUseCase::new(repo).execute(id) {
        Ok(store.name)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
