use std::sync::Arc;

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
        // can be read from the env
        // or passed as a param from the caller
        DiskRecordRepo { path: path.into() }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl RecordRepo for Arc<DiskRecordRepo> {
    fn set(&self, _record: &NewRecord, _store: &str) -> Result<(), String> {
        Ok(())
    }
    fn get(&self, _key: &str, _store: &str) -> Option<crate::domain::entities::record::Record> {
        None
    }
}
