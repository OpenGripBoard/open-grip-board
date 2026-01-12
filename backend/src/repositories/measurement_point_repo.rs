use entity::measurement_points::{self, ActiveModel};
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait};

use crate::{errors::RepositoryError, structs::measurement_point::MeasurementPoint};

pub struct MeasurementPointRepo {
    db: DatabaseConnection,
}

impl MeasurementPointRepo {
    #[allow(dead_code)]
    pub async fn insert_one(
        &self,
        new_point: MeasurementPoint,
        record_id: i32,
    ) -> Result<(), RepositoryError> {
        #[allow(clippy::needless_update)]
        let point = measurement_points::ActiveModel {
            measurement_point_timestamp: Set(new_point.timestamp),
            value: Set(new_point.value),
            record_id: Set(record_id),
            ..Default::default()
        };
        point.insert(&self.db).await?;
        Ok(())
    }

    pub async fn insert_many(
        &self,
        new_points: Vec<MeasurementPoint>,
        record_id: i32,
    ) -> Result<(), RepositoryError> {
        #[allow(clippy::needless_update)]
        let points: Vec<ActiveModel> = new_points
            .into_iter()
            .map(|new_point| measurement_points::ActiveModel {
                measurement_point_timestamp: Set(new_point.timestamp),
                value: Set(new_point.value),
                record_id: Set(record_id),
                ..Default::default()
            })
            .collect();
        measurement_points::Entity::insert_many(points)
            .exec(&self.db)
            .await?;
        Ok(())
    }
}
