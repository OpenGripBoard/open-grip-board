use entity::grip_types;

pub struct GripType {
    pub id: i32,
    #[allow(dead_code)]
    name: String,
}

impl From<grip_types::Model> for GripType {
    fn from(grip_type: grip_types::Model) -> Self {
        GripType {
            id: grip_type.grip_type_id,
            name: grip_type.name,
        }
    }
}
