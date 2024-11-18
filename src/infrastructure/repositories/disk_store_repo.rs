use crate::{
    domain::{entities::store::Store, repositories::store_repo::StoreRepo},
    presentation::handlers::store_handler::NewStore,
};

pub struct DiskStoreRepo {
    /// path of the directory in which stores are created
    path: String,
}

impl DiskStoreRepo {
    pub fn new() -> Self {
        // can be read from the env
        // or passed as a param from the caller
        let path = "".to_string();
        DiskStoreRepo { path }
    }
}

/// we can implement it for Arc<> in case we want it to be shared
/// impl StoreRepo for Arc<DiskStoreRepo>
impl StoreRepo for DiskStoreRepo {
    fn create(&self, _store: &NewStore) -> Result<(), String> {
        Ok(())
    }
    fn get_by_id(&self, _id: String) -> Option<Store> {
        None
    }
}
