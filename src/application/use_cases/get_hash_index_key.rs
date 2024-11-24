use crate::domain::{
    entities::hash_index::Offset, repositories::hash_index_repo::HashIndexRepo,
    services::hash_index_service::HashIndexService,
};

pub struct GetHasIndexKey<T: HashIndexRepo> {
    service: HashIndexService<T>,
}

impl<T: HashIndexRepo> GetHasIndexKey<T> {
    pub fn new(repo: T) -> Self {
        GetHasIndexKey {
            service: HashIndexService::new(repo),
        }
    }

    pub fn execute(&self, key: &str) -> Option<Offset> {
        self.service.get(key)
    }
}
