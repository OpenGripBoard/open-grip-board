use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Deserialize, JsonSchema)]
pub enum PatchOperation {
    Add,
    Remove,
}
