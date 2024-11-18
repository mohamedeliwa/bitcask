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
    fn create(&self, store: &NewStore) -> Result<(), String> {
        let path = format!("{}/{}", self.path, store.name);
        match fs::File::create_new(&path) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get_by_id(&self, id: &str) -> Option<Store> {
        let path = format!("{}/{}", self.path, id);
        match fs::File::open(&path) {
            Ok(_) => Some(Store {
                name: id.into(),
                id: id.into(),
            }),
            Err(e) => None,
        }
    }
}
