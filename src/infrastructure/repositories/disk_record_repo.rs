use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Seek, SeekFrom, Write},
    sync::Arc,
};

use crate::{
    domain::{
        entities::{hash_index::Offset, record::Record},
        repositories::record_repo::RecordRepo,
    },
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

    pub fn get_store_path(&self, store: &str) -> String {
        let path = format!("{}/{}", self.path, store);
        path
    }

    pub fn get_log_file_path(&self, store: &str) -> String {
        let path = self.get_store_path(store);
        let path = format!("{}/log", path);
        path
    }
}

impl RecordRepo for Arc<DiskRecordRepo> {
    fn set(&self, record: &NewRecord, store: &str) -> Result<Offset, String> {
        let path = self.get_log_file_path(store);
        // check if the file exist in the path
        let mut store_file = match OpenOptions::new().append(true).open(&path) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };
        let offset = store_file
            .seek(SeekFrom::End(0))
            .map_err(|e| e.to_string())?;
        // if yes, append the record to the file
        // the key and value of the same record are separated by a comma
        // records are separated by a new line
        write!(store_file, "{},{}\n", record.key, record.value).map_err(|e| e.to_string())?;
        Ok(Offset(offset))
    }

    fn get(&self, offset: Offset, store: &str) -> Result<Option<Record>, String> {
        let path = self.get_log_file_path(store);

        // check if the file exist in the path
        let mut store_file = match OpenOptions::new().read(true).open(&path) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        // seeking the record position in the store file
        store_file
            .seek(SeekFrom::Start(offset.0))
            .map_err(|e| e.to_string())?;

        let mut lines = BufReader::new(store_file).lines();

        let line = lines.next();
        if line.is_none() {
            return Err("failed to read record, bcz the store end is reached".into());
        }

        let line = line.unwrap().map_err(|e| e.to_string())?;
        if let Some((key, value)) = line.split_once(',') {
            return Ok(Some(Record {
                key: key.into(),
                value: value.into(),
            }));
        } else {
            return Err("failed to read record".into());
        };
    }
}
