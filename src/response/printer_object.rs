use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterObjectsListResponse {
    pub objects: Vec<String>, // Array of printer object names
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum PrinterObject {
//     #[serde(rename = "gcode_move")]
//     GcodeMove(Option<Vec<String>>),
//     #[serde(rename = "toolhead")]
//     Toolhead(Option<Vec<String>>),
//     #[serde(rename = "z_tilt")]
//     ZTilt(Option<Vec<String>>),
// }
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterObjectsQueryResponse {
    pub eventtime: f64,
    pub status: PrinterObjectStatus,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterObjectStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcode_move: Option<GcodeMove>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub toolhead: Option<Toolhead>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configfile: Option<Configfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extruder: Option<Extruder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heater_bed: Option<HeaterBed>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan: Option<Fan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idle_timeout: Option<IdleTimeout>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_sdcard: Option<VirtualSdcard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_stats: Option<PrintStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_status: Option<DisplayStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_tilt: Option<ZTilt>,
}

impl Default for PrinterObjectStatus {
    fn default() -> Self {
        Self {
            gcode_move: None,
            toolhead: None,
            configfile: None,
            extruder: None,
            heater_bed: None,
            fan: None,
            idle_timeout: None,
            virtual_sdcard: None,
            print_stats: None,
            display_status: None,
            z_tilt: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcodeMove {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_coordinates: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absolute_extrude: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extrude_factor: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcode_position: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homing_origin: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed_factor: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Toolhead {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub homed_axes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_print_time: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extruder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Vec<f64>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_velocity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_accel: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_accel_to_decel: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub square_corner_velocity: Option<f64>,
}

impl Default for Toolhead {
    fn default() -> Self {
        Self {
            homed_axes: None,
            print_time: None,
            estimated_print_time: None,
            extruder: None,
            position: None,
            max_velocity: None,
            max_accel: None,
            max_accel_to_decel: None,
            square_corner_velocity: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Configfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<Value>, // Can be more specific if the structure is known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<Value>, // Can be more specific if the structure is known
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_config_pending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Extruder {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pressure_advance: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smooth_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HeaterBed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Fan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub speed: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rpm: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct IdleTimeout {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub printing_time: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VirtualSdcard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_position: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintStats {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub print_duration: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filament_used: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<PrintStatsInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintStatsInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_layer: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_layer: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisplayStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ZTilt {
    pub applied: bool,
}

