use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ProfilePic::ProfilePics)
                    .if_not_exists()
                    .col(pk_auto(ProfilePic::ProfilePicID))
                    .col(string(ProfilePic::Url).not_null())
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(ClimbingGrade::ClimbingGrades)
                    .if_not_exists()
                    .col(pk_auto(ClimbingGrade::ClimbingGradeID))
                    .col(string(ClimbingGrade::Name).not_null())
                    .col(string(ClimbingGrade::GradeContext).not_null())
                    .col(integer(ClimbingGrade::NumvericalValue).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Climber::Climbers)
                    .if_not_exists()
                    .col(pk_auto(Climber::ClimberID))
                    .col(string(Climber::Email).not_null())
                    .col(string(Climber::PasswordHash))
                    .col(string(Climber::Username).not_null())
                    .col(string(Climber::Firstname))
                    .col(string(Climber::Lastname))
                    .col(integer(Climber::ProfilePicId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_climber_profile_pic")
                            .from(Climber::Climbers, Climber::ProfilePicId)
                            .to(ProfilePic::ProfilePics, ProfilePic::ProfilePicID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Climber::Weight))
                    .col(date(Climber::BirthDate))
                    .col(integer(Climber::LeadRpIndoor))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_climber_lead_rp_indoor")
                            .from(Climber::Climbers, Climber::LeadRpIndoor)
                            .to(ClimbingGrade::ClimbingGrades, ClimbingGrade::ClimbingGradeID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Climber::LeadRpOutdoor))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_climber_lead_rp_outdoor")
                            .from(Climber::Climbers, Climber::LeadRpOutdoor)
                            .to(ClimbingGrade::ClimbingGrades, ClimbingGrade::ClimbingGradeID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Climber::BoulderRpIndoor))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_climber_boulder_rp_indoor")
                            .from(Climber::Climbers, Climber::BoulderRpIndoor)
                            .to(ClimbingGrade::ClimbingGrades, ClimbingGrade::ClimbingGradeID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Climber::BoulderRpOutdoor))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_climber_boulder_rp_outdoor")
                            .from(Climber::Climbers, Climber::BoulderRpIndoor)
                            .to(ClimbingGrade::ClimbingGrades, ClimbingGrade::ClimbingGradeID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Climber::MaxForceRight))
                    .col(integer(Climber::MaxForceLeft))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Gym::Gyms)
                    .if_not_exists()
                    .col(pk_auto(Gym::GymID))
                    .col(string(Gym::Name).not_null())
                    .col(float(Gym::LocationX))
                    .col(float(Gym::LocationY))
                    .col(integer(Gym::AdminId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_climber_admin_id")
                            .from(Gym::Gyms, Gym::AdminId)
                            .to(Climber::Climbers, Climber::ClimberID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FavouriteGym::FavouriteGyms)
                    .if_not_exists()
                    .col(integer(FavouriteGym::ClimberID).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_favourite_gym_climber")
                            .from(FavouriteGym::FavouriteGyms, FavouriteGym::ClimberID)
                            .to(Climber::Climbers, Climber::ClimberID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(FavouriteGym::GymID).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_favourite_gym_gym")
                            .from(FavouriteGym::FavouriteGyms, FavouriteGym::GymID)
                            .to(Gym::Gyms, Gym::GymID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Hangboard::Hangboards)
                    .if_not_exists()
                    .col(pk_auto(Hangboard::HangboardID))
                    .col(integer(Hangboard::OwningGymId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_hangboard_gym_id")
                            .from(Hangboard::Hangboards, Hangboard::OwningGymId)
                            .to(Gym::Gyms, Gym::GymID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(string(Hangboard::Nickname).not_null())
                    .col(date_time(Hangboard::LastTimeOnline))
                    .col(date_time(Hangboard::LastTimeCalibrated))
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(GripType::GripTypes)
                    .if_not_exists()
                    .col(pk_auto(GripType::GripTypeID))
                    .col(string(GripType::Name).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(HangboardGripType::HangboardGripTypes)
                    .if_not_exists()
                    .col(integer(HangboardGripType::HangboardID).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_hangboard_grip_type_hangboard")
                            .from(HangboardGripType::HangboardGripTypes, HangboardGripType::HangboardID)
                            .to(Hangboard::Hangboards, Hangboard::HangboardID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(HangboardGripType::GripTypeID).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_hangboard_grip_type_grip_type")
                            .from(HangboardGripType::HangboardGripTypes, HangboardGripType::GripTypeID)
                            .to(GripType::GripTypes, GripType::GripTypeID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TrainingTemplate::TrainingTemplates)
                    .if_not_exists()
                    .col(pk_auto(TrainingTemplate::TrainingTemplateID))
                    .col(string(TrainingTemplate::Name).not_null())
                    .col(integer(TrainingTemplate::CreatorClimberId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_training_template_climber_id")
                            .from(TrainingTemplate::TrainingTemplates, TrainingTemplate::CreatorClimberId)
                            .to(Climber::Climbers, Climber::ClimberID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(TrainingTemplate::ClimbingGradeId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_training_template_climbing_grade_id")
                            .from(TrainingTemplate::TrainingTemplates, TrainingTemplate::ClimbingGradeId)
                            .to(ClimbingGrade::ClimbingGrades, ClimbingGrade::ClimbingGradeID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(time(TrainingTemplate::PredictedDuration).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FavouriteTraining::FavouriteTrainings)
                    .if_not_exists()
                    .col(integer(FavouriteTraining::ClimberID).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_favourite_training_climber")
                            .from(FavouriteTraining::FavouriteTrainings, FavouriteTraining::ClimberID)
                            .to(Climber::Climbers, Climber::ClimberID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(FavouriteTraining::TrainingTemplateID).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_favourite_training_training_template")
                            .from(FavouriteTraining::FavouriteTrainings, FavouriteTraining::TrainingTemplateID)
                            .to(TrainingTemplate::TrainingTemplates, TrainingTemplate::TrainingTemplateID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ExerciseTemplate::ExerciseTemplates)
                    .if_not_exists()
                    .col(pk_auto(ExerciseTemplate::ExerciseTemplateID))
                    .col(time(ExerciseTemplate::ActiveDuration).not_null())
                    .col(integer(ExerciseTemplate::ActiveForce).not_null())
                    .col(integer(ExerciseTemplate::GripTypeId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exercise_template_grip_type")
                            .from(ExerciseTemplate::ExerciseTemplates, ExerciseTemplate::GripTypeId)
                            .to(GripType::GripTypes, GripType::GripTypeID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(time(ExerciseTemplate::RestDuration).not_null())
                    .col(integer(ExerciseTemplate::RestForce).not_null())
                    .to_owned(),
            )
            .await?;
        
        manager
            .create_table(
                Table::create()
                    .table(TrainingExercise::TrainingExercises)
                    .if_not_exists()
                    .col(integer(TrainingExercise::TrainingTemplateID).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_training_exercise_training_template")
                            .from(TrainingExercise::TrainingExercises, TrainingExercise::TrainingTemplateID)
                            .to(TrainingTemplate::TrainingTemplates, TrainingTemplate::TrainingTemplateID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(TrainingExercise::ExerciseTemplateID).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_training_exercise_exercise_template")
                            .from(TrainingExercise::TrainingExercises, TrainingExercise::ExerciseTemplateID)
                            .to(ExerciseTemplate::ExerciseTemplates, ExerciseTemplate::ExerciseTemplateID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(TrainingExercise::Repetitions).not_null())
                    .col(integer(TrainingExercise::Position).not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TrainingRecord::TrainingRecords)
                    .if_not_exists()
                    .col(pk_auto(TrainingRecord::TrainingRecordID))
                    .col(integer(TrainingRecord::ClimberId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_training_record_climber")
                            .from(TrainingRecord::TrainingRecords, TrainingRecord::ClimberId)
                            .to(Climber::Climbers, Climber::ClimberID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(TrainingRecord::TrainingTemplateId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_training_record_training_template")
                            .from(TrainingRecord::TrainingRecords, TrainingRecord::TrainingTemplateId)
                            .to(TrainingTemplate::TrainingTemplates, TrainingTemplate::TrainingTemplateID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(time(TrainingRecord::TotalDuration))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(ExerciseRecord::ExerciseRecords)
                    .if_not_exists()
                    .col(pk_auto(ExerciseRecord::ExerciseRecordID))
                    .col(integer(ExerciseRecord::TrainingRecordId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exercise_record_training_record")
                            .from(ExerciseRecord::ExerciseRecords, ExerciseRecord::TrainingRecordId)
                            .to(TrainingRecord::TrainingRecords, TrainingRecord::TrainingRecordID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(ExerciseRecord::ExerciseTemplateId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_exercise_record_exercise_template")
                            .from(ExerciseRecord::ExerciseRecords, ExerciseRecord::ExerciseTemplateId)
                            .to(ExerciseTemplate::ExerciseTemplates, ExerciseTemplate::ExerciseTemplateID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(boolean(ExerciseRecord::WasSuccessful))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Record::Records)
                    .if_not_exists()
                    .col(pk_auto(Record::RecordID))
                    .col(integer(Record::ExerciseRecordId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_record_exercise_record")
                            .from(Record::Records, Record::ExerciseRecordId)
                            .to(ExerciseRecord::ExerciseRecords, ExerciseRecord::ExerciseRecordID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(Record::UsedGripTypeId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_record_grip_type")
                            .from(Record::Records, Record::UsedGripTypeId)
                            .to(GripType::GripTypes, GripType::GripTypeID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(time(Record::Duration))
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(MeasurementPoint::MeasurementPoints)
                    .if_not_exists()
                    .col(timestamp(MeasurementPoint::MeasurementPointTimestamp).primary_key())
                    .col(integer(MeasurementPoint::RecordId).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_measurement_point_record")
                            .from(MeasurementPoint::MeasurementPoints, MeasurementPoint::RecordId)
                            .to(Record::Records, Record::RecordID)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(integer(MeasurementPoint::Value))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MeasurementPoint::MeasurementPoints).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Record::Records).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ExerciseRecord::ExerciseRecords).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TrainingRecord::TrainingRecords).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TrainingExercise::TrainingExercises).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ExerciseTemplate::ExerciseTemplates).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(FavouriteTraining::FavouriteTrainings).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(TrainingTemplate::TrainingTemplates).to_owned())
            .await?;
        
        manager
            .drop_table(Table::drop().table(HangboardGripType::HangboardGripTypes).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(GripType::GripTypes).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Hangboard::Hangboards).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(FavouriteGym::FavouriteGyms).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Gym::Gyms).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Climber::Climbers).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ProfilePic::ProfilePics).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(ClimbingGrade::ClimbingGrades).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Climber {
    Climbers,
    ClimberID,
    Email,
    PasswordHash,
    Username,
    Firstname,
    Lastname,
    ProfilePicId,
    Weight,
    BirthDate,
    LeadRpIndoor,
    LeadRpOutdoor,
    BoulderRpIndoor,
    BoulderRpOutdoor,
    MaxForceRight,
    MaxForceLeft,
}

#[derive(DeriveIden)]
enum ProfilePic {
    ProfilePics,
    ProfilePicID,
    Url,
}

#[derive(DeriveIden)]
enum ClimbingGrade {
    ClimbingGrades,
    ClimbingGradeID,
    Name,
    GradeContext,
    NumvericalValue,
}

#[derive(DeriveIden)]
enum Gym {
    Gyms,
    GymID,
    Name,
    LocationX,
    LocationY,
    AdminId,
}

#[derive(DeriveIden)]
enum FavouriteGym {
    FavouriteGyms,
    ClimberID,
    GymID,
}

#[derive(DeriveIden)]
enum Hangboard {
    Hangboards,
    HangboardID,
    OwningGymId,
    Nickname,
    LastTimeOnline,
    LastTimeCalibrated,
}

#[derive(DeriveIden)]
enum GripType {
    GripTypes,
    GripTypeID,
    Name,
}

#[derive(DeriveIden)]
enum HangboardGripType {
    HangboardGripTypes,
    HangboardID,
    GripTypeID,
}

#[derive(DeriveIden)]
enum MeasurementPoint{
    MeasurementPoints,
    MeasurementPointTimestamp,
    RecordId,
    Value,
}

#[derive(DeriveIden)]
enum Record{
    Records,
    RecordID,
    ExerciseRecordId,
    UsedGripTypeId,
    Duration,
}

#[derive(DeriveIden)]
enum ExerciseRecord {
    ExerciseRecords,
    ExerciseRecordID,
    TrainingRecordId,
    ExerciseTemplateId,
    WasSuccessful,
}

#[derive(DeriveIden)]
enum TrainingRecord {
    TrainingRecords,
    TrainingRecordID,
    ClimberId,
    TrainingTemplateId,
    TotalDuration,
}

#[derive(DeriveIden)]
enum ExerciseTemplate{
    ExerciseTemplates,
    ExerciseTemplateID,
    ActiveDuration,
    ActiveForce,
    GripTypeId,
    RestDuration,
    RestForce,
}

#[derive(DeriveIden)]
enum TrainingTemplate{
    TrainingTemplates,
    TrainingTemplateID,
    Name,
    CreatorClimberId,
    ClimbingGradeId,
    PredictedDuration,
}

#[derive(DeriveIden)]
enum TrainingExercise {
    TrainingExercises,
    TrainingTemplateID,
    ExerciseTemplateID,
    Repetitions,
    Position,
}

#[derive(DeriveIden)]
enum FavouriteTraining {
    FavouriteTrainings,
    ClimberID,
    TrainingTemplateID,
}