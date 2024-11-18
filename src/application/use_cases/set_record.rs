use crate::domain::{
    entities::record::Record, repositories::record_repo::RecordRepo,
    services::record_service::RecordService,
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

    pub fn execute(&self, record: &Record, store: &str) -> Result<(), String> {
        self.service.set(record, store)
    }
}
