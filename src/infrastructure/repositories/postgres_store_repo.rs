use crate::{
    domain::{entities::store::Store, repositories::store_repo::StoreRepo},
    infrastructure::db::connection::{establish_connection, DBPool},
    presentation::handlers::store_handler::NewStore,
};

#[derive(Clone)]
pub struct PostgresStoreRepo {
    #[allow(dead_code)]
    pool: DBPool,
}

impl PostgresStoreRepo {
    pub fn new() -> Self {
        let db_url = "";
        let pool = establish_connection(db_url);
        PostgresStoreRepo { pool }
    }
}

// we can implement it for Arc<> in case we want it to be shared
// impl StoreRepo for Arc<PostgresStoreRepo>
impl StoreRepo for PostgresStoreRepo {
    fn create(&self, _store: &NewStore) -> Result<(), String> {
        Ok(())
    }
    fn find_by_id(&self, _id: String) -> Option<Store> {
        None
    }
}
