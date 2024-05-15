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
    None
}

impl Default for MoonResultData {
    fn default() -> Self {
        MoonResultData::None
    }
}

