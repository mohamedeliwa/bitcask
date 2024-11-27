use std::{
    collections::HashMap,
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use crate::domain::{
    entities::hash_index::{HashIndex, Offset},
    repositories::hash_index_repo::HashIndexRepo,
};

pub struct MemHashIndexRepo {
    path: String,
    index: Mutex<HashIndex>,
}

impl MemHashIndexRepo {
    pub fn new(path: &str) -> Self {
        MemHashIndexRepo {
            index: Mutex::new(HashIndex {
                index: HashMap::new(),
            }),
            path: path.into(),
        }
    }
}

impl HashIndexRepo for Arc<MemHashIndexRepo> {
    fn get(&self, key: &str) -> Option<Offset> {
        self.index.lock().unwrap().index.get(key).copied()
    }
    fn set(&self, key: &str, offset: Offset) -> Result<(), String> {
        self.index.lock().unwrap().index.insert(key.into(), offset);
        Ok(())
    }

    /// persists the in-memory index of a store to disk.
    ///
    /// # Arguments
    /// * `store` - The store id.
    /// * `index_name` - The name for the newly created index file.
    ///
    fn persist(&self, store: &str, index_name: &str) -> Result<(), String> {
        let index_path = format!("{store}/{index_name}.index");
        let index_path = PathBuf::from(&self.path).join(&index_path);
        // append the timestamp to the meta file
        let mut index_file = OpenOptions::new()
            .append(true)
            .open(index_path)
            .map_err(|e| e.to_string())?;
        // empty the cache
        let in_mem_index: Vec<(String, Offset)> = self.index.lock().unwrap().index.drain().collect();

        // write the cached value to it
        for (key, value) in in_mem_index {
            let record = format!("{key},{}\n", value.0);
            index_file
                .write_all(record.as_bytes())
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}
