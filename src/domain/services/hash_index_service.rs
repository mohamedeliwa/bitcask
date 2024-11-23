use crate::domain::{entities::hash_index::Offset, repositories::hash_index_repo::HashIndexRepo};

pub struct HashIndexService<T: HashIndexRepo> {
    repo: T,
}

impl<T: HashIndexRepo> HashIndexService<T> {
    pub fn new(repo: T) -> Self {
        HashIndexService { repo }
    }

    pub fn set(&self, key: &str, offset: Offset) -> Result<(), String> {
        self.repo.set(key, offset)
    }

    pub fn get(&self, key: &str) -> Option<Offset> {
        self.repo.get(key)
    }
}
