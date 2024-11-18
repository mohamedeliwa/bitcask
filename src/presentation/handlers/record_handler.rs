use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    application::use_cases::{get_record::GetRecordUseCase, set_record::SetRecordUseCase},
    infrastructure::repositories::disk_record_repo::DiskRecordRepo,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewRecord {
    pub key: String,
    pub value: String,
}

#[axum::debug_handler]
pub async fn set_record_handler(
    State(repo): State<Arc<DiskRecordRepo>>,
    Path(store): Path<String>,
    Json(input): Json<NewRecord>,
) -> Response {
    match SetRecordUseCase::new(repo).execute(&input, &store) {
        Ok(_) => Json(input).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_record_handler(
    State(repo): State<Arc<DiskRecordRepo>>,
    Path((store, key)): Path<(String, String)>,
) -> Response {
    if let Some(record) = GetRecordUseCase::new(repo).execute(&key, &store) {
        Json(NewRecord {
            key: record.key,
            value: record.value,
        })
        .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}