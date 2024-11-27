use crate::{
    domain::{
        entities::hash_index::Offset, repositories::record_repo::RecordRepo,
        services::record_service::RecordService,
    },
    presentation::handlers::record_handler::NewRecord,
};

pub struct SetRecordUseCase<T: RecordRepo> {
    service: RecordService<T>,
}

impl<T: RecordRepo> SetRecordUseCase<T> {
    pub fn new(repo: T) -> Self {
        SetRecordUseCase {
            service: RecordService::new(repo),
        }
    }

    pub fn execute(&self, record: &NewRecord, store: &str) -> Result<Offset, String> {
        self.service.set(record, store)
    }
}
