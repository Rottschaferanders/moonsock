use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcodeStore {
    message: String,
    time: f32,
    #[serde(rename = "type")]
    typee: GcodeType,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GcodeType {
    Command,
    Response,
}