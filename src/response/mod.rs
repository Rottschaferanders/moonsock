mod temperature;
pub use temperature::*;

mod moon_response;
pub use moon_response::*;

mod notify;
pub use notify::*;

mod printer;
pub use printer::*;

mod server;
pub use server::*;

mod result_data;
pub use result_data::*;

mod machine;
pub use machine::*;

// pub mod responses {
//     // pub use super::moon_response::*;
//     pub use super::temperature::*;
//     pub use super::machine::*;
//     pub use super::notify::*;
//     pub use super::printer::*;
//     pub use super::server::*;
//     pub use super::result_data::*;
// }


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum MoonOk {
//     #[serde(rename = "ok")]
//     Ok,
// }


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum MoonResultData {
//     #[serde(alias = "ok")]
//     Ok(MoonOk),
//     TemperatureStore(TemperatureStore),
//     PrinterInfoResponse(PrinterInfoResponse),
//     PrinterObjectsListResponse(PrinterObjectsListResponse),
//     PrinterObjectsQueryResponse(PrinterObjectsQueryResponse),
//     ServerInfo(ServerInfo),
//     ServerConfig(ServerConfig),
//     GcodeStore(GcodeStore),
//     RollOverResponse(RollOverResponse),
//     None
// }

// impl Default for MoonResultData {
//     fn default() -> Self {
//         MoonResultData::None
//     }
// }

// {
//     "jsonrpc": "2.0",
//     "result": {
//         "gcode_store": [
//             {
//                 "message": "FIRMWARE_RESTART",
//                 "time": 1615832299.1167388,
//                 "type": "command"
//             },
//             {
//                 "message": "// Klipper state: Ready",
//                 "time": 1615832309.9977088,
//                 "type": "response"
//             },
//             {
//                 "message": "M117 This is a test",
//                 "time": 1615834094.8662775,
//                 "type": "command"
//             },
//             {
//                 "message": "G4 P1000",
//                 "time": 1615834098.761729,
//                 "type": "command"
//             },
//             {
//                 "message": "STATUS",
//                 "time": 1615834104.2860553,
//                 "type": "command"
//             },
//             {
//                 "message": "// Klipper state: Ready",
//                 "time": 1615834104.3299904,
//                 "type": "response"
//             }
//         ]
//     },
//     "id": 345
// }