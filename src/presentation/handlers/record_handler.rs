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
    presentation::routes::record_routes::RecordRoutesState,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewRecord {
    pub key: String,
    pub value: String,
}

#[axum::debug_handler]
pub async fn set_record_handler(
    State(state): State<Arc<RecordRoutesState>>,
    Path(store): Path<String>,
    Json(input): Json<NewRecord>,
) -> Response {
    match SetRecordUseCase::new(state.repo.clone()).execute(&input, &store) {
        Ok(_) => Json(input).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

#[axum::debug_handler]
pub async fn get_record_handler(
    State(state): State<Arc<RecordRoutesState>>,
    Path((store, key)): Path<(String, String)>,
) -> Response {
    match GetRecordUseCase::new(state.repo.clone()).execute(&key, &store) {
        Ok(Some(record)) => Json(NewRecord {
            key: record.key,
            value: record.value,
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "record not found!").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}
