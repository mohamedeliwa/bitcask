use crate::domain::{entities::record::Record, repositories::record_repo::RecordRepo};

pub struct RecordService<T: RecordRepo> {
    repo: T,
}

impl<T: RecordRepo> RecordService<T> {
    pub fn new(repo: T) -> Self {
        RecordService { repo }
    }

    pub fn set(&self, record: &Record, store: &str) -> Result<(), String> {
        self.repo.set(record, store)
    }

    pub fn get(&self, key: &str, store: &str) -> Option<Record> {
        self.repo.get(key, store)
    }
}
