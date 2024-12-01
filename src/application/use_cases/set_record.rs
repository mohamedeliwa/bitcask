use crate::{
    domain::{
        entities::hash_index::Offset,
        repositories::{
            hash_index_repo::HashIndexRepo, record_repo::RecordRepo, store_repo::StoreRepo,
        },
        services::{
            hash_index_service::HashIndexService, record_service::RecordService,
            store_service::StoreService,
        },
    },
    presentation::handlers::record_handler::NewRecord,
};

const MAX_OFFSET: Offset = Offset(20);

/// Updates a record's value
///
/// By appending the record to the log file
///
/// And updating the (key, offset) pair in the hash index
pub struct SetRecordUseCase<T: RecordRepo, S: StoreRepo, H: HashIndexRepo> {
    record_service: RecordService<T>,
    store_service: StoreService<S>,
    hash_index_service: HashIndexService<H>,
}

impl<T: RecordRepo, S: StoreRepo, H: HashIndexRepo> SetRecordUseCase<T, S, H> {
    pub fn new(record_repo: T, store_repo: S, hash_index_repo: H) -> Self {
        SetRecordUseCase {
            record_service: RecordService::new(record_repo),
            store_service: StoreService::new(store_repo),
            hash_index_service: HashIndexService::new(hash_index_repo),
        }
    }

    pub fn execute(&self, record: &NewRecord, store: &str) -> Result<(), String> {
        let offset = self.record_service.set(record, store)?;
        if offset.0 < MAX_OFFSET.0 {
            // the log is not ready yet to be splitted

            // update the offset of the key in the log
            self.hash_index_service.set(&record.key, offset)?;
            return Ok(());
        } else {
            // if the offset exceeded a max limit
            // log splitting sprocess is triggered
            let closed_segment_name = self.store_service.split_log(store)?;

            // update the key offset before persisting the current hash index
            self.hash_index_service.set(&record.key, offset)?;

            // then hash persistence is trigered
            self.hash_index_service
                .persist(store, &closed_segment_name)?;
            Ok(())
        }
    }
}
