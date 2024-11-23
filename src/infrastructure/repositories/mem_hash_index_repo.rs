use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::domain::{
    entities::hash_index::{HashIndex, Offset},
    repositories::hash_index_repo::HashIndexRepo,
};

pub struct MemHashIndexRepo(Mutex<HashIndex>);

impl MemHashIndexRepo {
    pub fn new() -> Self {
        MemHashIndexRepo(Mutex::new(HashIndex {
            index: HashMap::new(),
        }))
    }
}

impl HashIndexRepo for Arc<MemHashIndexRepo> {
    fn get(&self, key: &str) -> Option<Offset> {
        self.0.lock().unwrap().index.get(key).copied()
    }
    fn set(&self, key: &str, offset: Offset) -> Result<(), String> {
        self.0.lock().unwrap().index.insert(key.into(), offset);
        Ok(())
    }
}
