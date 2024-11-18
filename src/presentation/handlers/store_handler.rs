use crate::{
    application::use_cases::{
        create_store::CreateStoreUseCase, get_store_by_id::GetStoreByIdUseCase,
    },
    domain::entities::store::Store,
    infrastructure::repositories::disk_store_repo::DiskStoreRepo,
};

#[derive(Clone, Debug)]
pub struct NewStore {
    pub id: String,
    pub name: String,
}

// should return http response, if it's a web handler
pub fn create_store_handler(repo: DiskStoreRepo, input: NewStore) -> Result<(), String> {
    CreateStoreUseCase::new(repo).execute(input)
}

pub fn get_store_by_id_handler(repo: DiskStoreRepo, id: String) -> Option<Store> {
    GetStoreByIdUseCase::new(repo).execute(id)
}
