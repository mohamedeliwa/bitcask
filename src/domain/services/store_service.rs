use crate::{
    domain::{entities::store::Store, repositories::store_repo::StoreRepo},
    presentation::handlers::store_handler::NewStore,
};

pub struct StoreService<T: StoreRepo> {
    repo: T,
}

impl<T: StoreRepo> StoreService<T> {
    pub fn new(repo: T) -> Self {
        StoreService { repo }
    }

    pub fn create_store(&self, store: &NewStore) -> Result<(), String> {
        self.repo.create(store)
    }

    pub fn get_by_id(&self, id: &str) -> Option<Store> {
        self.repo.get_by_id(id)
    }
}
