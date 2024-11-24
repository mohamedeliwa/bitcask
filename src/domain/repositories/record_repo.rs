use crate::{
    domain::entities::{hash_index::Offset, record::Record},
    presentation::handlers::record_handler::NewRecord,
};

pub trait RecordRepo {
    /// sets a record key/value pair
    fn set(&self, record: &NewRecord, store: &str) -> Result<Offset, String>;
    /// gets a record by a key
    fn get(&self, offset: Offset, store: &str) -> Result<Option<Record>, String>;
}
