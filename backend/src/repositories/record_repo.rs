use entity::{grip_types, measurement_points, records};
use rocket::async_trait;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DeleteResult, EntityTrait};

use crate::{
    commands::create_record::CreateRecord,
    errors::errors::RepositoryError,
    repositories::{crud_repo::CrudRepo, measurement_point_repo::MeasurementPointRepo},
    structs::record::Record,
};

pub struct RecordRepo {
    db: DatabaseConnection,
    measurement_point_repo: MeasurementPointRepo,
}

#[async_trait]
impl CrudRepo<Record, CreateRecord, i32> for RecordRepo {
    async fn find_by_id(&self, id: i32) -> Result<Record, RepositoryError> {
        let record_with_measurement_points: Vec<(records::Model, Vec<measurement_points::Model>)> =
            records::Entity::find_by_id(id)
                .find_with_related(measurement_points::Entity)
                .all(&self.db)
                .await?;
        let (record_model, points_models): (records::Model, Vec<measurement_points::Model>) =
            match record_with_measurement_points.first() {
                Some(tuple) => tuple.clone(),
                None => return Err(RepositoryError::NotFound),
            };
        let record_with_grip_type: Vec<(records::Model, Option<grip_types::Model>)> =
            records::Entity::find_by_id(id)
                .find_also_related(grip_types::Entity)
                .all(&self.db)
                .await?;
        let grip_type: grip_types::Model = match record_with_grip_type.first() {
            Some(tuple) => match tuple.1.clone() {
                Some(grip_type) => grip_type,
                None => return Err(RepositoryError::NotFound),
            },
            None => return Err(RepositoryError::NotFound),
        };
        Ok(Record::from((record_model, points_models, grip_type)))
    }

    async fn delete_by_id(&self, id: i32) -> Result<(), RepositoryError> {
        let res: DeleteResult = records::Entity::delete_by_id(id).exec(&self.db).await?;
        if res.rows_affected == 1 {
            Ok(())
        } else {
            Err(RepositoryError::NotFound)
        }
    }

    async fn insert(&self, new_record: CreateRecord) -> Result<Record, RepositoryError> {
        let record = records::ActiveModel {
            used_grip_type_id: Set(new_record.used_grip_type.id),
            duration: Set(new_record.duration),
            ..Default::default()
        };
        let record_model: records::Model = record.insert(&self.db).await?;
        self.measurement_point_repo
            .insert_many(new_record.measurement_points, record_model.record_id)
            .await?;
        Ok(self.find_by_id(record_model.record_id).await?)
    }
}
