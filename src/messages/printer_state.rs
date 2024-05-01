use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrinterState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "standby")]
    Standby,
    #[serde(rename = "printing")]
    Printing,
    #[serde(rename = "complete")]
    Complete,
    #[serde(rename = "cancelled")]
    Cancelled,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterInfoResponse {
    // pub state: String,
    pub state: PrinterState,
    pub state_message: String,
    pub hostname: String,
    pub klipper_path: String,
    pub python_path: String,
    pub process_id: i32,
    pub user_id: i32,
    pub group_id: i32,
    pub log_file: String,
    pub config_file: String,
    pub software_version: String,
    pub cpu_info: String,
}
