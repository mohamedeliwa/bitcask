use crate::domain::{
    entities::{hash_index::Offset, record::Record},
    repositories::record_repo::RecordRepo,
    services::record_service::RecordService,
};

pub struct GetRecordUseCase<T: RecordRepo> {
    service: RecordService<T>,
}

impl<T: RecordRepo> GetRecordUseCase<T> {
    pub fn new(repo: T) -> Self {
        GetRecordUseCase {
            service: RecordService::new(repo),
        }
    }

    pub fn execute(&self, offset: Offset, store: &str) -> Result<Option<Record>, String> {
        self.service.get(offset, store)
    }
}
