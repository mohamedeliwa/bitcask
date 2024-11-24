use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{
    application::use_cases::{
        get_hash_index_key::GetHasIndexKey, get_record::GetRecordUseCase,
        set_hash_index_key::SetHasIndexKey, set_record::SetRecordUseCase,
    },
    domain::repositories::{hash_index_repo::HashIndexRepo, record_repo::RecordRepo},
    infrastructure::web::WebAppState,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewRecord {
    pub key: String,
    pub value: String,
}

// #[axum::debug_handler]
pub async fn set_record_handler<T, U, V>(
    State(state): State<WebAppState<T, U, V>>,
    Path(store): Path<String>,
    Json(new_record): Json<NewRecord>,
) -> Response
where
    T: HashIndexRepo,
    U: RecordRepo,
{
    match SetRecordUseCase::new(state.record_repo).execute(&new_record, &store) {
        Ok(offset) => {
            SetHasIndexKey::new(state.hash_index)
                .execute(&new_record.key, offset)
                .expect("failed to set hash index key");
            Json(new_record).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

// #[axum::debug_handler]
pub async fn get_record_handler<T, U, V>(
    State(state): State<WebAppState<T, U, V>>,
    Path((store, key)): Path<(String, String)>,
) -> Response
where
    T: HashIndexRepo,
    U: RecordRepo,
{
    // getting the record offset from the hashindex
    let offset = match GetHasIndexKey::new(state.hash_index).execute(&key) {
        None => {
            return (
                StatusCode::NOT_FOUND,
                "failed to find the record in the hash index!",
            )
                .into_response()
        }
        Some(offset) => offset,
    };

    // reading the record from the store file using the offset
    match GetRecordUseCase::new(state.record_repo).execute(offset, &store) {
        Ok(Some(record)) => Json(NewRecord {
            key: record.key,
            value: record.value,
        })
        .into_response(),
        Ok(None) => (StatusCode::NOT_FOUND, "record not found!").into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}
