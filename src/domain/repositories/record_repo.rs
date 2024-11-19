use crate::{domain::entities::record::Record, presentation::handlers::record_handler::NewRecord};

pub trait RecordRepo {
    /// sets a record key/value pair
    fn set(&self, record: &NewRecord, store: &str) -> Result<(), String>;
    /// gets a record by a key
    fn get(&self, key: &str, store: &str) -> Result<Option<Record>, String>;
}
