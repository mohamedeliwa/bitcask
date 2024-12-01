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
        set_record::SetRecordUseCase,
    },
    domain::repositories::{
        hash_index_repo::HashIndexRepo, record_repo::RecordRepo, store_repo::StoreRepo,
    },
    infrastructure::web::WebAppState,
};

#[derive(Serialize, Deserialize)]
pub struct NewRecord {
    pub key: String,
    pub value: String,
}

// #[axum::debug_handler]
pub async fn set_record_handler<H, R, S>(
    State(state): State<WebAppState<H, R, S>>,
    Path(store): Path<String>,
    Json(new_record): Json<NewRecord>,
) -> Response
where
    H: HashIndexRepo,
    R: RecordRepo,
    S: StoreRepo,
{
    match SetRecordUseCase::new(state.record_repo, state.store_repo, state.hash_index_repo)
        .execute(&new_record, &store)
    {
        Ok(_) => Json(new_record).into_response(),
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
    let offset = match GetHasIndexKey::new(state.hash_index_repo).execute(&key) {
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
