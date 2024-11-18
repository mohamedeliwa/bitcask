use crate::domain::entities::record::Record;

pub trait RecordRepo {
    fn set(&self, record: &Record, store: &str) -> Result<(), String>;
    fn get(&self, key: &str, store: &str) -> Option<Record>;
}
