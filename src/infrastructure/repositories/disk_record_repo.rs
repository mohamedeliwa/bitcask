use std::{
    fs::OpenOptions,
    io::{BufRead, BufReader, Write},
    sync::Arc,
};

use crate::{
    domain::{entities::record::Record, repositories::record_repo::RecordRepo},
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
}

impl RecordRepo for Arc<DiskRecordRepo> {
    fn set(&self, record: &NewRecord, store: &str) -> Result<(), String> {
        let path = self.get_store_path(store);
        // check if the file exist in the path
        let mut store_file = match OpenOptions::new().append(true).open(&path) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };
        // if yes, append the record to the file
        write!(store_file, "{}", record.key).map_err(|e| e.to_string())?;
        // the key and value of the same record are separated by a comma
        write!(store_file, ",").map_err(|e| e.to_string())?;
        write!(store_file, "{}", record.value).map_err(|e| e.to_string())?;
        // records are separated by a new line
        write!(store_file, "\n").map_err(|e| e.to_string())?;
        Ok(())
    }
    fn get(&self, search_key: &str, store: &str) -> Result<Option<Record>, String> {
        let path = self.get_store_path(store);

        // check if the file exist in the path
        let store_file = match OpenOptions::new().read(true).open(&path) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let lines = BufReader::new(store_file).lines();
        let mut record: Option<Record> = None;
        for line in lines {
            if line.is_err() {
                return Err("failed to read record".into());
            }
            if let Some((key, value)) = line.unwrap().split_once(',') {
                if key == search_key {
                    record = Some(Record {
                        key: key.into(),
                        value: value.into(),
                    });
                }
            } else {
                return Err("failed to read record".into());
            };
        }

        Ok(record)
    }
}
