use crate::{
    commands::create_record::CreateRecord,
    errors::errors::RepositoryError,
    repositories::{crud_repo::CrudRepo, record_repo::RecordRepo},
    structs::record::Record,
};

pub struct RecordService {
    repo: RecordRepo,
}

impl RecordService {
    pub async fn create_record(&self, new_record: CreateRecord) -> Result<Record, RepositoryError> {
        self.repo.insert(new_record).await
    }
}
