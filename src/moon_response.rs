use serde::{Serialize, Deserialize};
use serde_json::Value;

use crate::{TemperatureStore, PrinterInfoResponse};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MoonOk {
    #[serde(rename = "ok")]
    Ok,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MoonResultData {
    #[serde(alias = "ok")]
    Ok(MoonOk),
    TemperatureStore(TemperatureStore),
    PrinterInfoResponse(PrinterInfoResponse),
    QueryPrinterObjectsResponse(QueryPrinterObjectsResponse)
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct QueryPrinterObjectsResponse {
//     eventtime: f64,
//     status: Vec<Status>,
// }
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct QueryPrinterObjectsResponse {
//     eventtime: f64,
//     status: Status,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Status {
//     gcode_move: Option<GcodeMove>,
//     toolhead: Option<Toolhead>,
//     configfile: Option<Configfile>,
//     // toolhead: Toolhead,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Toolhead {
//     homed_axes: String,
// }
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Toolhead {
//     homed_axes: Option<String>,
//     print_time: Option<f64>,
//     estimated_print_time: Option<f64>,
//     extruder: Option<String>,
//     position: Option<[f64; 4]>,
//     max_velocity: Option<f64>,
//     max_accel: Option<f64>,
//     max_accel_to_decel: Option<f64>,
//     square_corner_velocity: Option<f64>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct GcodeMove {
//     speed_factor: Option<f64>,
//     speed: Option<f64>,
//     extrude_factor: Option<f64>,
//     absolute_coordinates: Option<bool>,
//     absolute_extrude: Option<bool>,
//     homing_origin: Option<Vec<f64>>,
//     position: Option<Vec<f64>>,
//     gcode_position: Option<Vec<f64>>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Configfile {
//     config: Option<Value>, // or specific type if you have a known structure
//     settings: Option<Value>, // or specific type if you have a known structure
//     save_config_pending: Option<bool>,
// }


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcodeMove {
    speed_factor: Option<f64>,
    speed: Option<f64>,
    extrude_factor: Option<f64>,
    absolute_coordinates: Option<bool>,
    absolute_extrude: Option<bool>,
    homing_origin: Option<Vec<f64>>,
    position: Option<Vec<f64>>,
    gcode_position: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Toolhead {
    homed_axes: Option<String>,
    print_time: Option<f64>,
    estimated_print_time: Option<f64>,
    extruder: Option<String>,
    position: Option<Vec<f64>>,
    max_velocity: Option<f64>,
    max_accel: Option<f64>,
    max_accel_to_decel: Option<f64>,
    square_corner_velocity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Configfile {
    config: Option<Value>, // Can be more specific if the structure is known
    settings: Option<Value>, // Can be more specific if the structure is known
    save_config_pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Extruder {
    temperature: Option<f64>,
    target: Option<f64>,
    power: Option<f64>,
    pressure_advance: Option<f64>,
    smooth_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeaterBed {
    temperature: Option<f64>,
    target: Option<f64>,
    power: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fan {
    speed: Option<f64>,
    rpm: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdleTimeout {
    state: Option<String>,
    printing_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VirtualSdcard {
    progress: Option<f64>,
    is_active: Option<bool>,
    file_position: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintStats {
    filename: Option<String>,
    total_duration: Option<f64>,
    print_duration: Option<f64>,
    filament_used: Option<f64>,
    state: Option<String>,
    message: Option<String>,
    info: Option<PrintStatsInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintStatsInfo {
    total_layer: Option<u32>,
    current_layer: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisplayStatus {
    message: Option<String>,
    progress: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Status {
    gcode_move: Option<GcodeMove>,
    toolhead: Option<Toolhead>,
    configfile: Option<Configfile>,
    extruder: Option<Extruder>,
    heater_bed: Option<HeaterBed>,
    fan: Option<Fan>,
    idle_timeout: Option<IdleTimeout>,
    virtual_sdcard: Option<VirtualSdcard>,
    print_stats: Option<PrintStats>,
    display_status: Option<DisplayStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryPrinterObjectsResponse {
    eventtime: f64,
    status: Status,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// enum PrinterObjectFromResponse {
//     #[serde(rename = "toolhead")]
//     // Toolhead(Vec<ToolheadValue>),
    // Toolhead {
    //     homed_axes: Option<String>,
    //     print_time: Option<f32>,
    //     estimated_print_time: Option<f32>,
    //     extruder: Option<String>,
    //     position: Option<[f32; 4]>,
    //     max_velocity: Option<f32>,
    //     max_accel: Option<f32>,
    //     max_accel_to_decel: Option<f32>,
    //     square_corner_velocity: Option<f32>,
    // },
//     #[serde(rename = "gcode_move")]
//     GcodeMove {
//         speed: Option<u64>,
//         speed_factor: Option<f32>,
//         extrude_factor: Option<f32>,
//         absolute_coordinates: Option<bool>,
//         absolute_extrude: Option<bool>,
//         homing_origin: Option<[f32; 4]>,
//         position: Option<[f32; 4]>,
//         gcode_position: Option<[f32; 4]>,
//     },
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum ToolheadValue {
//     #[serde(rename = "homed_axes")]
//     HomedAxes(String),
//     PrintTime(f32),
//     // "print_time": 0.0,
//     EstimatedPrintTime(f32),
//     // "estimated_print_time": 0.0,
//     Extruder(String),
//     // "extruder": "extruder",
//     Position([f32; 4]),
//     // "position": [0.0, 0.0, 0.0, 0.0],
//     MaxVelocity(f32),
//     // "max_velocity": 500.0,
//     MaxAccel(f32),
//     // "max_accel": 3000.0,
//     MaxAccelToDecel(f32),
//     // "max_accel_to_decel": 1500.0,
//     SquareCornerVelocity(f32),
//     // "square_corner_velocity": 5.0
// }


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// // #[serde(tag = "type")] // here "type" needs to match the JSON discriminant if provided, or you can remove it if the JSON doesn't provide it
// #[serde(untagged)]
// pub enum Status {
//     #[serde(rename = "gcode_move")]
//     GcodeMove(GcodeMove),

//     #[serde(rename = "toolhead")]
//     Toolhead(Toolhead),

//     #[serde(rename = "configfile")]
//     Configfile(Configfile),

//     #[serde(rename = "extruder")]
//     Extruder(Extruder),

//     #[serde(rename = "heater_bed")]
//     HeaterBed(HeaterBed),

//     #[serde(rename = "fan")]
//     Fan(Fan),

//     #[serde(rename = "idle_timeout")]
//     IdleTimeout(IdleTimeout),

//     #[serde(rename = "virtual_sdcard")]
//     VirtualSdcard(VirtualSdcard),

//     #[serde(rename = "print_stats")]
//     PrintStats(PrintStats),

//     #[serde(rename = "display_status")]
//     DisplayStatus(DisplayStatus),
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct GcodeMove {
//     speed_factor: f64,
//     speed: f64,
//     extrude_factor: f64,
//     absolute_coordinates: bool,
//     absolute_extrude: bool,
//     homing_origin: [f64; 4],
//     position: [f64; 4],
//     gcode_position: [f64; 4],
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Toolhead {
//     homed_axes: Option<String>,
//     print_time: Option<f64>,
//     estimated_print_time: Option<f64>,
//     extruder: Option<String>,
//     position: Option<[f64; 4]>,
//     max_velocity: Option<f64>,
//     max_accel: Option<f64>,
//     max_accel_to_decel: Option<f64>,
//     square_corner_velocity: Option<f64>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Configfile {
//     config: Value,
//     settings: Value,
//     save_config_pending: bool,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Extruder {
//     temperature: f64,
//     target: f64,
//     power: f64,
//     pressure_advance: f64,
//     smooth_time: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct HeaterBed {
//     temperature: f64,
//     target: f64,
//     power: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Fan {
//     speed: f64,
//     rpm: u32,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct IdleTimeout {
//     state: String,
//     printing_time: f64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct VirtualSdcard {
//     progress: f64,
//     is_active: bool,
//     file_position: u64,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct PrintStats {
//     filename: String,
//     total_duration: f64,
//     print_duration: f64,
//     filament_used: f64,
//     state: String,
//     message: String,
//     info: PrintStatsInfo,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct PrintStatsInfo {
//     total_layer: Option<u32>,
//     current_layer: Option<u32>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct DisplayStatus {
//     message: String,
//     progress: f64,
// }