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

// impl Deserialize for PrinterState {
//     fn deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         let state_str = deserializer.deserialize_str()?;
//         match state_str {
//             "ready" => Ok(PrinterState::Ready),
//             "paused" => Ok(PrinterState::Paused),
//             "standby" => Ok(PrinterState::Standby),
//             "printing" => Ok(PrinterState::Printing),
//             "complete" => Ok(PrinterState::Complete),
//             "cancelled" => Ok(PrinterState::Cancelled),
//             "error" => Ok(PrinterState::Error),
//             _ => Err(serde::de::Error::custom("Invalid printer state")),
//         }
//     }
// }