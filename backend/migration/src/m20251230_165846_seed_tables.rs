use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::{
    ActiveModelTrait,
    ColumnTrait,
    EntityTrait,
    QueryFilter,
    Set,
};

use ::entity::climbing_grades;
use ::entity::profile_pics;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let seed_data = vec![
            ("4A", "Boulder Fontaineblau", 400),
            ("4A+", "Boulder Fontaineblau", 417),
            ("4B", "Boulder Fontaineblau", 433),
            ("4B+", "Boulder Fontaineblau", 450),
            ("4C", "Boulder Fontaineblau", 467),
            ("4C+", "Boulder Fontaineblau", 483),
            ("5A", "Boulder Fontaineblau", 500),
            ("5A+", "Boulder Fontaineblau", 517),
            ("5B", "Boulder Fontaineblau", 533),
            ("5B+", "Boulder Fontaineblau", 550),
            ("5C", "Boulder Fontaineblau", 567),
            ("5C+", "Boulder Fontaineblau", 583),
            ("6A", "Boulder Fontaineblau", 600),
            ("6A+", "Boulder Fontaineblau", 617),
            ("6B", "Boulder Fontaineblau", 633),
            ("6B+", "Boulder Fontaineblau", 650),
            ("6C", "Boulder Fontaineblau", 667),
            ("6C+", "Boulder Fontaineblau", 683),
            ("7A", "Boulder Fontaineblau", 700),
            ("7A+", "Boulder Fontaineblau", 717),
            ("7B", "Boulder Fontaineblau", 733),
            ("7B+", "Boulder Fontaineblau", 750),
            ("7C", "Boulder Fontaineblau", 767),
            ("7C+", "Boulder Fontaineblau", 783),
            ("8A", "Boulder Fontaineblau", 800),
            ("8A+", "Boulder Fontaineblau", 817),
            ("8B", "Boulder Fontaineblau", 833),
            ("8B+", "Boulder Fontaineblau", 850),
            ("8C", "Boulder Fontaineblau", 867),
            ("8C+", "Boulder Fontaineblau", 883),
            ("9A", "Boulder Fontaineblau", 900),
        ];

        for (name, grade_context, numerical_value) in seed_data {
            let model = climbing_grades::ActiveModel {
                name: Set(name.to_string()),
                grade_context: Set(grade_context.to_string()),
                numverical_value: Set(numerical_value),
                ..Default::default()
            };
            model.insert(db).await?;
        }

        println!("ClimbingGrades Table seeded successfully.");

        let seed_data_profile_pic = vec![
            ("https:://127.0.0.1:8000/assets/default-user.svg")
        ];

        for uri in seed_data_profile_pic {
            let model = profile_pics::ActiveModel {
                url: Set(uri.to_string()),
                ..Default::default()
            };
            model.insert(db).await?;
        }
        println!("ProfilePics Table seeded successfully.");
        
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        let titles_to_delete = vec![
            "4A", "4A+", "4B", "4B+", "4C", "4C+",
            "5A", "5A+", "5B", "5B+", "5C", "5C+",
            "6A", "6A+", "6B", "6B+", "6C", "6C+",
            "7A", "7A+", "7B", "7B+", "7C", "7C+",
            "8A", "8A+", "8B", "8B+", "8C", "8C+",
            "9A",
        ];
        climbing_grades::Entity::delete_many()
            .filter(climbing_grades::Column::Name.is_in(titles_to_delete))
            .exec(db)
            .await?;

        println!("ClimbingGrades seeded data removed.");

        let profile_pics_to_delete = vec![
            ("https:://127.0.0.1:8000/assets/default-user.svg")
        ];

        profile_pics::Entity::delete_many()
            .filter(profile_pics::Column::Url.is_in(profile_pics_to_delete))
            .exec(db)
            .await?;

        println!("ProfilePics seeded data removed.");

        Ok(())
    }
}