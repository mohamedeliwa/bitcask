use std::{fs, path::PathBuf, sync::Arc};

use crate::{
    domain::{entities::store::Store, repositories::store_repo::StoreRepo},
    presentation::handlers::store_handler::NewStore,
};

pub struct DiskStoreRepo {
    /// path of the directory in which stores are created
    path: String,
    log_file_name: String,
    index_file_name: String,
}

impl DiskStoreRepo {
    pub fn new(path: &str) -> Self {
        // making sure the stores directory exists
        fs::create_dir_all(path).expect("couldn't create Disk store repo");

        DiskStoreRepo {
            path: path.to_string(),
            log_file_name: "log".into(),
            index_file_name: "index.json".into(),
        }
    }
}

impl DiskStoreRepo {
    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl StoreRepo for Arc<DiskStoreRepo> {
    /// creates the store directory structure
    /// each store consits of a directory and two files
    /// and one for storing the data called log
    /// and one for storing the hash index called index
    fn create(&self, store: &NewStore) -> Result<(), String> {
        let mut path = PathBuf::from(&self.path);
        path.push(&store.id);

        // creating the directory container for the new store
        fs::create_dir_all(&path).expect("couldn't create store directory");

        let log_file = path.join(&self.log_file_name);
        let index_file = path.join(&self.index_file_name);

        match fs::File::create_new(&log_file) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        };
        match fs::File::create_new(&index_file) {
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
            Err(_) => None,
        }
    }
}
