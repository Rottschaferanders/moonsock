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
    }
    pub fn gcode(gcode: String) -> Self {
        Self::new(MoonMethod::PrinterGcodeScript, Some(MoonParam::GcodeScript { script: gcode.to_string() }))
    }
}