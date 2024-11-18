use crate::{
    domain::{repositories::store_repo::StoreRepo, services::store_service::StoreService},
    presentation::handlers::store_handler::NewStore,
};

pub struct CreateStoreUseCase<T: StoreRepo> {
    service: StoreService<T>,
}

impl<T: StoreRepo> CreateStoreUseCase<T> {
    pub fn new(repo: T) -> Self {
        CreateStoreUseCase {
            service: StoreService::new(repo),
        }
    }

    pub fn execute(&self, store: &NewStore) -> Result<(), String> {
        self.service.create_store(store)
    }
}
