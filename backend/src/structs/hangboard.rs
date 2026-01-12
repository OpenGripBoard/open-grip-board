use entity::hangboards;
use sea_orm::prelude::DateTime;

use crate::structs::grip_type::GripType;

pub struct Hangboard {
    id: i32,
    nickname: String,
    last_time_online: DateTime,
    last_time_calibrated: DateTime,
    available_grip_types: Option<Vec<GripType>>,
}

impl From<hangboards::Model> for Hangboard {
    fn from(hangboard: hangboards::Model) -> Self {
        Hangboard {
            id: hangboard.hangboard_id,
            nickname: hangboard.nickname,
            last_time_online: hangboard.last_time_online,
            last_time_calibrated: hangboard.last_time_calibrated,
            available_grip_types: None,
        }
    }
}
