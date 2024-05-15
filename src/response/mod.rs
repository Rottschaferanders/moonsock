// use serde::{Serialize, Deserialize};

// mod printer_info;
// pub use printer_info::*;

mod temperature;
pub use temperature::*;

// mod gcode;
// pub use gcode::*;

// mod printer_object;
// pub use printer_object::*;

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