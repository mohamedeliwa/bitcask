use std::{
    fs::{self, rename, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    domain::{entities::store::Store, repositories::store_repo::StoreRepo},
    presentation::handlers::store_handler::NewStore,
};

pub struct DiskStoreRepo {
    /// path of the directory in which stores are created
    path: String,
    log_file_name: String,
    #[allow(dead_code)]
    index_file_name: String,
    meta_file_name: String,
}

impl DiskStoreRepo {
    pub fn new(path: &str) -> Self {
        // making sure the stores directory exists
        fs::create_dir_all(path).expect("couldn't create Disk store repo");

        DiskStoreRepo {
            path: path.to_string(),
            log_file_name: "log".into(),
            index_file_name: "index".into(),
            meta_file_name: "meta".into(),
        }
    }
}

impl DiskStoreRepo {
    pub fn get_path(&self) -> PathBuf {
        PathBuf::from(&self.path)
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
        let meta_file = path.join(&self.meta_file_name);
        // let index_file = path.join(&self.index_file_name);

        match fs::File::create_new(&log_file) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
        // match fs::File::create_new(&index_file) {
        //     Ok(_) => Ok(()),
        //     Err(e) => Err(e.to_string()),
        // }
        match fs::File::create_new(&meta_file) {
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

    /// closes the current log segment
    /// opens a new log segment
    /// returns the timestamp string after which the old log segment is named
    fn split_log(&self, id: &str) -> Result<String, String> {
        let store_path = self.get_path().join(id);
        let log_path = store_path.join(&self.log_file_name);
        // getting the current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();

        let new_log_file_name = format!("{timestamp}.log");
        let new_log_path = store_path.join(&new_log_file_name);
        let meta_path = store_path.join(&self.meta_file_name);

        // rename the old log after the timestamp
        rename(&log_path, &new_log_path).map_err(|e| e.to_string())?;
        // append the timestamp to the meta file
        let mut meta_file = OpenOptions::new()
            .append(true)
            .open(meta_path)
            .map_err(|e| e.to_string())?;
        meta_file
            .write_all(timestamp.as_bytes())
            .map_err(|e| e.to_string())?;
        // create a new log file
        match fs::File::create_new(&log_path) {
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
        Ok(timestamp)
    }
}
