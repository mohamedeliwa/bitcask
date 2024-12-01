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

    /// persists the in-memory index of a store to disk.
    ///
    /// # Arguments
    /// * `store` - The store id.
    /// * `index_name` - The name for the index file to be created, it's named after the closed log segment.
    ///
    pub fn persist(&self, store: &str, index_name: &str) -> Result<(), String> {
        self.repo.persist(store, index_name)
    }
}
