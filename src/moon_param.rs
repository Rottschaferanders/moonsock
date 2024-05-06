use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MoonParam {
    None,
    ServerConnectionIdentifyParams {
        client_name: String,
        version: String,
        #[serde(rename = "type")]
        c_type: String,
        url: String,
    },
    PrinterObjectsQuery {
        objects: PrinterObject,
    },
    PrinterObjectsSubscribe {
        objects: PrinterObject,
    },
    NotifyProcStatUpdate {
        moonraker_stats: MoonrakerStats,
        cpu_temp: f64,
        network: Network,
        system_cpu_usage: SystemCpuUsage,
        system_memory: SystemMemory,
        websocket_connections: u64,
    },
    NotifyCpuThrottled {
        bits: u64,
        flags: Vec<String>,
    },
    NotifyGcodeResponse( String ),
    GcodeScript {
        script: String,
    },
    Count(u64),
    Filename(String),
    Service(SystemdSevice),
    Password(String),
    Root(String),
    Name(String),
    Refresh(bool),
    Device(String),
    Uuid(u64),
    ButtonEvent {
        name: String,
        typee: String,
        event: Event,
        aux: String,
    },
}

impl MoonParam {
    pub fn from_json(json: &str) -> Result<MoonParam, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrinterObject {
    #[serde(rename = "gcode_move")]
    GcodeMove,
    #[serde(rename = "toolhead")]
    Toolhead(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoonrakerStats {
    pub time: f64,
    pub cpu_usage: f64,
    pub memory: u64,
    pub mem_units: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Network {
    pub lo: NetworkData,
    pub eth0: NetworkData,
    pub wlan0: NetworkData,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkData {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub rx_packets: u64,
    pub tx_packets: u64,
    pub rx_errs: u64,
    pub tx_errs: u64,
    pub rx_drop: u64,
    pub tx_drop: u64,
    pub bandwidth: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemCpuUsage {
    pub cpu: f64,
    pub cpu0: f64,
    pub cpu1: f64,
    pub cpu2: f64,
    pub cpu3: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SystemMemory {
    pub total: u64,
    pub available: u64,
    pub used: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SystemdSevice {
    #[serde(rename = "{klipper}")]
    Klipper,
    #[serde(rename = "{moonraker}")]
    Moonraker,
    #[serde(rename = "{nginx}")]
    Nginx,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Event {
    pub elapsed_time: f32,
    pub received_time: f32,
    pub render_time: f32,
    pub pressed: bool,
}
