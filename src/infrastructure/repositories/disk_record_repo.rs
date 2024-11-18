use crate::domain::repositories::record_repo::RecordRepo;

pub struct DiskRecordRepo {
    /// path of the directory in which stores are created
    path: String,
}

impl DiskRecordRepo {
    pub fn new() -> Self {
        // can be read from the env
        // or passed as a param from the caller
        let path = "".to_string();
        DiskRecordRepo { path }
    }
}

/// we can implement it for Arc<> in case we want it to be shared
/// impl StoreRepo for Arc<DiskStoreRepo>
impl RecordRepo for DiskRecordRepo {
    fn set(
        &self,
        _record: &crate::domain::entities::record::Record,
        _store: &str,
    ) -> Result<(), String> {
        Ok(())
    }
    fn get(&self, _key: &str, _store: &str) -> Option<crate::domain::entities::record::Record> {
        None
    }
}
