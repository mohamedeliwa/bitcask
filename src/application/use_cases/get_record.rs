use crate::domain::{
    entities::record::Record, repositories::record_repo::RecordRepo,
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

    pub fn execute(&self, record: &str, store: &str) -> Option<Record> {
        self.service.get(record, store)
    }
}
