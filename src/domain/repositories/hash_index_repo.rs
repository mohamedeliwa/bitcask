use crate::domain::entities::hash_index::Offset;

pub trait HashIndexRepo {
    /// sets the offset of a record in the database file by its key
    fn set(&self, key: &str, offset: Offset) -> Result<(), String>;
    /// gets the offset of a record in the databse file by its key
    fn get(&self, key: &str) -> Option<Offset>;
}
