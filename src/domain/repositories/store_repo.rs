use crate::{domain::entities::store::Store, presentation::handlers::store_handler::NewStore};

pub trait StoreRepo {
    fn create(&self, store: &NewStore) -> Result<(), String>;
    fn get_by_id(&self, id: String) -> Option<Store>;
}
