use serde::de::DeserializeOwned;
use serde::{Serialize, Deserialize};

pub mod connection;
pub mod moon_method;
pub mod moon_param;

// Exports:
pub use connection::MoonConnection;
pub use moon_method::MoonMethod;
pub use moon_param::MoonParam;

// From this very helpful article: https://blog.dzejkop.space/serde-by-example-1/

/// ---------------------- Response Deserializing ------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct Response<T> {
    pub jsonrpc: JsonRpcVersion,
    #[serde(flatten)]
    #[serde(with = "ResultDef")]
    pub result: Result<T, ResponseError>,
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2_0
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(remote = "Result")]
pub enum ResultDef<T, E> {
    #[serde(rename = "result")]
    Ok(T),
    #[serde(rename = "error")]
    Err(E),
}

/// ---------------------- Request Serializing ------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MoonMSG {
    MoonError {
        jsonrpc: JsonRpcVersion,
        error: MoonError,
        id: u32,
    },
    MethodParamID {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
        params: MoonParam,
        id: u32,
    },
    MethodParamVecID {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
        params: Vec<MoonParam>,
        id: u32,
    },
    MethodParamVec {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
        params: Vec<MoonParam>,
    },
    MethodParam {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
        params: MoonParam,
    },
    MethodID {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
        id: u32,
    },
    Method {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
    },
    CouldNotParseParams {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
        params: serde_json::Value,
    },
    CouldNotParseMethod {
        jsonrpc: JsonRpcVersion,
        method: String,
        params: MoonParam,
    },
    CouldNotParseMethodID {
        jsonrpc: JsonRpcVersion,
        method: String,
        params: MoonParam,
        id: u32,
    },
    CouldNotParseMethodParams {
        jsonrpc: JsonRpcVersion,
        method: String,
        params: serde_json::Value,
    },
    CouldNotParseMethodParamsID {
        jsonrpc: JsonRpcVersion,
        method: String,
        params: serde_json::Value,
        id: u32,
    },

    // I think this is useless, but I don't want to scan through the moonraker API docs again.
    // NotRecognized { value: serde_json::Value },
    // ConnectionID { connection_id: u32 },
    // KlippyHostInfo {
    //     state: PrinterState,
    //     state_message: String,
    //     hostname: String,
    //     software_version: String,
    //     cpu_info: String,
    //     klipper_path: String,
    //     python_path: String,
    //     log_file: String,
    //     config_file: String,
    // },
    // #[serde(rename = "ok")]
    // Ok,
    // AvailablePrinterObjects { objects: Vec<String> },
    // PrinterObjectStatus { eventtime: f32, status: Vec<PrinterObject>},
    // EndstopStatuses { x: String, y: String, z: String },
    // ServerInfo {
    //     klippy_connected: bool,
    //     klippy_state: PrinterState,
    //     components: Vec<String>,
    //     failed_components: Vec<String>,
    //     registered_directories: Vec<String>,
    //     warnings: Vec<String>,
    //     websocket_count: u32,
    //     moonraker_version: String,
    //     api_version: [u32; 3],
    //     api_version_string: String,
    // },
    // ServerInfoWithPlugins {
    //     klippy_connected: bool,
    //     klippy_state: PrinterState,
    //     components: Vec<String>,
    //     failed_components: Vec<String>,
    //     plugins: Vec<String>,
    //     failed_plugins: Vec<String>,
    //     registered_directories: Vec<String>,
    //     warnings: Vec<String>,
    //     websocket_count: u32,
    //     moonraker_version: String,
    //     api_version: [u32; 3],
    //     api_version_string: String,
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoonError {
    pub code: u32,
    pub message: String,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GcodeStore {
    message: String,
    time: f32,
    #[serde(rename = "type")]
    typee: GcodeType,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GcodeType {
    Command,
    Response,
}

impl MoonMSG {
    pub fn new(method: MoonMethod, params: MoonParam, id: u32) -> MoonMSG {
        match params {
            MoonParam::None => MoonMSG::MethodID { jsonrpc: JsonRpcVersion::V2_0, method: method, id: id },
            _ => MoonMSG::MethodParamID { jsonrpc: JsonRpcVersion::V2_0, method: method, params: params, id: id },
        }
    }
    #[allow(dead_code)]
    pub fn new_method(method: MoonMethod, id: u32) -> MoonMSG {
        MoonMSG::MethodID {
            jsonrpc: JsonRpcVersion::V2_0,
            method,
            id,
        }
    }
    pub fn gcode(gcode: String, id: u32) -> MoonMSG {
        MoonMSG::new(MoonMethod::GcodeScript, MoonParam::GcodeScript { script: gcode.to_string() }, id)
    }
    pub fn method(&self) ->  &MoonMethod {
        match self {
            MoonMSG::MoonError { error, .. } => {
                panic!("Error: {:?}", error);
            },
            MoonMSG::MethodParamID { method, .. } => method,
            MoonMSG::MethodParamVecID { method, .. } => method,
            MoonMSG::MethodParam { method, .. } => method,
            MoonMSG::MethodParamVec { method, .. } => method,
            MoonMSG::MethodID { method, .. } => method,
            MoonMSG::Method { method, .. } => method,
            MoonMSG::CouldNotParseParams { method, .. } => {
                panic!("CouldNotParseParams: {:?}", method);
            },
            MoonMSG::CouldNotParseMethod { method, .. } => {
                panic!("CouldNotParseMethod: {:?}", method);
            },
            MoonMSG::CouldNotParseMethodID { method, .. } => {
                panic!("CouldNotParseMethodID: {:?}", method);
            },
            MoonMSG::CouldNotParseMethodParams { method, .. } => {
                panic!("CouldNotParseMethodParams: {:?}", method);
            },
            MoonMSG::CouldNotParseMethodParamsID { method, .. } => {
                panic!("CouldNotParseMethodParamsID: {:?}", method);
            },
        }
    }
    pub fn params(&self) -> Vec<MoonParam> {
        match self {
            MoonMSG::MoonError {..} => {
                panic!("MoonError has no params");
            },
            MoonMSG::MethodParamID { params, .. } => vec! {params.clone()},
            MoonMSG::MethodParamVecID { params, .. } => params.clone(),
            MoonMSG::MethodParam { params, .. } => vec! { params.clone() },
            MoonMSG::MethodParamVec { params, .. } => params.clone(),
            MoonMSG::MethodID {..} => {
                panic!("MethodID has no params");
            },
            MoonMSG::Method {..} => {
                panic!("Method has no params");
            },
            MoonMSG::CouldNotParseParams { params, .. } => {
                panic!("CouldNotParseParams: {:?}", params);
            },
            MoonMSG::CouldNotParseMethod { params, .. } => vec! {params.clone()},
            MoonMSG::CouldNotParseMethodID { params, .. } => vec! {params.clone()},
            MoonMSG::CouldNotParseMethodParams { params, .. } => {
                panic!("CouldNotParseMethodParams: {:?}", params);
            },
            MoonMSG::CouldNotParseMethodParamsID { params, .. } => {
                panic!("CouldNotParseMethodParamsID: {:?}", params);
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrinterState {
    Ready,
    Paused,
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum MoonMethod {
//     #[serde(rename="server.connection.identify")]
//     ServerConnectionIdentify,
//     #[serde(rename="server.info")]
//     ServerInfo,
//     #[serde(rename="server.config")]
//     ServerConfig,
//     #[serde(rename = "server.temperature_store")]
//     TemperatureStore,
//     #[serde(rename = "server.gcode_store")]
//     GcodeStore,
//     #[serde(rename = "server.restart")]
//     ServerRestart,
//     #[serde(rename="printer.query_endstops.status")]
//     PrinterQueryEndstopsStatus,
//     // #[serde(rename="server.websocket.id")]
//     // WebsocketID,
//     #[serde(rename="printer.info")]
//     PrinterInfo,
//     #[serde(rename="printer.emergency_stop")]
//     EmergencyStop,
//     #[serde(rename="printer.restart")]
//     Restart,
//     #[serde(rename="printer.firmware_restart")]
//     FirmwareRestart,
//     #[serde(rename="printer.printer.objects.list")]
//     PrinterObjectsList, // Lists available printer objects
//     #[serde(rename="printer.objects.query")]
//     PrinterObjectsQuery,
//     #[serde(rename="printer.objects.subscribe")]
//     PrinterObjectsSubscribe,
//     #[serde(rename="printer.gcode.script")]
//     GcodeScript,
//     #[serde(rename="printer.gcode.help")]
//     PrinterGcodeHelp,
//     #[serde(rename="printer.print.start")]
//     PrinterPrintStart,
//     #[serde(rename="printer.print.pause")]
//     PrinterPrintPause,
//     #[serde(rename="printer.print.resume")]
//     PrinterPrintResume,
//     #[serde(rename="printer.print.cancel")]
//     PrinterPrintCancel,
//     #[serde(rename="machine.system_info")]
//     MachineSystemInfo,
//     #[serde(rename="machine.shutdown")]
//     MachineShutdown,
//     #[serde(rename="machine.reboot")]
//     MachineReboot,
//     #[serde(rename="machine.services.restart")]
//     // Restarts a system service via sudo systemctl restart {name}. Currently the 
//     // moonraker, klipper, MoonCord, KlipperScreen and webcamd services are supported.
//     MachineServicesRestart, 
//     #[serde(rename="machine.services.stop")]
//     // Stops a system service via sudo systemctl stop <name>. Currently only webcamd 
//     // and klipper are supported.
//     MachineServicesStop,
//     #[serde(rename="machine.services.start")]
//     // Starts a system service via sudo systemctl start <name>. Currently only 
//     // webcamd and klipper are supported.
//     MachineServicesStart,
//     #[serde(rename="machine.proc_stat")]
//     MachineProcStat, // Returns system usage information about the moonraker process.
//     #[serde(rename="machine.sudo")]
//     MachineSudo, // Checks if Moonraker has permission to run commands as root.
//     #[serde(rename="machine.sudo.password")]
//     // Sets/updates the sudo password currently used by Moonraker. When the password 
//     // is set using this endpoint the change is not persistent across restarts.
//     MachineSudoPassword, 
//     #[serde(rename="server.files.list")]
//     ServerFilesList,
//     #[serde(rename="server.files.metadata")]
//     ServerFilesMetadata,
//     #[serde(rename="server.files.get_directory")]
//     ServerFilesGetDirectory,
//     #[serde(rename="server.files.post_directory")]
//     ServerFilesPostDirectory, // Creates a directory at the specified path.
//     #[serde(rename="server.files.delete_directory")]
//     ServerFilesDeleteDirectory,
//     #[serde(rename="server.files.move")]
//     ServerFilesMove,
//     #[serde(rename="server.files.copy")]
//     ServerFilesCopy,
//     #[serde(rename="server.files.delete_file")]
//     ServerFilesDeleteFile,
//     #[serde(rename="server.database.list")]
//     ServerDatabaseList,
//     #[serde(rename="server.database.get_item")]
//     ServerDatabaseGetItem,
//     #[serde(rename="server.database.post_item")]
//     ServerDatabasePostItem,
//     #[serde(rename="server.database.delete_item")]
//     ServerDatabaseDeleteItem,
//     #[serde(rename="server.job_queue.status")]
//     ServerJobQueueStatus,
//     #[serde(rename="server.job_queue.post_job")]
//     ServerJobQueuePostJob,
//     #[serde(rename="server.job_queue.delete_job")]
//     ServerJobQueueDeleteJob,
//     #[serde(rename="server.job_queue.pause")]
//     ServerJobQueuePause,
//     #[serde(rename="server.job_queue.start")]
//     ServerJobQueueStart,
//     #[serde(rename="server.announcements.list")]
//     ServerAnnouncementsList,
//     #[serde(rename="server.announcements.update")]
//     ServerAnnouncementsUpdate,
//     #[serde(rename="server.announcements.dismiss")]
//     ServerAnnouncementsDismiss,
//     #[serde(rename="server.announcements.feeds")]
//     ServerAnnouncementsFeeds,
//     #[serde(rename="server.announcements.post_feed")]
//     ServerAnnouncementsPostFeed,
//     #[serde(rename="server.announcements.delete_feed")]
//     ServerAnnouncements,
//     #[serde(rename="server.webcams.list")]
//     ServerWebcamsList,
//     #[serde(rename="server.webcams.get_item")]
//     ServerWebcamsGetItem,
//     #[serde(rename="server.webcams.delete_item")]
//     ServerWebcamsDeleteItem,
//     #[serde(rename="server.webcams.test")]
//     ServerWebcamsTest,
//     #[serde(rename="machine.update.status")]
//     MachineUpdateStatus,
//     #[serde(rename="machine.update.full")]
//     MachineUpdateFull,
//     #[serde(rename="machine.update.moonraker")]
//     MachineUpdateMoonraker,
//     #[serde(rename="machine.update.klipper")]
//     MachineUpdateKlipper,
//     #[serde(rename="machine.update.client")]
//     MachineUpdateClient,
//     #[serde(rename="machine.update.system")]
//     MachineUpdateSystem,
//     #[serde(rename="machine.update.recover")]
//     MachineUpdateRecover,
//     #[serde(rename="machine.device_power.devices")]
//     MachineDevicePowerDevices,
//     #[serde(rename="machine.device_power.get_device")]
//     MachineDevicePowerGetDevice,
//     #[serde(rename="machine.device_power.post_device")]
//     MachineDevicePowerPostDevice,
//     #[serde(rename="machine.device_power.status")]
//     MachineDevicePowerStatus,
//     #[serde(rename="machine.device_power.on")]
//     MachineDevicePowerOn,
//     #[serde(rename="machine.device_power.off")]
//     MachineDevicePowerOff,
//     #[serde(rename="machine.wled.strips")]
//     MachineWledStrips,
//     #[serde(rename="machine.wled.status")]
//     MachineWledStatus,
//     #[serde(rename="machine.wled.on")]
//     MachineWledOn,
//     #[serde(rename="machine.wled.off")]
//     MachineWledOff,
//     #[serde(rename="machine.wled.toggle")]
//     MachineWledToggle,
//     #[serde(rename="machine.wled.get_strip")]
//     MachineWledGetStrip,
//     #[serde(rename="server.history.list")]
//     ServerHistoryList,
//     #[serde(rename="server.history.totals")]
//     ServerHistoryTotals,
//     #[serde(rename="server.history.reset_totals")]
//     ServerHistoryResetTotals,
//     #[serde(rename="server.history.get_job")]
//     ServerHistoryGetJob,
//     #[serde(rename="server.history.delete_job")]
//     ServerHistoryDeleteJob,
//     #[serde(rename="server.mqtt.publish")]
//     ServerMgttPublish,
//     #[serde(rename="server.mqtt.subscribe")]
//     ServerMqttSubscribe,
//     #[serde(rename="server.extensions.list")]
//     ServerExtensionList,
//     #[serde(rename="server.extensions.request")]
//     ServerExtensionsRequest,
//     #[serde(rename="connection.send_event")]
//     ConnectionSendEvent,
//     #[serde(rename="notify_gcode_response")]
//     NotifyGcodeResponse,
//     #[serde(rename="notify_status_update")]
//     NotifyStatusUpdate,
//     #[serde(rename="notify_klippy_ready")]
//     NotifyKlippyReady,
//     #[serde(rename="notify_klippy_shutdown")]
//     NotifyKlippyShutdown,
//     #[serde(rename="notify_klippy_disconnected")]
//     NotifyKlippyDisconnected,
//     #[serde(rename="notify_filelist_change")]
//     NotifyFilelistChange,
//     #[serde(rename="notify_update_response")]
//     NotifyUpdateResponse,
//     #[serde(rename="notify_update_refreshed")]
//     NotifyUpdateRefreshed,
//     #[serde(rename="notify_cpu_throttled")]
//     NotifyCpuThrottled,
//     #[serde(rename="notify_proc_stat_update")]
//     NotifyProcStatUpdate,
//     #[serde(rename="notify_history_changed")]
//     NotifyHistoryChanged,
//     #[serde(rename="notify_user_created")]
//     NotifyUserCreated,
//     #[serde(rename="notify_user_deleted")]
//     NotifyUserDeleted,
//     #[serde(rename="notify_service_state_changed")]
//     NotifyServiceStateChanged,
//     #[serde(rename="notify_job_queue_changed")]
//     NotifyJobQueueChanged,
//     #[serde(rename="notify_button_event")]
//     NotifyButtonEvent,
//     #[serde(rename="notify_announcement_update")]
//     NotifyAnnouncementUpdate,
//     #[serde(rename="notify_announcement_dismissed")]
//     NotifyAnnouncementDismissed,
//     #[serde(rename="notify_announcement_wake")]
//     NotifyAnnouncementWake,
//     #[serde(rename="notify_agent_event")]
//     NotifyAgentEvent,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum MoonParam {
//     None,
//     ServerConnectionIdentifyParams {
//         client_name: String,
//         version: String,
//         #[serde(rename="type")]
//         c_type: String,
//         url: String,
//     },
//     PrinterObjectsQuery { objects: PrinterObject },
//     PrinterObjectsSubscribe { objects: PrinterObject },
//     NotifyProcStatUpdate {
//         moonraker_stats: ProcStatMoonrakerStats,
//         cpu_temp: f64, 
//         network: ProcStatNetwork, 
//         system_cpu_usage: ProcStatSystemCpuUsage, 
//         system_memory: ProcStatSystemMemory, 
//         websocket_connections: u32,
//     },
//     GcodeScript { script: String },
//     Count(u32),
//     Filename(String),
//     Service(SystemdSevice),
//     Password(String),
//     Root(String),
//     Name(String),
//     Refresh(bool),
//     Device(String),
//     Uuid(u32),
//     ButtonEvent {
//         name: String,
//         typee: String,
//         event: Event,
//         aux: String,
//     }
// }

// impl MoonParam {
//     pub fn from_json(json: &str) -> Result<MoonParam, serde_json::Error> {
//         serde_json::from_str(json)
//     }
    
// }
// impl MoonParam::NotifyProcStatUpdate {
//     pub fn moonraker_stats(&self) -> &ProcStatMoonrakerStats {
//         &self.moonraker_stats
//     }
//     pub fn cpu_temp(&self) -> f64 {
//         self.cpu_temp
//     }
//     pub fn network(&self) -> &ProcStatNetwork {
//         &self.network
//     }
//     pub fn system_cpu_usage(&self) -> &ProcStatSystemCpuUsage {
//         &self.system_cpu_usage
//     }
//     pub fn system_memory(&self) -> &ProcStatSystemMemory {
//         &self.system_memory
//     }
//     pub fn websocket_connections(&self) -> u32 {
//         self.websocket_connections
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct Event {
//     pub elapsed_time: f32,
//     pub received_time: f32,
//     pub render_time: f32,
//     pub pressed: bool,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum SystemdSevice {
//     #[serde(rename = "{klipper}")]
//     Klipper,
//     #[serde(rename = "{moonraker}")]
//     Moonraker,
//     #[serde(rename = "{nginx}")]
//     Nginx
// }


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub enum PrinterObject {
//     GcodeMove { 
//         absolute_coordinates: bool, 
//         absolute_extrude: bool,
//         extrude_factor: f32, 
//         gcode_position: [f32; 4],
//         homing_origin: [f32; 4], 
//         position: [f32; 4],
//         speed: u32, 
//         speed_factor: f32,
//     },
//     Toolhead { toolhead: Vec<String> },
// }


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ProcStatMoonrakerStats {
//     pub time: f64, 
//     pub cpu_usage: f64, 
//     pub memory: u32, 
//     pub mem_units: String
// }
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ProcStatNetwork {
//     lo: ProcStatNetworkData, 
//     eth0: ProcStatNetworkData, 
//     wlan0: ProcStatNetworkData,
//     docker0: ProcStatNetworkData,
// }
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ProcStatNetworkData { rx_bytes: u32, tx_bytes: u32, bandwidth: f64 }
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ProcStatSystemCpuUsage { pub cpu: f64, pub cpu0: f64, pub cpu1: f64, pub cpu2: f64, pub cpu3: f64}
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct ProcStatSystemMemory { pub total: u32, pub available: u32, pub used: u32}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    jsonrpc: JsonRpcVersion,
    method: MoonMethod,
    params: MoonParam,
    id: u32,
}

impl Request {
    #[allow(dead_code)]
    pub fn new(method: MoonMethod, params: MoonParam, id: u32) -> Request {
        // match params {
        //     MoonParam::None => Request { jsonrpc: JsonRpcVersion::V2_0, method: method },
        //     _ => Request::MethodParam { jsonrpc: JsonRpcVersion::V2_0, method: method, params: params },
        // }
        Request { 
            jsonrpc: JsonRpcVersion::V2_0, 
            method: method, 
            params: params,
            id: id,
        }
    }
}
