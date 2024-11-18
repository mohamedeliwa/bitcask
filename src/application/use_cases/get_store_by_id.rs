use crate::domain::{
    entities::store::Store, repositories::store_repo::StoreRepo,
    services::store_service::StoreService,
};

pub struct GetStoreByIdUseCase<T: StoreRepo> {
    service: StoreService<T>,
}

impl<T: StoreRepo> GetStoreByIdUseCase<T> {
    pub fn new(repo: T) -> Self {
        GetStoreByIdUseCase {
            service: StoreService::new(repo),
        }
    }

    pub fn execute(&self, id: String) -> Option<Store> {
        self.service.get_by_id(id)
    }
}
