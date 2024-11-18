use crate::{domain::entities::record::Record, presentation::handlers::record_handler::NewRecord};

pub trait RecordRepo {
    fn set(&self, record: &NewRecord, store: &str) -> Result<(), String>;
    fn get(&self, key: &str, store: &str) -> Option<Record>;
}
