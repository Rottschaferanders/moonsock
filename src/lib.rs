use serde::{Serialize, Deserialize};

pub mod connection;
pub mod moon_method;
pub mod moon_param;

// Exports:
pub use connection::MoonConnection;
pub use moon_method::MoonMethod;
pub use moon_param::MoonParam;

/// ---------------------- Request Serializing ------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2_0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum MoonMSG {
    MoonResult {
        jsonrpc: JsonRpcVersion,
        result: MoonResultData,
        id: u32,
    },
    MoonError {
        jsonrpc: JsonRpcVersion,
        error: MoonErrorContent,
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
    CouldNotParseParamsID {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
        params: serde_json::Value,
        id: u32,
    },
    CouldNotParseParams {
        jsonrpc: JsonRpcVersion,
        method: MoonMethod,
        params: serde_json::Value,
    },
    CouldNotParseMethodID {
        jsonrpc: JsonRpcVersion,
        method: String,
        params: MoonParam,
        id: u32,
    },
    CouldNotParseMethod {
        jsonrpc: JsonRpcVersion,
        method: String,
        params: MoonParam,
    },
    CouldNotParseMethodParamsID {
        jsonrpc: JsonRpcVersion,
        method: String,
        params: serde_json::Value,
        id: u32,
    },
    CouldNotParseMethodParams {
        jsonrpc: JsonRpcVersion,
        method: String,
        params: serde_json::Value,
    },
    Empty,
}
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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum TempStoreData {
    TempTgtsPowers {
        temperatures: Vec<f32>,
        targets: Vec<f32>,
        powers: Vec<f32>,
    },
    Temp {
        temperatures: Vec<f32>,
    },
}

/// The names of the items in the temperature store
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum HeaterNames {
    #[serde(rename = "heater_bed")]
    HeaterBed,
    #[serde(rename = "extruder")]
    Extruder,
    #[serde(rename = "extruder1")]
    Extruder1,
    #[serde(rename = "extruder2")]
    Extruder2,
    #[serde(rename = "extruder3")]
    Extruder3,
    #[serde(rename = "extruder4")]
    Extruder4,
    #[serde(rename = "extruder5")]
    Extruder5,
    #[serde(rename = "extruder6")]
    Extruder6,
    #[serde(rename = "extruder7")]
    Extruder7,
    #[serde(rename = "extruder8")]
    Extruder8,
    #[serde(rename = "extruder9")]
    Extruder9,
    #[serde(rename = "extruder10")]
    Extruder10,
    #[serde(rename = "temperature_fan")]
    TemperatureFan,
    #[serde(rename = "temperature_sensor")]
    TemperatureSensor,
    NameStr(String),
}
use std::collections::HashMap;
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemperatureStore {
    #[serde(flatten)]
    pub items: HashMap<HeaterNames, TempStoreData>,
}

impl TemperatureStore {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
    pub fn add_to_hashmap(&mut self, key: HeaterNames, value: TempStoreData)  {
        self.items.insert(key, value);
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoonErrorContent {
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
    /// Creates a new MoonMSG which can be sent to Moonraker via the websocket
    /// The method is the name of the method to be called and is required for all messages
    /// The params are the parameters to be passed to the method and are optional for some types of messages
    /// refer to the moonraker api docs for more information for now.
    /// The id is the id of the message and is optional for some types of messages. It allows you to match up responses to requests.
    /// Assuming you use unique ids for every message you send, a response with a match id is the response to the request with that id.
    pub fn new(method: MoonMethod, params: Option<MoonParam>, id: Option<u32>) -> MoonMSG {
        match (params, id) {
            (None, None) => MoonMSG::Method { jsonrpc: JsonRpcVersion::V2_0, method },
            (None, Some(id)) => MoonMSG::MethodID { jsonrpc: JsonRpcVersion::V2_0, method, id },
            (Some(params), None) => MoonMSG::MethodParam { jsonrpc: JsonRpcVersion::V2_0, method, params },
            (Some(params), Some(id)) => MoonMSG::MethodParamID { jsonrpc: JsonRpcVersion::V2_0, method, params, id },
        }
    }
    pub fn new_error(error: MoonErrorContent, id: u32) -> MoonMSG {
        MoonMSG::MoonError {
            jsonrpc: JsonRpcVersion::V2_0,
            error,
            id,
        }
    }
    pub fn new_result(result: MoonResultData, id: u32) -> MoonMSG {
        MoonMSG::MoonResult {
            jsonrpc: JsonRpcVersion::V2_0,
            result,
            id,
        }
    }
    pub fn gcode(gcode: String, id: u32) -> MoonMSG {
        MoonMSG::new(MoonMethod::GcodeScript, Some(MoonParam::GcodeScript { script: gcode.to_string() }), Some(id))
    }
    pub fn method(&self) ->  MoonMethod {
        match self {
            MoonMSG::MoonResult { result, .. } => {
                panic!("No method on type MoonMSG::MoonResult: {:?}", result);
            },
            MoonMSG::MoonError { error, .. } => {
                panic!("Error: {:?}", error);
            },
            MoonMSG::MethodParamID { method, .. } => method.clone(),
            MoonMSG::MethodParamVecID { method, .. } => method.clone(),
            MoonMSG::MethodParam { method, .. } => method.clone(),
            MoonMSG::MethodParamVec { method, .. } => method.clone(),
            MoonMSG::MethodID { method, .. } => method.clone(),
            MoonMSG::Method { method, .. } => method.clone(),
            MoonMSG::CouldNotParseParams { method, .. } => method.clone(),
            MoonMSG::CouldNotParseParamsID { method, .. } => method.clone(),
            MoonMSG::CouldNotParseMethod { method, .. } => {
                panic!("CouldNotParseMethod does not have a method of type MoonMethod, it is String: {:?}", method);
            },
            MoonMSG::CouldNotParseMethodID { method, .. } => {
                panic!("CouldNotParseMethodID does not have a method of type MoonMethod, it is String: {:?}", method);
            },
            MoonMSG::CouldNotParseMethodParams { method, .. } => {
                panic!("CouldNotParseMethodParams does not have a method of type MoonMethod, it is String: {:?}", method);
            },
            MoonMSG::CouldNotParseMethodParamsID { method, .. } => {
                panic!("CouldNotParseMethodParamsID does not have a method of type MoonMethod, it is String: {:?}", method);
            },
            MoonMSG::Empty => {
                MoonMethod::Empty
            },
        }
    }
    pub fn params(&self) -> Vec<MoonParam> {
        match self {
            MoonMSG::MoonResult { result, .. } => {
                panic!("No params on type MoonMSG::MoonResult \n{:?}", result);
            },
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
                panic!("CouldNotParseParams has params of type serde_json::Value not MoonParam. This function cannot be called on MoonMSG::CouldNotParseParams \n{:?}", params);
            },
            MoonMSG::CouldNotParseParamsID { params, .. } => {
                panic!("CouldNotParseParamsID has params of type serde_json::Value not MoonParam. This function cannot be called on MoonMSG::CouldNotParseParamsID \n{:?}", params);
            },
            MoonMSG::CouldNotParseMethod { params, .. } => vec! {params.clone()},
            MoonMSG::CouldNotParseMethodID { params, .. } => vec! {params.clone()},
            MoonMSG::CouldNotParseMethodParams { params, .. } => {
                panic!("CouldNotParseMethodParams has params of type serde_json::Value not MoonParam. This method cannot be called on MoonMSG::CouldNotParseMethodParams \n{:?}", params);
            },
            MoonMSG::CouldNotParseMethodParamsID { params, .. } => {
                panic!("CouldNotParseMethodParamsID has params of type serde_json::Value not MoonParam. This function cannot be called on MoonMSG::CouldNotParseMethodParamsID \n{:?}", params);
            },
            MoonMSG::Empty => {
                panic!("MoonMSG::Empty has no method");
            },
        }
    }
    pub fn set_id(&self, id: u32) -> MoonMSG {
        match self {
            MoonMSG::MoonResult { result, .. } => {
                MoonMSG::new_result(result.clone(), id)
                // MoonMSG::MoonResult { jsonrpc: JsonRpcVersion::V2_0, result: result.clone(), id }
            },
            MoonMSG::MoonError { error, .. } => {
                MoonMSG::new_error(error.clone(), id)
                // MoonMSG::MoonError { jsonrpc: JsonRpcVersion::V2_0, error.clone(), id }
            },
            MoonMSG::MethodParamID { method, params, .. } => {
                MoonMSG::new(method.clone(), Some(params.clone()), Some(id))
                // MoonMSG::MethodParamID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id: id }
            },
            MoonMSG::MethodParamVecID { method, params, .. } => {
                // MoonMSG::new(method.clone(), Some(params.clone()), Some(id))
                MoonMSG::MethodParamVecID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id }
            },
            MoonMSG::MethodParam { method, params, .. } => MoonMSG::MethodParamID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id },
            MoonMSG::MethodParamVec { method, params, .. } => MoonMSG::MethodParamVecID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone() , id },
            MoonMSG::MethodID { method, .. } => MoonMSG::MethodID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), id },
            MoonMSG::Method { method, .. } => MoonMSG::MethodID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), id},
            MoonMSG::CouldNotParseParams { method, params, .. } => {
                MoonMSG::CouldNotParseParamsID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id }
            },
            MoonMSG::CouldNotParseParamsID { method, params, .. } => {
                MoonMSG::CouldNotParseParamsID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id }
            },
            MoonMSG::CouldNotParseMethod { method, params, .. } => {
                MoonMSG::CouldNotParseMethodID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id }
            },
            MoonMSG::CouldNotParseMethodID { method, params, .. } => {
                MoonMSG::CouldNotParseMethodID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id }
            },
            MoonMSG::CouldNotParseMethodParams { method, params, .. } => {
                MoonMSG::CouldNotParseMethodParamsID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id }
            },
            MoonMSG::CouldNotParseMethodParamsID { method, params, .. } => {
                MoonMSG::CouldNotParseMethodParamsID { jsonrpc: JsonRpcVersion::V2_0, method: method.clone(), params: params.clone(), id }
            },
            MoonMSG::Empty => {
                MoonMSG::new(MoonMethod::Empty, None, Some(id))
            },
        }
    }
    pub fn id(&self) -> Option<u32> {
        match self {
            MoonMSG::MoonResult { id, .. } => Some(id.clone()),
            MoonMSG::MoonError { id, .. } => Some(id.clone()),
            MoonMSG::MethodParamID { id, .. } => Some(id.clone()),
            MoonMSG::MethodParamVecID { id, .. } => Some(id.clone()),
            MoonMSG::MethodID { id, .. } => Some(id.clone()),
            MoonMSG::MethodParam {..} => None,
            MoonMSG::MethodParamVec {..} => None,
            MoonMSG::Method {..} => None,
            MoonMSG::CouldNotParseParams {..} => None,
            MoonMSG::CouldNotParseParamsID {id, ..} => Some(id.clone()),
            MoonMSG::CouldNotParseMethod {..} => None,
            MoonMSG::CouldNotParseMethodID {id, ..} => Some(id.clone()),
            MoonMSG::CouldNotParseMethodParams {..} => None,
            MoonMSG::CouldNotParseMethodParamsID {id, ..} => Some(id.clone()),
            MoonMSG::Empty => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrinterState {
    Ready,
    Paused,
}

