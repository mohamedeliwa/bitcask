use std::{fs, sync::Arc};

use crate::{
    domain::{entities::store::Store, repositories::store_repo::StoreRepo},
    presentation::handlers::store_handler::NewStore,
};

pub struct DiskStoreRepo {
    /// path of the directory in which stores are created
    path: String,
}

impl DiskStoreRepo {
    pub fn new(path: &str) -> Self {
        // making sure the stores directory exists
        fs::create_dir_all(path).expect("couldn't create Disk store repo");

        DiskStoreRepo {
            path: path.to_string(),
        }
    }
}

impl DiskStoreRepo {
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl StoreRepo for Arc<DiskStoreRepo> {
    fn create(&self, _store: &NewStore) -> Result<(), String> {
        Ok(())
    }

    fn get_by_id(&self, _id: String) -> Option<Store> {
        None
    }
}
