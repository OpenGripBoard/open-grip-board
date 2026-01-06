use rocket::time::Date;

use crate::structs::grip_type::GripType;

pub struct Hangboard{
    id: i32,
    nickname: String,
    last_time_online: Date,
    last_time_calibrated: Date,
    available_grip_types: Option<Vec<GripType>>
}