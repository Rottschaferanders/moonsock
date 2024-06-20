use crate::response::{
    ConnectionIdResponse, EndstopStatus, GcodeHelp, GcodeStore, 
    PrinterInfoResponse, PrinterObjectsListResponse, PrinterObjectsQueryResponse, 
    RollOverResponse, 
    ServerConfig, ServerInfo, SystemInfo, 
    TemperatureStore, WebsocketIdResponse,
    MachineProcStats,
};

use serde::{Deserialize, Serialize};

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
    PrinterInfoResponse(PrinterInfoResponse),
    PrinterObjectsListResponse(PrinterObjectsListResponse),
    PrinterObjectsQueryResponse(PrinterObjectsQueryResponse),
    ServerInfo(ServerInfo),
    ServerConfig(ServerConfig),
    GcodeStore(GcodeStore),
    RollOverResponse(RollOverResponse),
    ConnectionIdResponse(ConnectionIdResponse),
    WebsocketIdResponse(WebsocketIdResponse),
    EndstopStatus(EndstopStatus),
    GcodeHelp(GcodeHelp),
    // SystemInfo(SystemInfo),
    SystemInfo {
        system_info: SystemInfo,
    },
    MachineProcStats(MachineProcStats),
    AuthenticationResponse(AuthenticationResponse),
    None
}

impl Default for MoonResultData {
    fn default() -> Self {
        MoonResultData::None
    }
}

impl std::fmt::Display for MoonResultData {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthenticationResponse {
    pub username: String,
    pub token: String,
    pub refresh_token: String,
    pub action: String,
    pub source: String,
}
