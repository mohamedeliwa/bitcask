use crate::{domain::entities::store::Store, presentation::handlers::store_handler::NewStore};

pub trait StoreRepo {
    fn create(&self, store: &NewStore) -> Result<(), String>;
    fn get_by_id(&self, id: &str) -> Option<Store>;
    fn close_log(&self, id: &str) -> Result<String, String>;
}
