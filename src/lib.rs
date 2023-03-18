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
// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(bound = "T: Serialize + DeserializeOwned")]
// pub struct Response<T> {
//     pub jsonrpc: JsonRpcVersion,
//     #[serde(flatten)]
//     #[serde(with = "ResultDef")]
//     pub result: Result<T, ResponseError>,
//     pub id: u32,
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2_0
}

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct ResponseError {
//     pub code: i32,
//     pub message: String,
//     pub data: Option<serde_json::Value>,
// }

// #[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(remote = "Result")]
// pub enum ResultDef<T, E> {
//     #[serde(rename = "result")]
//     Ok(T),
//     #[serde(rename = "error")]
//     Err(E),
// }
/// ---------------------- Request Serializing ------------------------

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

// extruder: {
//     "temperatures": [21.05, 21.12, 21.1, 21.1, 21.1],
//     "targets": [0, 0, 0, 0, 0],
//     "powers": [0, 0, 0, 0, 0]
// },
// "temperature_fan my_fan": {
//     "temperatures": [21.05, 21.12, 21.1, 21.1, 21.1],
//     "targets": [0, 0, 0, 0, 0],
//     "speeds": [0, 0, 0, 0, 0],
// },
// "temperature_sensor my_sensor": {
//     "temperatures": [21.05, 21.12, 21.1, 21.1, 21.1]
// }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MoonResultData {
    #[serde(rename = "ok")]
    Ok,
    ServerTemperatureStore {
        #[serde(flatten)]
        items: TemperatureStoreItems,
    },
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TemperatureStoreItems {
    Extruder {
        temperatures: Vec<f32>,
        targets: Vec<f32>,
        powers: Vec<f32>,
    },
    TemperatureFan {
        temperatures: Vec<f32>,
        targets: Vec<f32>,
        speeds: Vec<f32>,
    },
    TemperatureSensor {
        temperatures: Vec<f32>,
    },
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
    // pub fn new_with_params(method: MoonMethod, params: Option(MoonParam), id: u32) -> MoonMSG:: {
    //     match params {
    //         MoonParam::None => MoonMSG::MethodID { jsonrpc: JsonRpcVersion::V2_0, method, id },
    //         _ => MoonMSG::MethodParamID { jsonrpc: JsonRpcVersion::V2_0, method, params, id },
    //     }
    // }
    // #[allow(dead_code)]
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

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct Request {
//     jsonrpc: JsonRpcVersion,
//     method: MoonMethod,
//     params: MoonParam,
//     id: u32,
// }

// impl Request {
//     #[allow(dead_code)]
//     pub fn new(method: MoonMethod, params: MoonParam, id: u32) -> Request {
//         // match params {
//         //     MoonParam::None => Request { jsonrpc: JsonRpcVersion::V2_0, method: method },
//         //     _ => Request::MethodParam { jsonrpc: JsonRpcVersion::V2_0, method: method, params: params },
//         // }
//         Request { 
//             jsonrpc: JsonRpcVersion::V2_0, 
//             method: method, 
//             params: params,
//             id: id,
//         }
//     }
// }
