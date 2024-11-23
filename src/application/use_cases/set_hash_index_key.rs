use crate::domain::{
    entities::hash_index::Offset, repositories::hash_index_repo::HashIndexRepo, services::hash_index_service::HashIndexService
};

pub struct SetHasIndexKey<T: HashIndexRepo> {
    service: HashIndexService<T>,
}

impl<T: HashIndexRepo> SetHasIndexKey<T> {
    pub fn new(service: HashIndexService<T>) -> Self {
        SetHasIndexKey { service }
    }

    pub fn execute(&self, key: &str, offset: Offset) -> Result<(), String> {
        self.service.set(key, offset)
    }
}
