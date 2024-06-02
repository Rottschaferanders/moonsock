use serde::{Serialize, Deserialize};

use crate::{
    jsonrpc_ws_client::JsonRpcMessage, JsonRpcVersion, MoonMethod, MoonParam
};


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MoonRequest {
    pub jsonrpc: JsonRpcVersion,
    pub method: MoonMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<MoonParam>,
    pub id: u32,
}

impl JsonRpcMessage for MoonRequest {
    fn id(&self) -> Option<u32> {
        Some(self.id)
    }
    fn set_id(&mut self, id: u32) {
        self.id = id;
        // match self.id() {
        //     Some(_) => self.id = id,
        //     None => panic!("Cannot set id for message with no id")
        // }
    }
}


impl MoonRequest {
    /// Creates a new MoonRequest which can be sent to Moonraker via the websocket
    /// The method is the name of the method to be called and is required for all messages
    /// The params are the parameters to be passed to the method and are optional for some types of messages
    /// refer to the moonraker api docs for more information for now.
    /// The id is the id of the message and is optional for some types of messages. It allows you to match up responses to requests.
    /// Assuming you use unique ids for every message you send, a response with a match id is the response to the request with that id.
    pub fn new(method: MoonMethod, params: Option<MoonParam>) -> Self {
        let id = rand::random();
        Self {
            jsonrpc: JsonRpcVersion::V2,
            method,
            params,
            id
        }
        // match (params, id) {
        //     (None, None) => Self::Method { jsonrpc: JsonRpcVersion::V2, method },
        //     (None, Some(id)) => Self::MethodID { jsonrpc: JsonRpcVersion::V2, method, id },
        //     (Some(params), None) => Self::MethodParam { jsonrpc: JsonRpcVersion::V2, method, params },
        //     (Some(params), Some(id)) => Self::MethodParamID { jsonrpc: JsonRpcVersion::V2, method, params, id },
        // }
    }
    pub fn gcode(gcode: String) -> Self {
        Self::new(MoonMethod::PrinterGcodeScript, Some(MoonParam::GcodeScript { script: gcode.to_string() }))
    }
    // pub fn method(&self) -> Option<MoonMethod> {
    //     match self {
    //         // Self::MethodParamID { method, .. } 
    //         // | Self::MethodParam { method, .. } 
    //         // | Self::MethodID { method, .. } 
    //         // | Self::Method { method, .. } => Some(method.clone()),
    //         // Self::Empty => None,
    //         // _ => None,
    //     }
    // }
    // pub fn params(&self) -> Option<&MoonParam> {
    //     match self {
    //         Self::MethodParamID { params, .. } 
    //         | Self::MethodParam { params, .. } => Some(params),
    //         Self::Empty => None,
    //         _ => None,
    //     }
    // }
    
    // pub fn set_id(&self, id: u32) -> Self {
    //     // match self {
    //     //     Self::MoonResult { result, .. } => {
    //     //         Self::new_result(result.clone(), id)
    //     //     },
    //     //     Self::MoonError { error, .. } => {
    //     //         Self::new_error(error.clone(), id)
    //     //     },
    //     //     Self::MethodParamID { method, params, .. } => {
    //     //         Self::new(method.clone(), Some(params.clone()), Some(id))
    //     //     },
    //     //     Self::MethodParam { method, params, .. } => Self::MethodParamID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), params: params.clone(), id },
    //     //     Self::MethodID { method, .. } => Self::MethodID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), id },
    //     //     Self::Method { method, .. } => Self::MethodID { jsonrpc: JsonRpcVersion::V2, method: method.clone(), id},
    //     //     Self::Empty => {
    //     //         Self::new(MoonMethod::Empty, None, Some(id))
    //     //     },
    //     // }
    // }
    // pub fn id(&self) -> Option<u32> {
    //     match self {
    //         Self::MoonResult { id, .. } => Some(id.clone()),
    //         Self::MoonError { id, .. } => Some(id.clone()),
    //         Self::MethodParamID { id, .. } => Some(id.clone()),
    //         Self::MethodID { id, .. } => Some(id.clone()),
    //         Self::MethodParam {..} => None,
    //         Self::Method {..} => None,
    //         Self::Empty => None,
    //     }
    // }
}