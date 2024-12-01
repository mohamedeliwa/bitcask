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
    domain::repositories::store_repo::StoreRepo,
    infrastructure::web::WebAppState,
};

#[derive(Serialize, Deserialize)]
pub struct NewStore {
    pub id: String,
    pub name: String,
}

// #[axum::debug_handler]
pub async fn create_store_handler<T, U, V>(
    State(state): State<WebAppState<T, U, V>>,
    Json(input): Json<NewStore>,
) -> Response
where
    V: StoreRepo,
{
    match CreateStoreUseCase::new(state.store_repo).execute(&input) {
        Ok(_) => Json(input).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

// #[axum::debug_handler]
pub async fn get_store_by_id_handler<T, U, V>(
    State(repo): State<WebAppState<T, U, V>>,
    Path(id): Path<String>,
) -> Response
where
    V: StoreRepo,
{
    if let Some(store) = GetStoreByIdUseCase::new(repo.store_repo).execute(&id) {
        Json(NewStore {
            id: store.id,
            name: store.name,
        })
        .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
