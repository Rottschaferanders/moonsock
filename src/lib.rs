use std::{error::Error, fmt};
use serde::{Serialize, Deserialize};

pub mod response;
pub use response::*;

mod request;
pub use request::*;
// mod moon_response;
// pub use moon_responses::*;

pub mod connection;
pub mod moon_method;
// pub mod moon_param;
mod moon_param;
mod notification;
pub use notification::*;
pub mod utils;

// Exports:
pub use connection::MoonConnection;
pub use moon_method::MoonMethod;
pub use moon_param::*;
// use messages::{PrinterState, PrinterInfoResponse};
// use response::{
//     PrinterInfoResponse,
//     MoonResultData,
//     ServerInfo, ServerConfig,
//     // TemperatureStore, 
//     // GcodeType,
// };

pub mod fast_ws_stuff;
// pub mod fast_ws_connection;
mod fast_ws_connection;
pub use fast_ws_connection::*;

pub mod jsonrpc_ws_client;
// pub mod moonraker_client;
mod moonraker_client;
pub use moonraker_client::MoonrakerClient;

/// ---------------------- Request Serializing ------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum JsonRpcVersion {
    #[serde(rename = "2.0")]
    V2
}

impl Default for JsonRpcVersion {
    fn default() -> Self {
        JsonRpcVersion::V2
    }
}

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum MoonMSG {
//     MoonResult {
//         jsonrpc: JsonRpcVersion,
//         result: MoonResultData,
//         id: u32,
//     },
//     MoonError {
//         jsonrpc: JsonRpcVersion,
//         error: MoonErrorContent,
//         id: u32,
//     },
//     MethodParamID {
//         jsonrpc: JsonRpcVersion,
//         method: MoonMethod,
//         params: MoonParam,
//         id: u32,
//     },
//     // MethodParamVecID {
//     //     jsonrpc: JsonRpcVersion,
//     //     method: MoonMethod,
//     //     params: Vec<MoonParam>,
//     //     id: u32,
//     // },
//     // MethodParamVec {
//     //     jsonrpc: JsonRpcVersion,
//     //     method: MoonMethod,
//     //     params: Vec<MoonParam>,
//     // },
//     MethodParam {
//         jsonrpc: JsonRpcVersion,
//         method: MoonMethod,
//         params: MoonParam,
//     },
//     MethodID {
//         jsonrpc: JsonRpcVersion,
//         method: MoonMethod,
//         id: u32,
//     },
//     Method {
//         jsonrpc: JsonRpcVersion,
//         method: MoonMethod,
//     },
//     // CouldNotParseParamsID {
//     //     jsonrpc: JsonRpcVersion,
//     //     method: MoonMethod,
//     //     params: serde_json::Value,
//     //     id: u32,
//     // },
//     // CouldNotParseParams {
//     //     jsonrpc: JsonRpcVersion,
//     //     method: MoonMethod,
//     //     params: serde_json::Value,
//     // },
//     // CouldNotParseMethodID {
//     //     jsonrpc: JsonRpcVersion,
//     //     method: String,
//     //     params: MoonParam,
//     //     id: u32,
//     // },
//     // CouldNotParseMethod {
//     //     jsonrpc: JsonRpcVersion,
//     //     method: String,
//     //     params: MoonParam,
//     // },
//     // CouldNotParseMethodParamsID {
//     //     jsonrpc: JsonRpcVersion,
//     //     method: String,
//     //     params: serde_json::Value,
//     //     id: u32,
//     // },
//     // CouldNotParseMethodParams {
//     //     jsonrpc: JsonRpcVersion,
//     //     method: String,
//     //     params: serde_json::Value,
//     // },
//     Empty,
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// #[serde(untagged)]
// pub enum MoonResponse {
//     MoonResult {
//         jsonrpc: JsonRpcVersion,
//         result: MoonResultData,
//         id: u32,
//     },
//     MoonError {
//         jsonrpc: JsonRpcVersion,
//         error: MoonErrorContent,
//         #[serde(skip_serializing_if = "Option::is_none")]
//         id: Option<u32>,
//     },
//     Notification {
//         jsonrpc: JsonRpcVersion,
//         method: NotificationMethod,
//         #[serde(skip_serializing_if = "Option::is_none")]
//         params: Option<NotificationParam>,
//     },
// }

// impl Default for MoonResponse {
//     fn default() -> Self {
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             method: NotificationMethod::Other(""),
//             params: None,
//         }
//     }
// }

// impl Default for MoonResponse {
//     fn default() -> Self {
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             result: MoonResultData::None,
//             id: 0,
//         }
//     }
// }

// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct MoonErrorContent {
//     pub code: u32,
//     pub message: String,
// }

// impl fmt::Display for MoonErrorContent {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}: {}", self.code, self.message)
//     }
// }

// impl Error for MoonErrorContent {}

// impl Default for MoonResponse {
//     fn default() -> Self {
//         MoonResponse::MoonResult {
//             jsonrpc: JsonRpcVersion::V2,
//             result: MoonResultData::None,
//             id: 0,
//         }
//     }
// }

// impl MoonResponse {
//     pub fn method(&self) -> Option<&NotificationMethod> {
//         match self {
//             Self::MoonResult { .. } 
//             | Self::MoonError { .. } => {
//                 None
//             },
//             Self::Notification { method, .. } => {
//                 Some(method)
//             },
//         }
//     }
//     // pub fn params(&self) -> Option<Vec<NotificationParam>> {
//     pub fn params(&self) -> Option<NotificationParam> {
//         match self {
//             Self::MoonResult { .. } 
//             | Self::MoonError { .. } => {
//                 None
//             },
//             Self::Notification { params, .. } => {
//                 params.clone()
//             },
//         }
//     }
//     pub fn set_id(&mut self, new_id: u32) {
//         match self {
//             Self::MoonError { jsonrpc, error, .. } => {
//                 let new = Self::MoonError { 
//                     jsonrpc: jsonrpc.clone(), 
//                     error: error.clone(), 
//                     id: Some(new_id) 
//                 };
//                 *self = new;
//             },
//             Self::MoonResult { jsonrpc, result, .. } => {
//                 let new = Self::MoonResult { 
//                     jsonrpc: jsonrpc.clone(), 
//                     result: result.clone(), 
//                     id: new_id 
//                 };
//                 *self = new;
//             },
//             Self::Notification { .. } => {},
//         }
//     }
//     pub fn default_server_info_result(server_info: ServerInfo) -> Self {
//         MoonResponse::MoonResult {
//             jsonrpc: JsonRpcVersion::V2,
//             result: MoonResultData::ServerInfo(server_info),
//             id: 0,
//         }
//     }
//     pub fn default_server_config_result(config: ServerConfig) -> Self {
//         MoonResponse::MoonResult {
//             jsonrpc: JsonRpcVersion::V2,
//             result: MoonResultData::ServerConfig(config),
//             id: 0,
//         }
//     }
//     // pub fn new_error(error: MoonErrorContent, id: u32) -> Self {
//     //     Self::MoonError {
//     //         jsonrpc: JsonRpcVersion::V2,
//     //         error,
//     //         id,
//     //     }
//     // }
//     // pub fn new_result(result: moon_result::MoonResultData, id: u32) -> Self {
//     //     Self::MoonResult {
//     //         jsonrpc: JsonRpcVersion::V2,
//     //         result,
//     //         id,
//     //     }
//     // }
// }


// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// pub struct MoonRequest {
//     pub jsonrpc: JsonRpcVersion,
//     pub method: MoonMethod,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub params: Option<MoonParam>,
//     pub id: u32,
// }


// impl MoonRequest {
//     /// Creates a new MoonRequest which can be sent to Moonraker via the websocket
//     /// The method is the name of the method to be called and is required for all messages
//     /// The params are the parameters to be passed to the method and are optional for some types of messages
//     /// refer to the moonraker api docs for more information for now.
//     /// The id is the id of the message and is optional for some types of messages. It allows you to match up responses to requests.
//     /// Assuming you use unique ids for every message you send, a response with a match id is the response to the request with that id.
//     pub fn new(method: MoonMethod, params: Option<MoonParam>) -> Self {
//         let id = rand::random();
//         Self {
//             jsonrpc: JsonRpcVersion::V2,
//             method,
//             params,
//             id
//         }
//         // match (params, id) {
//         //     (None, None) => Self::Method { jsonrpc: JsonRpcVersion::V2, method },
//         //     (None, Some(id)) => Self::MethodID { jsonrpc: JsonRpcVersion::V2, method, id },
//         //     (Some(params), None) => Self::MethodParam { jsonrpc: JsonRpcVersion::V2, method, params },
//         //     (Some(params), Some(id)) => Self::MethodParamID { jsonrpc: JsonRpcVersion::V2, method, params, id },
//         // }
//     }
//     pub fn gcode(gcode: String) -> Self {
//         Self::new(MoonMethod::PrinterGcodeScript, Some(MoonParam::GcodeScript { script: gcode.to_string() }))
//     }
//     // pub fn method(&self) -> Option<MoonMethod> {
//     //     match self {
//     //         // Self::MethodParamID { method, .. } 
//     //         // | Self::MethodParam { method, .. } 
//     //         // | Self::MethodID { method, .. } 
//     //         // | Self::Method { method, .. } => Some(method.clone()),
//     //         // Self::Empty => None,
//     //         // _ => None,
//     //     }
//     // }
//     // pub fn params(&self) -> Option<&MoonParam> {
//     //     match self {
//     //         Self::MethodParamID { params, .. } 
//     //         | Self::MethodParam { params, .. } => Some(params),
//     //         Self::Empty => None,
//     //         _ => None,
//     //     }
//     // }
    
//     // pub fn set_id(&self, id: u32) -> Self {
//     //     // match self {
//     //     //     Self::MoonResult { result, .. } => {
//     //     //         Self::new_result(result.clone(), id)
//     //     //     },
//     //     //     Self::MoonError { error, .. } => {
//     //     //         Self::new_error(error.clone(), id)
//     //     //     },
//     //     //     Self::MethodParamID { method, params, .. } => {
//     //     //         Self::new(method.clone(), Some(params.clone()), Some(id))
//     //     //     },
//     //     //     Self::MethodParam { method, params, .. } => Self::MethodParamID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id },
//     //     //     Self::MethodID { method, .. } => Self::MethodID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), id },
//     //     //     Self::Method { method, .. } => Self::MethodID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), id},
//     //     //     Self::Empty => {
//     //     //         Self::new(MoonMethod::Empty, None, Some(id))
//     //     //     },
//     //     // }
//     // }
//     // pub fn id(&self) -> Option<u32> {
//     //     match self {
//     //         Self::MoonResult { id, .. } => Some(id.clone()),
//     //         Self::MoonError { id, .. } => Some(id.clone()),
//     //         Self::MethodParamID { id, .. } => Some(id.clone()),
//     //         Self::MethodID { id, .. } => Some(id.clone()),
//     //         Self::MethodParam {..} => None,
//     //         Self::Method {..} => None,
//     //         Self::Empty => None,
//     //     }
//     // }
// }

// impl MoonMSG {
//     /// Creates a new MoonMSG which can be sent to Moonraker via the websocket
//     /// The method is the name of the method to be called and is required for all messages
//     /// The params are the parameters to be passed to the method and are optional for some types of messages
//     /// refer to the moonraker api docs for more information for now.
//     /// The id is the id of the message and is optional for some types of messages. It allows you to match up responses to requests.
//     /// Assuming you use unique ids for every message you send, a response with a match id is the response to the request with that id.
//     pub fn new(method: MoonMethod, params: Option<MoonParam>, id: Option<u32>) -> MoonMSG {
//         match (params, id) {
//             (None, None) => MoonMSG::Method { jsonrpc: JsonRpcVersion::V2, method },
//             (None, Some(id)) => MoonMSG::MethodID { jsonrpc: JsonRpcVersion::V2, method, id },
//             (Some(params), None) => MoonMSG::MethodParam { jsonrpc: JsonRpcVersion::V2, method, params },
//             (Some(params), Some(id)) => MoonMSG::MethodParamID { jsonrpc: JsonRpcVersion::V2, method, params, id },
//         }
//     }
//     pub fn new_error(error: MoonErrorContent, id: u32) -> MoonMSG {
//         MoonMSG::MoonError {
//             jsonrpc: JsonRpcVersion::V2,
//             error,
//             id,
//         }
//     }
//     pub fn new_result(result: moon_result::MoonResultData, id: u32) -> MoonMSG {
//         MoonMSG::MoonResult {
//             jsonrpc: JsonRpcVersion::V2,
//             result,
//             id,
//         }
//     }
//     pub fn gcode(gcode: String, id: u32) -> MoonMSG {
//         MoonMSG::new(MoonMethod::PrinterGcodeScript, Some(MoonParam::GcodeScript { script: gcode.to_string() }), Some(id))
//     }
//     pub fn method(&self) -> Option<MoonMethod> {
//         match self {
//             MoonMSG::MethodParamID { method, .. } 
//             // | MoonMSG::MethodParamVecID { method, .. } 
//             | MoonMSG::MethodParam { method, .. } 
//             // | MoonMSG::MethodParamVec { method, .. } 
//             | MoonMSG::MethodID { method, .. } 
//             | MoonMSG::Method { method, .. } => Some(method.clone()),
//             // | MoonMSG::CouldNotParseParams { method, .. } 
//             // | MoonMSG::CouldNotParseParamsID { method, .. } => Some(method.clone()),
//             MoonMSG::Empty => None,
//             _ => None,
//         }
//     }
//     // pub fn method(&self) -> Result<MoonMethod, Box<dyn std::error::Error>> {
//     //     match self {
//     //         MoonMSG::MethodParamID { method, .. } 
//     //         | MoonMSG::MethodParamVecID { method, .. } 
//     //         | MoonMSG::MethodParam { method, .. } 
//     //         | MoonMSG::MethodParamVec { method, .. } 
//     //         | MoonMSG::MethodID { method, .. } 
//     //         | MoonMSG::Method { method, .. } 
//     //         | MoonMSG::CouldNotParseParams { method, .. } 
//     //         | MoonMSG::CouldNotParseParamsID { method, .. } => Ok(method.clone()),
//     //         MoonMSG::Empty => Ok(MoonMethod::Empty),
//     //         other => Err(format!("Variant {:?} does not have a method of type MoonMethod", other).into()),
//     //     }
//     // }
//     // pub fn method(&self) ->  MoonMethod {
//     //     match self {
//     //         MoonMSG::MoonResult { result, .. } => {
//     //             panic!("No method on type MoonMSG::MoonResult: {:?}", result);
//     //         },
//     //         MoonMSG::MoonError { error, .. } => {
//     //             panic!("Error: {:?}", error);
//     //         },
//     //         MoonMSG::MethodParamID { method, .. } => method.clone(),
//     //         MoonMSG::MethodParamVecID { method, .. } => method.clone(),
//     //         MoonMSG::MethodParam { method, .. } => method.clone(),
//     //         MoonMSG::MethodParamVec { method, .. } => method.clone(),
//     //         MoonMSG::MethodID { method, .. } => method.clone(),
//     //         MoonMSG::Method { method, .. } => method.clone(),
//     //         MoonMSG::CouldNotParseParams { method, .. } => method.clone(),
//     //         MoonMSG::CouldNotParseParamsID { method, .. } => method.clone(),
//     //         MoonMSG::CouldNotParseMethod { method, .. } => {
//     //             panic!("CouldNotParseMethod does not have a method of type MoonMethod, it is String: {:?}", method);
//     //         },
//     //         MoonMSG::CouldNotParseMethodID { method, .. } => {
//     //             panic!("CouldNotParseMethodID does not have a method of type MoonMethod, it is String: {:?}", method);
//     //         },
//     //         MoonMSG::CouldNotParseMethodParams { method, .. } => {
//     //             panic!("CouldNotParseMethodParams does not have a method of type MoonMethod, it is String: {:?}", method);
//     //         },
//     //         MoonMSG::CouldNotParseMethodParamsID { method, .. } => {
//     //             panic!("CouldNotParseMethodParamsID does not have a method of type MoonMethod, it is String: {:?}", method);
//     //         },
//     //         MoonMSG::Empty => {
//     //             MoonMethod::Empty
//     //         },
//     //     }
//     // }
//     // pub fn method(&self) -> MoonMethod {
//     //     match self {
//     //         MoonMSG::MoonResult { result, .. } => {
//     //             panic!("No method on type MoonMSG::MoonResult: {:?}", result);
//     //         },
//     //         MoonMSG::Empty => MoonMethod::Empty,
//     //         // Extract 'method' directly in these cases
//     //         MoonMSG::MethodParamID { method, .. }
//     //         | MoonMSG::MethodParamVecID { method, .. }
//     //         | MoonMSG::MethodParam { method, .. }
//     //         | MoonMSG::MethodParamVec { method, .. }
//     //         | MoonMSG::MethodID { method, .. }
//     //         | MoonMSG::Method { method, .. }
//     //         | MoonMSG::CouldNotParseParams { method, .. }
//     //         | MoonMSG::CouldNotParseParamsID { method, .. } => method.clone(),
//     //         // Handle error cases
//     //         MoonMSG::CouldNotParseMethod { .. }
//     //         | MoonMSG::CouldNotParseMethodID { .. }
//     //         | MoonMSG::CouldNotParseMethodParams { .. }
//     //         | MoonMSG::CouldNotParseMethodParamsID { .. } => {
//     //             panic!("Error extracting method (check message type)") 
//     //         }
//     //         MoonMSG::MoonError { error, .. } => {
//     //             panic!("Error: {:?}", error);
//     //         },
//     //     }
//     // }
//     // pub fn params(&self) -> Vec<MoonParam> {
//     //     match self {
//     //         MoonMSG::MoonResult { result, .. } => {
//     //             panic!("No params on type MoonMSG::MoonResult \n{:?}", result);
//     //         },
//     //         MoonMSG::MoonError {..} => {
//     //             panic!("MoonError has no params");
//     //         },
//     //         MoonMSG::MethodParamID { params, .. } => vec! {params.clone()},
//     //         MoonMSG::MethodParamVecID { params, .. } => params.clone(),
//     //         MoonMSG::MethodParam { params, .. } => vec! { params.clone() },
//     //         MoonMSG::MethodParamVec { params, .. } => params.clone(),
//     //         MoonMSG::MethodID {..} => {
//     //             panic!("MethodID has no params");
//     //         },
//     //         MoonMSG::Method {..} => {
//     //             panic!("Method has no params");
//     //         },
//     //         MoonMSG::CouldNotParseParams { params, .. } => {
//     //             panic!("CouldNotParseParams has params of type serde_json::Value not MoonParam. This function cannot be called on MoonMSG::CouldNotParseParams \n{:?}", params);
//     //         },
//     //         MoonMSG::CouldNotParseParamsID { params, .. } => {
//     //             panic!("CouldNotParseParamsID has params of type serde_json::Value not MoonParam. This function cannot be called on MoonMSG::CouldNotParseParamsID \n{:?}", params);
//     //         },
//     //         MoonMSG::CouldNotParseMethod { params, .. } => vec! {params.clone()},
//     //         MoonMSG::CouldNotParseMethodID { params, .. } => vec! {params.clone()},
//     //         MoonMSG::CouldNotParseMethodParams { params, .. } => {
//     //             panic!("CouldNotParseMethodParams has params of type serde_json::Value not MoonParam. This method cannot be called on MoonMSG::CouldNotParseMethodParams \n{:?}", params);
//     //         },
//     //         MoonMSG::CouldNotParseMethodParamsID { params, .. } => {
//     //             panic!("CouldNotParseMethodParamsID has params of type serde_json::Value not MoonParam. This function cannot be called on MoonMSG::CouldNotParseMethodParamsID \n{:?}", params);
//     //         },
//     //         MoonMSG::Empty => {
//     //             panic!("MoonMSG::Empty has no method");
//     //         },
//     //     }
//     // }
//     // pub fn params(&self) -> Vec<MoonParam> {
//     //     match self {
//     //         MoonMSG::MethodParamID { params, .. } 
//     //         | MoonMSG::MethodParam { params, .. } 
//     //         | MoonMSG::CouldNotParseMethod { params, .. } 
//     //         | MoonMSG::CouldNotParseMethodID { params, .. } => vec![params.clone()],
//     //         MoonMSG::MethodParamVecID { params, .. } 
//     //         | MoonMSG::MethodParamVec { params, .. } => params.clone(),
//     //         _ => panic!("This variant does not have params or params are not of type MoonParam"),
//     //     }
//     // }
//     // pub fn params(&self) -> Result<Vec<MoonParam>, Box<dyn std::error::Error>> {
//     //     match self {
//     //         MoonMSG::MethodParamID { params, .. } 
//     //         | MoonMSG::MethodParam { params, .. } => Ok(vec![params.clone()]),
//     //         // | MoonMSG::CouldNotParseMethod { params, .. } 
//     //         // | MoonMSG::CouldNotParseMethodID { params, .. } => Ok(vec![params.clone()]),
//     //         MoonMSG::MethodParamVecID { params, .. } 
//     //         | MoonMSG::MethodParamVec { params, .. } => Ok(params.clone()),
//     //         other => Err(format!("Variant {:?} does not have params or params are not of type MoonParam", other).into()),
//     //     }
//     // }
//     // pub fn params(&self) -> Option<Vec<MoonParam>> {
//     //     match self {
//     //         MoonMSG::MethodParamID { params, .. } 
//     //         | MoonMSG::MethodParam { params, .. } => Some(vec![params.clone()]),
//     //         MoonMSG::MethodParamVecID { params, .. } 
//     //         | MoonMSG::MethodParamVec { params, .. } => Some(params.clone()),
//     //         MoonMSG::CouldNotParseParams { params, .. } 
//     //         | MoonMSG::CouldNotParseParamsID { params, .. } => Some(vec![MoonParam::JsonValue(params.clone())]),
//     //         MoonMSG::Empty => None,
//     //         _ => None,
//     //     }
//     // }
//     // pub fn params(&self) -> Option<Vec<MoonParam>> {
//     //     match self {
//     //         MoonMSG::MethodParamID { params, .. } 
//     //         // | MoonMSG::MethodParamVecID { params, .. } 
//     //         | MoonMSG::MethodParam { params, .. } => Some(vec![params.clone()]),
//     //         // | MoonMSG::MethodParamVec { params, .. } 
//     //         // | MoonMSG::CouldNotParseParams { params, .. } 
//     //         // | MoonMSG::CouldNotParseParamsID { params, .. } => Some(vec![params.clone()]),
//     //         MoonMSG::Empty => None,
//     //         _ => None,
//     //     }
//     // }
//     pub fn params(&self) -> Option<&MoonParam> {
//         match self {
//             MoonMSG::MethodParamID { params, .. } 
//             // | MoonMSG::MethodParamVecID { params, .. } 
//             | MoonMSG::MethodParam { params, .. } => Some(params),
//             // | MoonMSG::MethodParamVec { params, .. } 
//             // | MoonMSG::CouldNotParseParams { params, .. } 
//             // | MoonMSG::CouldNotParseParamsID { params, .. } => Some(vec![params.clone()]),
//             MoonMSG::Empty => None,
//             _ => None,
//         }
//     }
    
//     pub fn set_id(&self, id: u32) -> MoonMSG {
//         match self {
//             MoonMSG::MoonResult { result, .. } => {
//                 MoonMSG::new_result(result.clone(), id)
//                 // MoonMSG::MoonResult { jsonrpc: JsonRpcVersion::V2, result: result.clone(), id }
//             },
//             MoonMSG::MoonError { error, .. } => {
//                 MoonMSG::new_error(error.clone(), id)
//                 // MoonMSG::MoonError { jsonrpc: JsonRpcVersion::V2, error.clone(), id }
//             },
//             MoonMSG::MethodParamID { method, params, .. } => {
//                 MoonMSG::new(method.clone(), Some(params.clone()), Some(id))
//                 // MoonMSG::MethodParamID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id: id }
//             },
//             // MoonMSG::MethodParamVecID { method, params, .. } => {
//             //     // MoonMSG::new(method.clone(), Some(params.clone()), Some(id))
//             //     MoonMSG::MethodParamVecID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id }
//             // },
//             MoonMSG::MethodParam { method, params, .. } => MoonMSG::MethodParamID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id },
//             // MoonMSG::MethodParamVec { method, params, .. } => MoonMSG::MethodParamVecID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone() , id },
//             MoonMSG::MethodID { method, .. } => MoonMSG::MethodID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), id },
//             MoonMSG::Method { method, .. } => MoonMSG::MethodID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), id},
//             // MoonMSG::CouldNotParseParams { method, params, .. } => {
//             //     MoonMSG::CouldNotParseParamsID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id }
//             // },
//             // MoonMSG::CouldNotParseParamsID { method, params, .. } => {
//             //     MoonMSG::CouldNotParseParamsID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id }
//             // },
//             // MoonMSG::CouldNotParseMethod { method, params, .. } => {
//             //     MoonMSG::CouldNotParseMethodID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id }
//             // },
//             // MoonMSG::CouldNotParseMethodID { method, params, .. } => {
//             //     MoonMSG::CouldNotParseMethodID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id }
//             // },
//             // MoonMSG::CouldNotParseMethodParams { method, params, .. } => {
//             //     MoonMSG::CouldNotParseMethodParamsID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id }
//             // },
//             // MoonMSG::CouldNotParseMethodParamsID { method, params, .. } => {
//             //     MoonMSG::CouldNotParseMethodParamsID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id }
//             // },
//             MoonMSG::Empty => {
//                 MoonMSG::new(MoonMethod::Empty, None, Some(id))
//             },
//         }
//     }
//     pub fn id(&self) -> Option<u32> {
//         match self {
//             MoonMSG::MoonResult { id, .. } => Some(id.clone()),
//             MoonMSG::MoonError { id, .. } => Some(id.clone()),
//             MoonMSG::MethodParamID { id, .. } => Some(id.clone()),
//             // MoonMSG::MethodParamVecID { id, .. } => Some(id.clone()),
//             MoonMSG::MethodID { id, .. } => Some(id.clone()),
//             MoonMSG::MethodParam {..} => None,
//             // MoonMSG::MethodParamVec {..} => None,
//             MoonMSG::Method {..} => None,
//             // MoonMSG::CouldNotParseParams {..} => None,
//             // MoonMSG::CouldNotParseParamsID {id, ..} => Some(id.clone()),
//             // MoonMSG::CouldNotParseMethod {..} => None,
//             // MoonMSG::CouldNotParseMethodID {id, ..} => Some(id.clone()),
//             // MoonMSG::CouldNotParseMethodParams {..} => None,
//             // MoonMSG::CouldNotParseMethodParamsID {id, ..} => Some(id.clone()),
//             MoonMSG::Empty => None,
//         }
//     }
// }


