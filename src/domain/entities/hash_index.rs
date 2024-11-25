use std::collections::HashMap;

/// keeps an in-memory hash map where every key is mapped to a byte offset
/// in the data file -the location at which the value can be found-,
/// Whenever you append a new key-value pair to the file, you also update
/// the hash map to reflect the offset of the data you just wrote
/// (this works both for inserting new keys and for updating existing keys).
/// When you want to look up a value, use the hash map to find the offset in
/// the data file, seek to that location, and read the value.
pub struct HashIndex {
    pub index: HashMap<String, Offset>,
}

#[derive(Debug, Clone, Copy)]
/// bytes offset in a file
pub struct Offset(pub u64);
