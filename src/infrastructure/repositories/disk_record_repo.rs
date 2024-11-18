use std::{fs::OpenOptions, io::Write, sync::Arc};

use crate::{
    domain::repositories::record_repo::RecordRepo,
    presentation::handlers::record_handler::NewRecord,
};

pub struct DiskRecordRepo {
    /// path of the directory in which stores are created
    path: String,
}

impl DiskRecordRepo {
    pub fn new(path: &str) -> Self {
        DiskRecordRepo { path: path.into() }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl RecordRepo for Arc<DiskRecordRepo> {
    fn set(&self, record: &NewRecord, store: &str) -> Result<(), String> {
        let path = format!("{}/{}", self.path, store);
        // check if the file exist in the path
        let mut store_file = match OpenOptions::new().append(true).open(&path) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };
        // if yes, append the record to the file
        write!(store_file, "{}", record.key).map_err(|e| e.to_string())?;
        write!(store_file, ",").map_err(|e| e.to_string())?;
        write!(store_file, "{}", record.value).map_err(|e| e.to_string())?;
        write!(store_file, "\n").map_err(|e| e.to_string())?;
        Ok(())
    }
    fn get(&self, _key: &str, _store: &str) -> Option<crate::domain::entities::record::Record> {
        None
    }
}
