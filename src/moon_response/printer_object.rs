use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcodeMove {
    pub speed_factor: Option<f64>,
    pub speed: Option<f64>,
    pub extrude_factor: Option<f64>,
    pub absolute_coordinates: Option<bool>,
    pub absolute_extrude: Option<bool>,
    pub homing_origin: Option<Vec<f64>>,
    pub position: Option<Vec<f64>>,
    pub gcode_position: Option<Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Toolhead {
    pub homed_axes: Option<String>,
    pub print_time: Option<f64>,
    pub estimated_print_time: Option<f64>,
    pub extruder: Option<String>,
    pub position: Option<Vec<f64>>,
    pub max_velocity: Option<f64>,
    pub max_accel: Option<f64>,
    pub max_accel_to_decel: Option<f64>,
    pub square_corner_velocity: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Configfile {
    pub config: Option<Value>, // Can be more specific if the structure is known
    pub settings: Option<Value>, // Can be more specific if the structure is known
    pub save_config_pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Extruder {
    pub temperature: Option<f64>,
    pub target: Option<f64>,
    pub power: Option<f64>,
    pub pressure_advance: Option<f64>,
    pub smooth_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeaterBed {
    pub temperature: Option<f64>,
    pub target: Option<f64>,
    pub power: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fan {
    pub speed: Option<f64>,
    pub rpm: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdleTimeout {
    pub state: Option<String>,
    pub printing_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VirtualSdcard {
    pub progress: Option<f64>,
    pub is_active: Option<bool>,
    pub file_position: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintStats {
    pub filename: Option<String>,
    pub total_duration: Option<f64>,
    pub print_duration: Option<f64>,
    pub filament_used: Option<f64>,
    pub state: Option<String>,
    pub message: Option<String>,
    pub info: Option<PrintStatsInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintStatsInfo {
    pub total_layer: Option<u32>,
    pub current_layer: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisplayStatus {
    pub message: Option<String>,
    pub progress: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterObjectStatus {
    pub gcode_move: Option<GcodeMove>,
    pub toolhead: Option<Toolhead>,
    pub configfile: Option<Configfile>,
    pub extruder: Option<Extruder>,
    pub heater_bed: Option<HeaterBed>,
    pub fan: Option<Fan>,
    pub idle_timeout: Option<IdleTimeout>,
    pub virtual_sdcard: Option<VirtualSdcard>,
    pub print_stats: Option<PrintStats>,
    pub display_status: Option<DisplayStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryPrinterObjectsResponse {
    pub eventtime: f64,
    pub status: PrinterObjectStatus,
}
