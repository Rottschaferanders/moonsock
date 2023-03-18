// use serde::de::DeserializeOwned;
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

fn asf() {
    let message = r##"{
    "jsonrpc": "2.0", 
    "method": "notify_proc_stat_update", 
    "params": 
        [
            {
                "moonraker_stats": {
                    "time": 1679112330.7287357, 
                    "cpu_usage": 2.37, 
                    "memory": 18156, 
                    "mem_units": "kB"
                }, 
                "cpu_temp": 54.53, 
                "network": {
                    "lo": {
                        "rx_bytes": 6394808114, 
                        "tx_bytes": 6394808114, 
                        "rx_packets": 21933659, 
                        "tx_packets": 21933659, 
                        "rx_errs": 0, 
                        "tx_errs": 0, 
                        "rx_drop": 0, 
                        "tx_drop": 0, 
                        "bandwidth": 8630.21
                    }, 
                    "eth0": {
                        "rx_bytes": 0, 
                        "tx_bytes": 0, 
                        "rx_packets": 0, 
                        "tx_packets": 0, 
                        "rx_errs": 0,
                        "tx_errs": 0, 
                        "rx_drop": 0, 
                        "tx_drop": 0, 
                        "bandwidth": 0.0
                    }, 
                    "wlan0": {
                        "rx_bytes": 2637034553, 
                        "tx_bytes": 3243733993, 
                        "rx_packets": 14919504, 
                        "tx_packets": 13199378, 
                        "rx_errs": 0, 
                        "tx_errs": 0, 
                        "rx_drop": 0, 
                        "tx_drop": 0, 
                        "bandwidth": 4659.79
                    }
                }, 
                "system_cpu_usage": {
                    "cpu": 8.79, 
                    "cpu0": 2.02, 
                    "cpu1": 1.03, 
                    "cpu2": 2.97, 
                    "cpu3": 29.0
                }, 
                "system_memory": {
                    "total": 3748168, 
                    "available": 901140, 
                    "used": 2847028
                }, 
                "websocket_connections": 2
            }
        ]
    }"##;
}

impl MoonParam {
    pub fn from_json(json: &str) -> Result<MoonParam, serde_json::Error> {
        serde_json::from_str(json)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrinterObject {
    GcodeMove {
        absolute_coordinates: bool,
        absolute_extrude: bool,
        extrude_factor: f32,
        gcode_position: [f32; 4],
        homing_origin: [f32; 4],
        position: [f32; 4],
        speed: u64,
        speed_factor: f32,
    },
    Toolhead {
        toolhead: Vec<String>,
    },
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

// {"rx_bytes": , "tx_bytes": , "rx_packets": , "tx_packets": , "rx_errs": 0, "tx_errs": 0, "rx_drop": 0, "tx_drop": 0, "bandwidth": 8630.21}
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
