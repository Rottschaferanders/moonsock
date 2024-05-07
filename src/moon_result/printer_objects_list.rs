use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterObjectsListResponse {
    pub objects: Vec<String>, // Array of printer object names
}
